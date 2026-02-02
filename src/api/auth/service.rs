use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use std::sync::OnceLock;
use uuid::Uuid;
use validator::Validate;
use tracing::{info, warn};

use crate::{
    api::auth::{dto::*, jwt, repo::AuthRepo},
    api::user::repo as user_repo,
    error::{AppError, AppResult},
    state::AppState,
};

pub struct AuthService;

impl AuthService {
    // =========================================================================
    // Helper Functions
    // =========================================================================

    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }

    /// 타이밍 공격 방지를 위한 더미 해시 수행
    fn dummy_password_hash() -> AppResult<PasswordHash<'static>> {
        static DUMMY_HASH: OnceLock<String> = OnceLock::new();
        let hash_str = DUMMY_HASH.get_or_init(|| {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(b"dummy_password", &salt)
                .expect("argon2 dummy hash should succeed")
                .to_string()
        });
        PasswordHash::new(hash_str)
            .map_err(|_| AppError::Internal("Failed to parse dummy password hash".into()))
    }

    fn map_device_for_db(dev: Option<&str>) -> &'static str {
        match dev.unwrap_or("").to_ascii_lowercase().as_str() {
            "mobile" => "mobile",
            "tablet" => "tablet",
            "desktop" | "web" | "browser" => "desktop",
            _ => "other",
        }
    }

    // 리프레시 토큰 생성 (Token, Hash)
    fn generate_refresh_token_and_hash(session_id: &str) -> (String, String) {
        let random_uuid = Uuid::new_v4().to_string();
        // 포맷: session_id:random_uuid
        let payload = format!("{session_id}:{random_uuid}");
        
        let refresh_token = URL_SAFE_NO_PAD.encode(payload.as_bytes());
        let refresh_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(payload.as_bytes()));
        
        (refresh_token, refresh_hash)
    }

    // 주어진 리프레시 토큰 해싱
    fn hash_refresh_token(token: &str) -> AppResult<String> {
        let decoded_bytes = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        Ok(URL_SAFE_NO_PAD.encode(Sha256::digest(decoded_bytes)))
    }

    // 리프레시 토큰 파싱 (SessionID 추출)
    fn parse_refresh_token(token: &str) -> AppResult<(String, String)> {
        let decoded = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        // 입력받은 토큰의 해시 계산 (DB 비교용)
        let incoming_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(&decoded));
        
        let decoded_str = String::from_utf8(decoded)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        
        let mut parts = decoded_str.splitn(2, ':');
        let session_id = parts.next().filter(|s| !s.is_empty())
            .ok_or_else(|| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        let random_part = parts.next().filter(|s| !s.is_empty())
            .ok_or_else(|| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        // UUID 형식 검증
        if Uuid::parse_str(session_id).is_err() || Uuid::parse_str(random_part).is_err() {
             return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()));
        }

        Ok((session_id.to_string(), incoming_hash))
    }

    // =========================================================================
    // Main Business Logic
    // =========================================================================

    /// 로그인 처리
    #[allow(clippy::too_many_arguments)]
    pub async fn login(
        st: &AppState,
        req: LoginReq,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<(LoginRes, Cookie<'static>, i64)> {
        // [Step 1] Input Validation
        let email = req.email.trim().to_lowercase();
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!("AUTH_400_INVALID_INPUT: {}", e)));
        }

        // [Step 2] Rate Limiting
        let rl_key = format!("rl:login:{}:{}", email, login_ip);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
        }

        // [Step 3] User Verification (Timing Attack Protected)
        let user_info = AuthRepo::find_user_by_email(&st.db, &email).await?;

        let parsed_hash = match &user_info {
            Some(user) => PasswordHash::new(&user.user_password)
                .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?,
            None => Self::dummy_password_hash()?,
        };

        let password_ok = Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok();

        if user_info.is_none() || !password_ok {
            return Err(AppError::Unauthorized("AUTH_401_BAD_CREDENTIALS".into()));
        }

        let user_info = user_info.expect("checked above");

        if !user_info.user_state {
            return Err(AppError::Forbidden);
        }

        // [Step 4] Token & Session Generation
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
        // 역할별 세션 TTL 적용 (HYMN: 1일, Admin/Manager: 7일, Learner: 30일)
        let refresh_ttl_secs = st.cfg.refresh_ttl_days_for_role(&user_info.user_auth) * 24 * 3600;

        // JWT Access Token (role 포함)
        let access_token_res = jwt::create_token(
            user_info.user_id,
            &session_id,
            user_info.user_auth,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;

        // [Step 5] DB Transaction (Login Record)
        let mut tx = st.db.begin().await?;
        let mapped_device = Self::map_device_for_db(req.device.as_deref());
        
        AuthRepo::insert_login_record_tx(
            &mut tx,
            user_info.user_id,
            &session_id,
            &refresh_hash,
            &login_ip,
            Some(mapped_device),
            req.browser.as_deref(),
            req.os.as_deref(),
            user_agent.as_deref(),
        ).await?;

        AuthRepo::insert_login_log_tx(
            &mut tx,
            user_info.user_id,
            "login",
            true,
            &session_id,
            &refresh_hash,
            &login_ip,
            Some(mapped_device),
            req.browser.as_deref(),
            req.os.as_deref(),
            user_agent.as_deref(),
        ).await?;
        
        tx.commit().await?;

        // [Step 6] Redis Caching (After DB Commit)
        // 1. Session ID -> User ID
        let _: () = redis_conn.set_ex(
            format!("ak:session:{}", session_id),
            user_info.user_id,
            st.cfg.jwt_access_ttl_min as u64 * 60,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 2. Refresh Hash -> Session ID
        let _: () = redis_conn.set_ex(
            format!("ak:refresh:{}", refresh_hash),
            &session_id,
            refresh_ttl_secs as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 3. User Sessions Set (for bulk logout)
        let _: () = redis_conn.sadd(
            format!("ak:user_sessions:{}", user_info.user_id),
            &session_id,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        let mut refresh_cookie =
        Cookie::new(st.cfg.refresh_cookie_name.clone(), refresh_token_value);
        refresh_cookie.set_path("/");
        refresh_cookie.set_http_only(true);
        refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
        refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
            "Strict" => SameSite::Strict,
            "Lax" => SameSite::Lax,
            "None" => SameSite::None,
            _ => SameSite::Lax, 
        });
        refresh_cookie
            .set_expires(OffsetDateTime::now_utc() + time::Duration::seconds(refresh_ttl_secs));
        
        // 도메인이 있을 때만 설정 (빈 문자열 방지)
        if let Some(domain) = &st.cfg.refresh_cookie_domain {
            refresh_cookie.set_domain(domain.clone());
        }

        Ok((
            LoginRes {
                user_id: user_info.user_id,
                access: access_token_res,
                session_id,
            },
            refresh_cookie.into_owned(),
            refresh_ttl_secs,
        ))
    }

    /// 토큰 갱신 (Rotation 적용)
    pub async fn refresh(
        st: &AppState,
        old_refresh_token: &str,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<(LoginRes, String, i64)> {
        let (session_id, incoming_hash) = Self::parse_refresh_token(old_refresh_token)?;

        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        // [Step 1] DB Lock & Lookup
        let mut tx = st.db.begin().await?;
        let login_record = match AuthRepo::find_login_by_session_id_for_update_tx(&mut tx, &session_id).await? {
            Some(record) => record,
            None => return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into())),
        };

        // [Step 2] Reuse Detection (Security Critical)
        if login_record.refresh_hash != incoming_hash {
            warn!("Refresh token reuse detected! Session: {}", session_id);
            
            // 2-1. Mark session compromised
            AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "compromised").await?;
            AuthRepo::insert_login_log_tx(
                &mut tx,
                login_record.user_id,
                "reuse_detected",
                false,
                &session_id,
                &login_record.refresh_hash,
                login_record.login_ip.as_deref().unwrap_or(&login_ip),
                Some(&login_record.login_device),
                login_record.login_browser.as_deref(),
                login_record.login_os.as_deref(),
                user_agent.as_deref(),
            ).await?;
            tx.commit().await?;

            // 2-2. Invalidate Redis keys immediately
            let _ = redis_conn.del::<_, ()>(format!("ak:refresh:{}", login_record.refresh_hash)).await;
            let _ = redis_conn.del::<_, ()>(format!("ak:session:{}", session_id)).await;
            let _ = redis_conn.srem::<_, _, ()>(format!("ak:user_sessions:{}", login_record.user_id), &session_id).await;

            return Err(AppError::Conflict("AUTH_409_REUSE_DETECTED".into()));
        }

        // [Step 3] Validate State
        if login_record.state != "active" {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()));
        }

        // [Step 4] Rotate Token
        let (new_refresh_token_value, new_refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);

        AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash).await?;
        AuthRepo::insert_login_log_tx(
            &mut tx,
            login_record.user_id,
            "rotate",
            true,
            &session_id,
            &new_refresh_hash,
            login_record.login_ip.as_deref().unwrap_or(&login_ip),
            Some(&login_record.login_device),
            login_record.login_browser.as_deref(),
            login_record.login_os.as_deref(),
            user_agent.as_deref(),
        ).await?;
        
        tx.commit().await?;

        // [Step 5] Fetch user role for JWT and TTL calculation
        let user = user_repo::find_user(&st.db, login_record.user_id)
            .await?
            .ok_or(AppError::Unauthorized("User not found".into()))?;

        // [Step 6] Redis Sync
        // Delete old hash
        let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash))
            .await.map_err(|e| AppError::Internal(e.to_string()))?;

        // Set new hash (역할별 TTL 적용)
        let refresh_ttl_secs = st.cfg.refresh_ttl_days_for_role(&user.user_auth) * 24 * 3600;
        let _: () = redis_conn.set_ex(
            format!("ak:refresh:{}", new_refresh_hash),
            &session_id,
            refresh_ttl_secs as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // Issue new Access Token (role 포함)
        let access_token_res = jwt::create_token(
            login_record.user_id,
            &session_id,
            user.user_auth,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;

        Ok((
            LoginRes {
                user_id: login_record.user_id,
                access: access_token_res,
                session_id,
            },
            new_refresh_token_value,
            refresh_ttl_secs,
        ))
    }

    /// 아이디 찾기
    pub async fn find_id(st: &AppState, req: FindIdReq, client_ip: String) -> AppResult<FindIdRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!("AUTH_400_INVALID_INPUT: {}", e)));
        }

        // Rate Limiting
        let rl_key = format!("rl:find_id:{}", client_ip);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
        }

        let user = AuthRepo::find_user_by_name_and_email(&st.db, &req.name, &req.email).await?;
        
        if let Some(found) = user {
            // 실제로는 여기서 이메일 발송 로직이 수행되어야 함
            let _ = user_repo::insert_user_log_after(&st.db, Some(found.user_id), found.user_id, "find_id", true).await;
            info!("Find ID email simulated for user_id={}", found.user_id);
        } else {
            // Security: Don't log the actual email to prevent enumeration via logs
            info!("Find ID request processed");
        }

        Ok(FindIdRes {
            message: "If the account exists, the ID has been sent to your email.".to_string(),
        })
    }

    /// 비밀번호 재설정
    pub async fn reset_password(
        st: &AppState,
        req: ResetPwReq,
        client_ip: String,
    ) -> AppResult<ResetPwRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!("AUTH_400_INVALID_INPUT: {}", e)));
        }
        if !Self::validate_password_policy(&req.new_password) {
            return Err(AppError::Unprocessable("password policy violation".into()));
        }

        // Rate Limiting
        let rl_key = format!("rl:reset_pw:{}", client_ip);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
        }

        // Token Decode
        let claims = jwt::decode_token(&req.reset_token, &st.cfg.jwt_secret)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_RESET_TOKEN".into()))?;
        let user_id = claims.sub;

        // Hash New Password
        let salt = SaltString::generate(&mut OsRng);
        let params = argon2::Params::new(19_456, 2, 1, None).unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let new_password_hash = argon2
            .hash_password(req.new_password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
            .to_string();

        // DB Update (Password + Revoke Sessions)
        let mut tx = st.db.begin().await?;
        AuthRepo::update_user_password_tx(&mut tx, user_id, &new_password_hash).await?;
        user_repo::insert_user_log_after_tx(&mut tx, Some(user_id), user_id, "reset_pw", true).await?;
        AuthRepo::update_login_state_by_user_tx(&mut tx, user_id, "revoked").await?;
        tx.commit().await?;

        // Redis Session Cleanup
        let session_key = format!("ak:user_sessions:{}", user_id);
        let session_ids: Vec<String> = redis_conn.smembers(&session_key).await.unwrap_or_default();
        
        for sid in session_ids.iter() {
            // Find hash to delete refresh token mapping
            if let Some(login_record) = AuthRepo::find_login_by_session_id(&st.db, sid).await? {
                let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await.unwrap_or(());
            }
            let _: () = redis_conn.del(format!("ak:session:{}", sid)).await.unwrap_or(());
            let _: () = redis_conn.srem(&session_key, sid).await.unwrap_or(());
        }
        let _: () = redis_conn.del(&session_key).await.unwrap_or(());

        Ok(ResetPwRes {
            message: "Password has been reset. All active sessions are invalidated.".to_string(),
        })
    }

    /// 로그아웃 (단일 세션)
    pub async fn logout(
        st: &AppState,
        user_id: i64,
        session_id: &str,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 1) DB Update
        let mut tx = st.db.begin().await?;
        let login_record = AuthRepo::find_login_by_session_id_tx(&mut tx, session_id).await?;
        
        if let Some(record) = &login_record {
            AuthRepo::update_login_state_by_session_tx(&mut tx, session_id, "logged_out").await?;
            AuthRepo::insert_logout_log_tx(
                &mut tx,
                user_id,
                session_id,
                &record.refresh_hash,
                &login_ip,
                user_agent.as_deref(),
            ).await?;
        }
        tx.commit().await?;

        // 2) Redis Cleanup
        if let Some(record) = login_record {
            let _: () = redis_conn.del(format!("ak:refresh:{}", record.refresh_hash)).await.unwrap_or(());
        }
        let _: () = redis_conn.del(format!("ak:session:{}", session_id)).await.unwrap_or(());
        let _: () = redis_conn.srem(format!("ak:user_sessions:{}", user_id), session_id).await.unwrap_or(());

        Ok(())
    }

    /// 로그아웃 (모든 기기 or 현재 기기)
    pub async fn logout_all(
        st: &AppState,
        refresh_token: Option<&str>,
        req: LogoutAllReq,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<LogoutRes> {
        // Find User from Refresh Token
        let mut user_id: Option<i64> = None;
        let mut current_session_id: Option<String> = None;

        if let Some(token) = refresh_token {
            let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
            
            // 해시 계산은 에러가 나면 무효한 토큰으로 간주
            if let Ok(refresh_hash) = Self::hash_refresh_token(token) {
                 if let Ok(sid) = redis_conn.get::<_, String>(format!("ak:refresh:{}", refresh_hash)).await {
                    if let Ok(uid) = redis_conn.get::<_, i64>(format!("ak:session:{}", sid)).await {
                        user_id = Some(uid);
                        current_session_id = Some(sid);
                    }
                }
            }
        }

        let Some(uid) = user_id else {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()));
        };

        let mut tx = st.db.begin().await?;
        let mut sessions_to_invalidate = Vec::new();

        if req.everywhere {
            // 모든 세션 조회
            let session_ids = AuthRepo::find_user_session_ids_tx(&mut tx, uid).await?;
            sessions_to_invalidate.extend(session_ids);
            
            // DB 상태 일괄 업데이트
            AuthRepo::update_login_state_by_user_tx(&mut tx, uid, "logged_out").await?;
        } else if let Some(sid) = current_session_id {
            // 현재 세션만
            sessions_to_invalidate.push(sid.clone());
            AuthRepo::update_login_state_by_session_tx(&mut tx, &sid, "logged_out").await?;
        }

        // 로그 기록 (Loop)
        for sid in &sessions_to_invalidate {
             if let Some(record) = AuthRepo::find_login_by_session_id_tx(&mut tx, sid).await? {
                AuthRepo::insert_logout_log_tx(
                    &mut tx,
                    uid,
                    sid,
                    &record.refresh_hash,
                    &login_ip,
                    user_agent.as_deref(),
                ).await?;
             }
        }
        
        tx.commit().await?;

        // Redis Cleanup
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        
        for sid in sessions_to_invalidate {
            if let Some(record) = AuthRepo::find_login_by_session_id(&st.db, &sid).await? {
                let _: () = redis_conn.del(format!("ak:refresh:{}", record.refresh_hash)).await.unwrap_or(());
            }
            let _: () = redis_conn.del(format!("ak:session:{}", sid)).await.unwrap_or(());
            let _: () = redis_conn.srem(format!("ak:user_sessions:{}", uid), &sid).await.unwrap_or(());
        }

        if req.everywhere {
             let _: () = redis_conn.del(format!("ak:user_sessions:{}", uid)).await.unwrap_or(());
        }

        Ok(LogoutRes { ok: true })
    }
}