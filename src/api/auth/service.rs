// FILE: src/api/auth/service.rs
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use std::sync::OnceLock;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::auth::{dto::*, jwt, repo::AuthRepo},
    api::user::repo as user_repo,
    error::{AppError, AppResult},
    state::AppState,
};
use tracing::{info, warn};

pub struct AuthService;

impl AuthService {
    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }
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
    // Helper to map login method for DB
    fn map_login_method_for_db_password() -> &'static str {
        "email"
    }

    // Helper to map device for DB
    fn map_device_for_db(dev: Option<&str>) -> &'static str {
        match dev.unwrap_or("").to_ascii_lowercase().as_str() {
            "mobile" => "mobile",
            "tablet" => "tablet",
            "desktop" | "web" | "browser" => "desktop",
            _ => "other",
        }
    }

    // 리프레시 토큰 생성 및 해싱
    fn generate_refresh_token_and_hash(session_id: &str) -> (String, String) {
        let random_uuid = Uuid::new_v4().to_string();
        let payload = format!("{session_id}:{random_uuid}");
        let refresh_token = URL_SAFE_NO_PAD.encode(payload.as_bytes());
        let refresh_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(payload.as_bytes()));
        (refresh_token, refresh_hash)
    }

    // 리프레시 토큰 해싱 (주어진 토큰)
    fn hash_refresh_token(token: &str) -> AppResult<String> {
        let decoded_bytes = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        Ok(URL_SAFE_NO_PAD.encode(Sha256::digest(decoded_bytes)))
    }

    fn parse_refresh_token(token: &str) -> AppResult<(String, String)> {
        let decoded = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        let incoming_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(&decoded));
        let decoded_str = String::from_utf8(decoded)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        let mut parts = decoded_str.splitn(2, ':');
        let session_id = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        let random_part = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        let _ = Uuid::parse_str(session_id)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        let _ = Uuid::parse_str(random_part)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        Ok((session_id.to_string(), incoming_hash))
    }

    // 로그인 서비스
    #[allow(clippy::too_many_arguments)]
    pub async fn login(
        st: &AppState,
        req: LoginReq,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<(LoginRes, Cookie<'static>, i64)> {
        // 0) 이메일 정규화 및 유효성 검사
        let email = req.email.trim().to_lowercase();
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!(
                "AUTH_400_INVALID_INPUT: {}",
                e
            )));
        }

        // (B) 로그인 레이트리밋 체크
        let rl_key = format!("rl:login:{}:{}", email.to_lowercase(), login_ip.clone());
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn
                .expire(&rl_key, st.cfg.rate_limit_login_window_sec)
                .await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests(
                "AUTH_429_TOO_MANY_ATTEMPTS".into(),
            ));
        }

        // 1) 사용자 조회 (없어도 타이밍 보호를 위해 더미 검증)
        let user_info = AuthRepo::find_user_by_email(&st.db, &email).await?;

        // 2) 비밀번호 검증 (존재하지 않아도 더미 해시로 동일 비용 수행)
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

        // 3) 사용자 상태 확인
        if !user_info.user_state {
            return Err(AppError::Forbidden);
        }

        // 4) 세션 및 리프레시 토큰 생성
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) =
            Self::generate_refresh_token_and_hash(&session_id);

        let refresh_ttl_secs = st.cfg.refresh_ttl_days * 24 * 3600;

        // (C) jwt.rs의 create_token을 실제로 사용하도록 service.rs에서 호출
        let access_token_res = jwt::create_token(
            user_info.user_id,
            &session_id,
            st.cfg.jwt_access_ttl_min,
            &st.cfg.jwt_secret,
        )?;

        // 5) DB에 로그인 기록 및 로그 삽입 (트랜잭션)
        let mut tx = st.db.begin().await?;
        let mapped_device = Self::map_device_for_db(req.device.as_deref());
        let _login_method = Self::map_login_method_for_db_password();

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
        )
        .await?;

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
        )
        .await?;
        tx.commit().await?;

        // 6) Redis에 세션 정보 저장 (커밋 성공 후)
        let _: () = redis_conn
            .set_ex(
                format!("ak:session:{}", session_id),
                user_info.user_id,
                st.cfg.jwt_access_ttl_min as u64 * 60, // Access token TTL in seconds
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: () = redis_conn
            .set_ex(
                format!("ak:refresh:{}", refresh_hash),
                &session_id,
                refresh_ttl_secs as u64,
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: () = redis_conn
            .sadd(
                format!("ak:user_sessions:{}", user_info.user_id),
                &session_id,
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // (A) refresh cookie 설정
        let mut refresh_cookie =
            Cookie::new(st.cfg.refresh_cookie_name.clone(), refresh_token_value);
        refresh_cookie.set_path("/");
        refresh_cookie.set_http_only(true);
        refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
        refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
            "Strict" => SameSite::Strict,
            "Lax" => SameSite::Lax,
            "None" => SameSite::None,
            _ => SameSite::Lax, // Default to Lax
        });
        refresh_cookie
            .set_expires(OffsetDateTime::now_utc() + time::Duration::seconds(refresh_ttl_secs));
        refresh_cookie.set_domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default());

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

    // 리프레시 서비스
    pub async fn refresh(
        st: &AppState,
        old_refresh_token: &str,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<(LoginRes, String, i64)> {
        let (session_id, incoming_hash) = Self::parse_refresh_token(old_refresh_token)?;

        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 1) 세션 조회 (FOR UPDATE)
        let mut tx = st.db.begin().await?;
        let login_record =
            AuthRepo::find_login_by_session_id_for_update_tx(&mut tx, &session_id).await?;
        let login_record = match login_record {
            Some(record) => record,
            None => return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into())),
        };

        // 2) 재사용 탐지
        if login_record.refresh_hash != incoming_hash {
            AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "compromised")
                .await?;
            AuthRepo::insert_login_log_tx(
                &mut tx,
                login_record.user_id,
                "reuse_detected",
                false,
                &session_id,
                &login_record.refresh_hash,
                login_record
                    .login_ip
                    .as_deref()
                    .unwrap_or(login_ip.as_str()),
                Some(login_record.login_device.as_str()),
                login_record.login_browser.as_deref(),
                login_record.login_os.as_deref(),
                user_agent.as_deref(),
            )
            .await?;
            tx.commit().await?;

            let _: () = redis_conn
                .del(format!("ak:refresh:{}", login_record.refresh_hash))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            let _: () = redis_conn
                .del(format!("ak:session:{}", session_id))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            let _: () = redis_conn
                .srem(format!("ak:user_sessions:{}", login_record.user_id), &session_id)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            return Err(AppError::Conflict("AUTH_409_REUSE_DETECTED".into()));
        }

        // 3) 세션 상태 확인
        if login_record.state != "active" {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()));
        }

        // 4) 새 리프레시 토큰 생성 및 해싱
        let (new_refresh_token_value, new_refresh_hash) =
            Self::generate_refresh_token_and_hash(&session_id);

        // 5) DB 업데이트 + 로그 기록
        AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash).await?;
        AuthRepo::insert_login_log_tx(
            &mut tx,
            login_record.user_id,
            "rotate",
            true,
            &session_id,
            &new_refresh_hash,
            login_record
                .login_ip
                .as_deref()
                .unwrap_or(login_ip.as_str()),
            Some(login_record.login_device.as_str()),
            login_record.login_browser.as_deref(),
            login_record.login_os.as_deref(),
            user_agent.as_deref(),
        )
        .await?;
        tx.commit().await?;

        // 6) Redis Sync
        let _: () = redis_conn
            .del(format!("ak:refresh:{}", login_record.refresh_hash))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let refresh_ttl_secs = st.cfg.refresh_ttl_days * 24 * 3600;

        let _: () = redis_conn
            .set_ex(
                format!("ak:refresh:{}", new_refresh_hash),
                &session_id,
                refresh_ttl_secs as u64,
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 7) 새 액세스 토큰 생성
        let access_token_res = jwt::create_token(
            login_record.user_id,
            &session_id,
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

    pub async fn find_id(st: &AppState, req: FindIdReq, client_ip: String) -> AppResult<FindIdRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!("AUTH_400_INVALID_INPUT: {}", e)));
        }

        let rl_key = format!("rl:find_id:{}", client_ip);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn
                .expire(&rl_key, st.cfg.rate_limit_login_window_sec)
                .await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests(
                "AUTH_429_TOO_MANY_ATTEMPTS".into(),
            ));
        }

        let user = AuthRepo::find_user_by_name_and_email(&st.db, &req.name, &req.email).await?;
        if let Some(found) = user {
            let _ = user_repo::insert_user_log_after(
                &st.db,
                Some(found.user_id),
                found.user_id,
                "find_id",
                true,
            )
            .await;
            info!("Find ID email simulation for {}", found.user_email);
        } else {
            warn!("Find ID request failed. User not found.");
        }

        Ok(FindIdRes {
            message: "If the account exists, the ID has been sent to your email.".to_string(),
        })
    }

    pub async fn reset_password(
        st: &AppState,
        req: ResetPwReq,
        client_ip: String,
    ) -> AppResult<ResetPwRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(format!("AUTH_400_INVALID_INPUT: {}", e)));
        }

        if !Self::validate_password_policy(&req.new_password) {
            return Err(AppError::Unprocessable(
                "password policy violation".into(),
            ));
        }

        let rl_key = format!("rl:reset_pw:{}", client_ip);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn
                .expire(&rl_key, st.cfg.rate_limit_login_window_sec)
                .await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests(
                "AUTH_429_TOO_MANY_ATTEMPTS".into(),
            ));
        }

        let claims = jwt::decode_token(&req.reset_token, &st.cfg.jwt_secret)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_RESET_TOKEN".into()))?;
        let user_id = claims.sub;

        let salt = SaltString::generate(&mut OsRng);
        let params = argon2::Params::new(19_456, 2, 1, None).unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let new_password_hash = argon2
            .hash_password(req.new_password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
            .to_string();

        let mut tx = st.db.begin().await?;
        AuthRepo::update_user_password_tx(&mut tx, user_id, &new_password_hash).await?;
        user_repo::insert_user_log_after_tx(&mut tx, Some(user_id), user_id, "reset_pw", true)
            .await?;
        AuthRepo::update_login_state_by_user_tx(&mut tx, user_id, "revoked").await?;
        tx.commit().await?;

        let session_key = format!("ak:user_sessions:{}", user_id);
        let session_ids: Vec<String> = redis_conn.smembers(&session_key).await.unwrap_or_default();
        for sid in session_ids.iter() {
            if let Some(login_record) = AuthRepo::find_login_by_session_id(&st.db, sid).await? {
                let _: () = redis_conn
                    .del(format!("ak:refresh:{}", login_record.refresh_hash))
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
            }
            let _: () = redis_conn
                .del(format!("ak:session:{}", sid))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            let _: () = redis_conn
                .srem(&session_key, sid)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
        let _: () = redis_conn
            .del(&session_key)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(ResetPwRes {
            message: "Password has been reset. All active sessions are invalidated.".to_string(),
        })
    }

    // 로그아웃 서비스
    pub async fn logout(
        st: &AppState,
        user_id: i64,
        session_id: &str,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 1) DB 로그인 상태 업데이트 및 로그 기록 (트랜잭션)
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
            )
            .await?;
        }
        tx.commit().await?;

        // 2) Redis 키 삭제 (커밋 성공 후)
        if let Some(record) = login_record {
            let _: () = redis_conn
                .del(format!("ak:refresh:{}", record.refresh_hash))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
        let _: () = redis_conn
            .del(format!("ak:session:{}", session_id))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis_conn
            .srem(format!("ak:user_sessions:{}", user_id), session_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    // 모든 세션 로그아웃 서비스
    pub async fn logout_all(
        st: &AppState,
        refresh_token: Option<&str>,
        req: LogoutAllReq,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<LogoutRes> {
        let mut user_id: Option<i64> = None;
        let mut current_session_id: Option<String> = None;

        if let Some(token) = refresh_token {
            let refresh_hash = Self::hash_refresh_token(token)?;
            let mut redis_conn = st
                .redis
                .get()
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            if let Ok(sid) = redis_conn
                .get::<_, String>(format!("ak:refresh:{}", refresh_hash))
                .await
            {
                if let Ok(uid) = redis_conn
                    .get::<_, i64>(format!("ak:session:{}", sid))
                    .await
                {
                    user_id = Some(uid);
                    current_session_id = Some(sid);
                }
            }
        }

        let Some(uid) = user_id else {
            return Err(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()));
        };

        // 1) DB에 모든 세션 로그아웃 기록 (트랜SACTION)
        let mut tx = st.db.begin().await?;
        let mut logged_out_session_ids = Vec::new();

        if req.everywhere {
            let session_ids: Vec<String> = AuthRepo::find_user_session_ids_tx(&mut tx, uid).await?;

            for sid in session_ids {
                if let Some(login_record) =
                    AuthRepo::find_login_by_session_id_tx(&mut tx, &sid).await?
                {
                    let refresh_hash = login_record.refresh_hash;
                    AuthRepo::update_login_state_by_session_tx(&mut tx, &sid, "logged_out").await?;
                    AuthRepo::insert_logout_log_tx(
                        &mut tx,
                        uid,
                        &sid,
                        &refresh_hash,
                        &login_ip,
                        user_agent.as_deref(),
                    )
                    .await?;
                    logged_out_session_ids.push(sid);
                }
            }
            AuthRepo::update_login_state_by_user_tx(&mut tx, uid, "logged_out").await?;
        } else if let Some(sid) = current_session_id {
            if let Some(login_record) = AuthRepo::find_login_by_session_id_tx(&mut tx, &sid).await?
            {
                let refresh_hash = login_record.refresh_hash;
                AuthRepo::update_login_state_by_session_tx(&mut tx, &sid, "logged_out").await?;
                AuthRepo::insert_logout_log_tx(
                    &mut tx,
                    uid,
                    &sid,
                    &refresh_hash,
                    &login_ip,
                    user_agent.as_deref(),
                )
                .await?;
                logged_out_session_ids.push(sid);
            }
        }
        tx.commit().await?;

        // 2) Redis 키 삭제 (커밋 성공 후)
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        for sid in logged_out_session_ids {
            if let Some(login_record) = AuthRepo::find_login_by_session_id(&st.db, &sid).await? {
                let refresh_hash = login_record.refresh_hash;
                let _: () = redis_conn
                    .del(format!("ak:refresh:{}", refresh_hash))
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                let _: () = redis_conn
                    .del(format!("ak:session:{}", sid))
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                let _: () = redis_conn
                    .srem(format!("ak:user_sessions:{}", uid), &sid)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
            }
        }
        if req.everywhere {
            let _: () = redis_conn
                .del(format!("ak:user_sessions:{}", uid))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }

        Ok(LogoutRes { ok: true })
    }
}
