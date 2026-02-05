use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use std::sync::OnceLock;
use uuid::Uuid;
use validator::Validate;
use tracing::{info, warn};

use crate::external::email::EmailTemplate;

use crate::{
    api::auth::{dto::*, jwt, repo::AuthRepo},
    api::user::repo as user_repo,
    error::{AppError, AppResult},
    external::google::{GoogleOAuthClient, GoogleUserInfo},
    state::AppState,
    types::UserAuth,
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
        parsed_ua: crate::api::auth::handler::ParsedUa,
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

        // 소셜 전용 계정 체크 (비밀번호가 NULL인 경우)
        if let Some(ref user) = user_info {
            if user.user_password.is_none() {
                // OAuth 연결 정보 조회
                let providers = AuthRepo::find_oauth_providers_by_user_id(&st.db, user.user_id).await?;
                let provider_list = if providers.is_empty() {
                    "social".to_string()
                } else {
                    providers.join(",")
                };
                return Err(AppError::Unauthorized(format!(
                    "AUTH_401_SOCIAL_ONLY_ACCOUNT:{}",
                    provider_list
                )));
            }
        }

        let parsed_hash = match &user_info {
            Some(user) => PasswordHash::new(user.user_password.as_ref().unwrap())
                .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?,
            None => Self::dummy_password_hash()?,
        };

        let password_ok = Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok();

        if user_info.is_none() || !password_ok {
            // 로그인 실패 로그 (사용자가 존재하는 경우만 기록)
            if let Some(ref user) = user_info {
                let fail_session = Uuid::new_v4().to_string();
                let mut tx = st.db.begin().await?;
                let _ = AuthRepo::insert_login_log_tx(
                    &mut tx, user.user_id, "login", false,
                    &fail_session, "", &login_ip,
                    Some(parsed_ua.device.as_str()), parsed_ua.browser.as_deref(),
                    parsed_ua.os.as_deref(), user_agent.as_deref(),
                    None, None, None,
                    None, None, Some("invalid_credentials"),
                    None,
                ).await;
                let _ = tx.commit().await;
            }
            return Err(AppError::Unauthorized("AUTH_401_BAD_CREDENTIALS".into()));
        }

        let user_info = user_info.expect("checked above");

        if !user_info.user_state {
            // 비활성 계정 실패 로그
            let fail_session = Uuid::new_v4().to_string();
            let mut tx = st.db.begin().await?;
            let _ = AuthRepo::insert_login_log_tx(
                &mut tx, user_info.user_id, "login", false,
                &fail_session, "", &login_ip,
                Some(parsed_ua.device.as_str()), parsed_ua.browser.as_deref(),
                parsed_ua.os.as_deref(), user_agent.as_deref(),
                None, None, None,
                None, None, Some("account_disabled"),
                None,
            ).await;
            let _ = tx.commit().await;
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }

        // [Step 4] Token & Session Generation
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
        // 역할별 세션 TTL 적용 (HYMN: 1일, Admin/Manager: 7일, Learner: 30일)
        let refresh_ttl_secs = st.cfg.refresh_ttl_days_for_role(&user_info.user_auth) * 24 * 3600;

        // JWT Access Token (role 포함)
        let (access_token_res, jti) = jwt::create_token(
            user_info.user_id,
            &session_id,
            user_info.user_auth,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;

        // Access token SHA-256 hash (audit log용)
        let access_hash: String = Sha256::digest(access_token_res.access_token.as_bytes())
            .iter().map(|b| format!("{:02x}", b)).collect();

        // [Step 5] IP Geolocation (best-effort, non-blocking)
        let geo = st.ipgeo.lookup(&login_ip).await.unwrap_or_default();

        // [Step 6] DB Transaction (Login Record)
        let mut tx = st.db.begin().await?;

        AuthRepo::insert_login_record_tx(
            &mut tx,
            user_info.user_id,
            &session_id,
            &refresh_hash,
            &login_ip,
            Some(parsed_ua.device.as_str()),
            parsed_ua.browser.as_deref(),
            parsed_ua.os.as_deref(),
            user_agent.as_deref(),
            refresh_ttl_secs,
            geo.country_code.as_deref(),
            geo.asn,
            geo.org.as_deref(),
        ).await?;

        AuthRepo::insert_login_log_tx(
            &mut tx,
            user_info.user_id,
            "login",
            true,
            &session_id,
            &refresh_hash,
            &login_ip,
            Some(parsed_ua.device.as_str()),
            parsed_ua.browser.as_deref(),
            parsed_ua.os.as_deref(),
            user_agent.as_deref(),
            geo.country_code.as_deref(),
            geo.asn,
            geo.org.as_deref(),
            Some(&access_hash),
            Some(&jti),
            Some("none"),
            Some(refresh_ttl_secs),
        ).await?;

        tx.commit().await?;

        // [Step 7] Redis Caching (After DB Commit)
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
            AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "compromised", Some("security_concern")).await?;
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
                None, None, None,
                None, None,
                Some("token_reuse"),
                None,
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

        // [Step 4] Fetch user role for TTL calculation (before rotate)
        let user = user_repo::find_user(&st.db, login_record.user_id)
            .await?
            .ok_or(AppError::Unauthorized("User not found".into()))?;
        let refresh_ttl_secs = st.cfg.refresh_ttl_days_for_role(&user.user_auth) * 24 * 3600;

        // [Step 5] Rotate Token & Issue new Access Token
        let (new_refresh_token_value, new_refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
        let (access_token_res, jti) = jwt::create_token(
            login_record.user_id,
            &session_id,
            user.user_auth,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;
        let access_hash: String = Sha256::digest(access_token_res.access_token.as_bytes())
            .iter().map(|b| format!("{:02x}", b)).collect();

        AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash, refresh_ttl_secs).await?;
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
            None, None, None,
            Some(&access_hash),
            Some(&jti),
            Some("none"),
            Some(refresh_ttl_secs),
        ).await?;

        tx.commit().await?;

        // [Step 6] Redis Sync
        // Delete old hash
        let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash))
            .await.map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis_conn.set_ex(
            format!("ak:refresh:{}", new_refresh_hash),
            &session_id,
            refresh_ttl_secs as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

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
        AuthRepo::update_login_state_by_user_tx(&mut tx, user_id, "revoked", Some("password_changed")).await?;
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
            AuthRepo::update_login_state_by_session_tx(&mut tx, session_id, "logged_out", Some("none")).await?;
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
            AuthRepo::update_login_state_by_user_tx(&mut tx, uid, "logged_out", Some("none")).await?;
        } else if let Some(sid) = current_session_id {
            // 현재 세션만
            sessions_to_invalidate.push(sid.clone());
            AuthRepo::update_login_state_by_session_tx(&mut tx, &sid, "logged_out", Some("none")).await?;
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

    // =========================================================================
    // Password Reset (이메일 인증 기반)
    // =========================================================================

    /// 6자리 숫자 인증코드 생성
    fn generate_verification_code() -> String {
        let mut rng = rand::thread_rng();
        let code: u32 = rng.gen_range(100000..1000000);
        format!("{:06}", code)
    }

    /// 비밀번호 재설정 요청 (인증코드 발송)
    pub async fn request_password_reset(
        st: &AppState,
        email: &str,
        client_ip: String,
    ) -> AppResult<RequestResetRes> {
        let email = email.trim().to_lowercase();

        // [Step 1] Rate Limiting (이메일당 5회/시간)
        let rl_key = format!("rl:request_reset:{}", email);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, 3600).await?; // 1시간 윈도우
        }
        if attempts > 5 {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_RESET_REQUESTS".into()));
        }

        // [Step 2] 사용자 존재 확인 (타이밍 공격 방지를 위해 항상 성공 응답)
        let user = AuthRepo::find_user_by_email(&st.db, &email).await?;

        // 사용자가 없거나 OAuth 전용 계정이면 이메일 발송 없이 성공 응답
        if user.is_none() {
            info!("Password reset requested for non-existent email");
            return Ok(RequestResetRes {
                message: "If the email exists, a verification code has been sent.".to_string(),
            });
        }

        let user_info = user.unwrap();

        // OAuth 전용 계정 (비밀번호가 NULL)이면 이메일 발송 없이 성공 응답
        if user_info.user_password.is_none() {
            info!("Password reset requested for OAuth-only account: {}", user_info.user_id);
            return Ok(RequestResetRes {
                message: "If the email exists, a verification code has been sent.".to_string(),
            });
        }

        // [Step 3] 이메일 클라이언트 확인
        let email_client = st.email.as_ref()
            .ok_or_else(|| AppError::Internal("Email service not configured".into()))?;

        // [Step 4] 인증코드 생성 및 Redis 저장
        let code = Self::generate_verification_code();
        let code_key = format!("ak:reset_code:{}", email);
        let ttl_sec = st.cfg.verification_code_ttl_sec;

        let _: () = redis_conn.set_ex(
            &code_key,
            &code,
            ttl_sec as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // [Step 5] 이메일 발송
        let expires_in_min = (ttl_sec / 60) as i32;
        email_client.send_templated(
            &email,
            EmailTemplate::PasswordResetCode { code: code.clone(), expires_in_min },
        ).await?;

        info!(
            user_id = user_info.user_id,
            ip = %client_ip,
            "Password reset code sent"
        );

        Ok(RequestResetRes {
            message: "If the email exists, a verification code has been sent.".to_string(),
        })
    }

    /// 인증코드 검증 및 reset_token 발급
    pub async fn verify_reset_code(
        st: &AppState,
        email: &str,
        code: &str,
        client_ip: String,
    ) -> AppResult<VerifyResetRes> {
        let email = email.trim().to_lowercase();
        let code = code.trim();

        // [Step 1] Rate Limiting (이메일당 10회/시간 - brute force 방지)
        let rl_key = format!("rl:verify_reset:{}", email);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, 3600).await?; // 1시간 윈도우
        }
        if attempts > 10 {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_VERIFY_ATTEMPTS".into()));
        }

        // [Step 2] Redis에서 저장된 코드 조회
        let code_key = format!("ak:reset_code:{}", email);
        let stored_code: Option<String> = redis_conn.get(&code_key).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let Some(expected_code) = stored_code else {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_OR_EXPIRED_CODE".into()));
        };

        // [Step 3] 코드 검증
        if code != expected_code {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_OR_EXPIRED_CODE".into()));
        }

        // [Step 4] 코드 삭제 (일회용)
        let _: () = redis_conn.del(&code_key).await.unwrap_or(());

        // [Step 5] 사용자 조회 (user_id 필요)
        let user = AuthRepo::find_user_by_email(&st.db, &email).await?
            .ok_or_else(|| AppError::Unauthorized("AUTH_401_USER_NOT_FOUND".into()))?;

        // [Step 6] reset_token 생성 (Redis에 저장, JWT 대신 단순 토큰 사용)
        let reset_token = format!("ak_reset_{}", Uuid::new_v4());
        let token_key = format!("ak:reset_token:{}", reset_token);
        let token_ttl = st.cfg.reset_token_ttl_sec;

        let _: () = redis_conn.set_ex(
            &token_key,
            user.user_id,
            token_ttl as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        info!(
            user_id = user.user_id,
            ip = %client_ip,
            "Password reset code verified, token issued"
        );

        Ok(VerifyResetRes {
            reset_token,
            expires_in: token_ttl,
        })
    }

    /// 비밀번호 재설정 (새 비밀번호 설정) - 기존 reset_password와 통합
    pub async fn reset_password_with_token(
        st: &AppState,
        reset_token: &str,
        new_password: &str,
        client_ip: String,
    ) -> AppResult<ResetPwRes> {
        // [Step 1] 비밀번호 정책 검증
        if !Self::validate_password_policy(new_password) {
            return Err(AppError::Unprocessable("AUTH_422_PASSWORD_POLICY_VIOLATION".into()));
        }

        // [Step 2] Rate Limiting
        let rl_key = format!("rl:reset_pw:{}", client_ip);
        let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
        }

        // [Step 3] reset_token 검증 (Redis 기반 or JWT)
        let user_id = if reset_token.starts_with("ak_reset_") {
            // Redis 기반 토큰 (새 flow)
            let token_key = format!("ak:reset_token:{}", reset_token);
            let stored_user_id: Option<i64> = redis_conn.get(&token_key).await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let uid = stored_user_id
                .ok_or_else(|| AppError::Unauthorized("AUTH_401_INVALID_OR_EXPIRED_TOKEN".into()))?;

            // 토큰 삭제 (일회용)
            let _: () = redis_conn.del(&token_key).await.unwrap_or(());
            uid
        } else {
            // JWT 기반 토큰 (기존 flow - 하위 호환)
            let claims = jwt::decode_token(reset_token, &st.cfg.jwt_secret)
                .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_RESET_TOKEN".into()))?;
            claims.sub
        };

        // [Step 4] 새 비밀번호 해싱
        let salt = SaltString::generate(&mut OsRng);
        let params = argon2::Params::new(19_456, 2, 1, None).unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let new_password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
            .to_string();

        // [Step 5] DB 업데이트 (비밀번호 + 세션 무효화)
        let mut tx = st.db.begin().await?;
        AuthRepo::update_user_password_tx(&mut tx, user_id, &new_password_hash).await?;
        user_repo::insert_user_log_after_tx(&mut tx, Some(user_id), user_id, "reset_pw", true).await?;
        AuthRepo::update_login_state_by_user_tx(&mut tx, user_id, "revoked", Some("password_changed")).await?;
        tx.commit().await?;

        // [Step 6] Redis 세션 정리
        let session_key = format!("ak:user_sessions:{}", user_id);
        let session_ids: Vec<String> = redis_conn.smembers(&session_key).await.unwrap_or_default();

        for sid in session_ids.iter() {
            if let Some(login_record) = AuthRepo::find_login_by_session_id(&st.db, sid).await? {
                let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await.unwrap_or(());
            }
            let _: () = redis_conn.del(format!("ak:session:{}", sid)).await.unwrap_or(());
            let _: () = redis_conn.srem(&session_key, sid).await.unwrap_or(());
        }
        let _: () = redis_conn.del(&session_key).await.unwrap_or(());

        info!(user_id = user_id, ip = %client_ip, "Password reset successful");

        Ok(ResetPwRes {
            message: "Password has been reset. All active sessions are invalidated.".to_string(),
        })
    }

    // =========================================================================
    // Google OAuth
    // =========================================================================

    /// Google OAuth 인증 URL 생성
    pub async fn google_auth_start(st: &AppState) -> AppResult<String> {
        // Google OAuth 설정 확인
        let client_id = st.cfg.google_client_id.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_CLIENT_ID not configured".into()))?;
        let client_secret = st.cfg.google_client_secret.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_CLIENT_SECRET not configured".into()))?;
        let redirect_uri = st.cfg.google_redirect_uri.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_REDIRECT_URI not configured".into()))?;

        // State와 Nonce 생성 (CSRF/Replay 방지)
        let state = Uuid::new_v4().to_string();
        let nonce = Uuid::new_v4().to_string();

        // Redis에 state -> nonce 저장
        let mut redis_conn = st.redis.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let state_key = format!("ak:oauth_state:{}", state);
        let _: () = redis_conn.set_ex(
            &state_key,
            &nonce,
            st.cfg.oauth_state_ttl_sec as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // Auth URL 생성
        let client = GoogleOAuthClient::new(
            client_id.clone(),
            client_secret.clone(),
            redirect_uri.clone(),
        );

        let auth_url = client.build_auth_url(&state, &nonce);

        Ok(auth_url)
    }

    /// Google OAuth 콜백 처리
    /// 반환: (LoginRes, Cookie, refresh_ttl, is_new_user)
    pub async fn google_auth_callback(
        st: &AppState,
        code: &str,
        state: &str,
        login_ip: String,
        user_agent: Option<String>,
        parsed_ua: crate::api::auth::handler::ParsedUa,
    ) -> AppResult<(LoginRes, Cookie<'static>, i64, bool)> {
        // Google OAuth 설정 확인
        let client_id = st.cfg.google_client_id.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_CLIENT_ID not configured".into()))?;
        let client_secret = st.cfg.google_client_secret.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_CLIENT_SECRET not configured".into()))?;
        let redirect_uri = st.cfg.google_redirect_uri.as_ref()
            .ok_or_else(|| AppError::Internal("GOOGLE_REDIRECT_URI not configured".into()))?;

        // [Step 1] State 검증 (CSRF 방지)
        let mut redis_conn = st.redis.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let state_key = format!("ak:oauth_state:{}", state);
        let stored_nonce: Option<String> = redis_conn.get(&state_key).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let Some(nonce) = stored_nonce else {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_OAUTH_STATE".into()));
        };

        // State 사용 후 즉시 삭제 (일회용)
        let _: () = redis_conn.del(&state_key).await.unwrap_or(());

        // [Step 2] Authorization Code → Token 교환
        let client = GoogleOAuthClient::new(
            client_id.clone(),
            client_secret.clone(),
            redirect_uri.clone(),
        );

        let token_response = client.exchange_code(code).await?;

        // [Step 3] ID Token 검증 및 사용자 정보 추출
        let claims = client.decode_id_token(&token_response.id_token)?;

        // Nonce 검증
        if claims.nonce.as_deref() != Some(&nonce) {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_NONCE".into()));
        }

        // Audience 검증
        if claims.aud != *client_id {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_AUDIENCE".into()));
        }

        let user_info = client.extract_user_info(&claims);

        // [Step 4] 사용자 조회 또는 생성
        let (user_id, user_auth, is_new_user) = Self::find_or_create_oauth_user(st, &user_info, "google").await?;

        // [Step 5] 세션 생성
        let (login_res, cookie, refresh_ttl) = Self::create_oauth_session(st, user_id, user_auth, "google", login_ip, user_agent, parsed_ua).await?;

        Ok((login_res, cookie, refresh_ttl, is_new_user))
    }

    /// OAuth 사용자 조회 또는 생성
    /// 반환: (user_id, user_auth, is_new_user)
    async fn find_or_create_oauth_user(
        st: &AppState,
        user_info: &GoogleUserInfo,
        provider: &str,
    ) -> AppResult<(i64, UserAuth, bool)> {
        // 1. OAuth 연결 정보로 기존 계정 조회
        if let Some(oauth) = AuthRepo::find_oauth_by_provider_subject(
            &st.db, provider, &user_info.sub
        ).await? {
            // 이미 연결된 계정 존재 - 마지막 로그인 시간 업데이트
            AuthRepo::update_oauth_last_login(&st.db, oauth.user_oauth_id).await?;

            let user = user_repo::find_user(&st.db, oauth.user_id).await?
                .ok_or_else(|| AppError::Internal("OAuth linked user not found".into()))?;

            info!("OAuth login: existing user {} via {}", oauth.user_id, provider);
            return Ok((user.id, user.user_auth, false));
        }

        // 2. 이메일로 기존 계정 조회 (자동 연결)
        if let Some(existing_user) = AuthRepo::find_user_by_email(&st.db, &user_info.email).await? {
            // 기존 계정에 OAuth 연결
            let mut tx = st.db.begin().await?;

            AuthRepo::insert_oauth_link_tx(
                &mut tx,
                existing_user.user_id,
                provider,
                &user_info.sub,
                Some(&user_info.email),
                user_info.name.as_deref(),
                user_info.picture.as_deref(),
            ).await?;

            tx.commit().await?;

            info!("OAuth account linked to existing user: {} ({})", existing_user.user_id, user_info.email);
            return Ok((existing_user.user_id, existing_user.user_auth, false));
        }

        // 3. 신규 사용자 생성 (자동 회원가입)
        let (user_id, user_auth) = Self::create_oauth_user(st, user_info, provider).await?;
        Ok((user_id, user_auth, true))
    }

    /// OAuth 신규 사용자 생성
    async fn create_oauth_user(
        st: &AppState,
        user_info: &GoogleUserInfo,
        provider: &str,
    ) -> AppResult<(i64, UserAuth)> {
        let mut tx = st.db.begin().await?;

        // 닉네임 생성 (이름 또는 이메일 앞부분)
        let nickname = user_info.name.clone()
            .unwrap_or_else(|| user_info.email.split('@').next().unwrap_or("User").to_string());

        // 사용자 생성 (비밀번호 없이, user_check_email = true)
        let user_id = sqlx::query_scalar::<_, i64>(r#"
            INSERT INTO users (
                user_email, user_password, user_name,
                user_nickname, user_language, user_country,
                user_birthday, user_gender,
                user_terms_service, user_terms_personal,
                user_check_email
            )
            VALUES (
                $1, NULL, $2,
                $3, 'ko'::user_language_enum, 'Unknown',
                CURRENT_DATE, 'none'::user_gender_enum,
                true, true,
                true
            )
            RETURNING user_id
        "#)
        .bind(&user_info.email)
        .bind(&nickname)
        .bind(&nickname)
        .fetch_one(&mut *tx)
        .await?;

        // OAuth 연결 정보 생성
        AuthRepo::insert_oauth_link_tx(
            &mut tx,
            user_id,
            provider,
            &user_info.sub,
            Some(&user_info.email),
            user_info.name.as_deref(),
            user_info.picture.as_deref(),
        ).await?;

        // 회원가입 로그
        user_repo::insert_user_log_after_tx(&mut tx, None, user_id, "signup", true).await?;

        tx.commit().await?;

        info!("New OAuth user created: {} ({}) via {}", user_id, user_info.email, provider);
        Ok((user_id, UserAuth::Learner))
    }

    /// OAuth 세션 생성
    async fn create_oauth_session(
        st: &AppState,
        user_id: i64,
        user_auth: UserAuth,
        login_method: &str,
        login_ip: String,
        user_agent: Option<String>,
        parsed_ua: crate::api::auth::handler::ParsedUa,
    ) -> AppResult<(LoginRes, Cookie<'static>, i64)> {
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
        let refresh_ttl_secs = st.cfg.refresh_ttl_days_for_role(&user_auth) * 24 * 3600;

        // Access Token 생성
        let (access_token_res, jti) = jwt::create_token(
            user_id,
            &session_id,
            user_auth,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;
        let access_hash: String = Sha256::digest(access_token_res.access_token.as_bytes())
            .iter().map(|b| format!("{:02x}", b)).collect();

        // IP Geolocation (best-effort, non-blocking)
        let geo = st.ipgeo.lookup(&login_ip).await.unwrap_or_default();

        // DB 기록
        let mut tx = st.db.begin().await?;

        AuthRepo::insert_login_record_oauth_tx(
            &mut tx,
            user_id,
            &session_id,
            &refresh_hash,
            &login_ip,
            login_method,
            Some(parsed_ua.device.as_str()),
            parsed_ua.browser.as_deref(),
            parsed_ua.os.as_deref(),
            user_agent.as_deref(),
            refresh_ttl_secs,
            geo.country_code.as_deref(),
            geo.asn,
            geo.org.as_deref(),
        ).await?;

        AuthRepo::insert_login_log_oauth_tx(
            &mut tx,
            user_id,
            "login",
            true,
            &session_id,
            &refresh_hash,
            &login_ip,
            login_method,
            Some(parsed_ua.device.as_str()),
            parsed_ua.browser.as_deref(),
            parsed_ua.os.as_deref(),
            user_agent.as_deref(),
            geo.country_code.as_deref(),
            geo.asn,
            geo.org.as_deref(),
            Some(&access_hash),
            Some(&jti),
            Some("none"),
            Some(refresh_ttl_secs),
        ).await?;

        tx.commit().await?;

        // Redis 캐싱
        let mut redis_conn = st.redis.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 1. Session ID -> User ID
        let _: () = redis_conn.set_ex(
            format!("ak:session:{}", session_id),
            user_id,
            st.cfg.jwt_access_ttl_min as u64 * 60,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 2. Refresh Hash -> Session ID
        let _: () = redis_conn.set_ex(
            format!("ak:refresh:{}", refresh_hash),
            &session_id,
            refresh_ttl_secs as u64,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 3. User Sessions Set
        let _: () = redis_conn.sadd(
            format!("ak:user_sessions:{}", user_id),
            &session_id,
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // Cookie 생성
        let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), refresh_token_value);
        refresh_cookie.set_path("/");
        refresh_cookie.set_http_only(true);
        refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
        refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
            "Strict" => SameSite::Strict,
            "Lax" => SameSite::Lax,
            "None" => SameSite::None,
            _ => SameSite::Lax,
        });
        refresh_cookie.set_expires(OffsetDateTime::now_utc() + time::Duration::seconds(refresh_ttl_secs));

        if let Some(domain) = &st.cfg.refresh_cookie_domain {
            refresh_cookie.set_domain(domain.clone());
        }

        Ok((
            LoginRes {
                user_id,
                access: access_token_res,
                session_id,
            },
            refresh_cookie.into_owned(),
            refresh_ttl_secs,
        ))
    }
}