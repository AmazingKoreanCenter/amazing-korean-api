// FILE: src/api/auth/service.rs
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::{thread_rng, Rng};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::auth::{dto::*, jwt, repo::AuthRepo},
    error::{AppError, AppResult},
    state::AppState,
};

pub struct AuthService;

impl AuthService {
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
    fn generate_refresh_token_and_hash() -> (String, String) {
        let mut refresh_bytes = [0u8; 32];
        thread_rng().fill(&mut refresh_bytes);
        let refresh_token = URL_SAFE_NO_PAD.encode(refresh_bytes);
        let refresh_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(refresh_bytes));
        (refresh_token, refresh_hash)
    }

    // 리프레시 토큰 해싱 (주어진 토큰)
    fn hash_refresh_token(token: &str) -> AppResult<String> {
        let decoded_bytes = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;
        Ok(URL_SAFE_NO_PAD.encode(Sha256::digest(decoded_bytes)))
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

        // 1) 사용자 조회
        let user_info = AuthRepo::find_user_by_email(&st.db, &email)
            .await?
            .ok_or(AppError::Unauthorized("AUTH_401_BAD_CREDENTIALS".into()))?;

        // 2) 사용자 상태 확인
        if !user_info.user_state {
            return Err(AppError::Forbidden);
        }

        // 3) 비밀번호 검증
        let parsed_hash = PasswordHash::new(&user_info.user_password)
            .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?;
        Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized("AUTH_401_BAD_CREDENTIALS".into()))?;

        // 4) 세션 및 리프레시 토큰 생성
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) = Self::generate_refresh_token_and_hash();

        let refresh_ttl_secs = st.cfg.refresh_ttl_days * 24 * 3600;

        // (C) jwt.rs의 create_token을 실제로 사용하도록 service.rs에서 호출
        let access_token_res = jwt::create_token(
            user_info.user_id,
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
    ) -> AppResult<(RefreshRes, Cookie<'static>, i64)> {
        let old_refresh_hash = Self::hash_refresh_token(old_refresh_token)?;

        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 1) Redis에서 세션 ID 조회
        let session_id: String = redis_conn
            .get(format!("ak:refresh:{}", old_refresh_hash))
            .await
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_INVALID_REFRESH".into()))?;

        // 2) Redis에서 사용자 ID 조회
        let user_id: i64 = redis_conn
            .get(format!("ak:session:{}", session_id))
            .await
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        // 3) 새 리프레시 토큰 생성 및 해싱 (트랜잭션 시작 전)
        let (new_refresh_token_value, new_refresh_hash) = Self::generate_refresh_token_and_hash();

        // 4) DB에 리프레시 토큰 해시 업데이트 및 로그 기록 (트랜잭션)
        let mut tx = st.db.begin().await?;
        AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash).await?;
        AuthRepo::insert_login_log_tx(
            &mut tx,
            user_id,
            "rotate",
            true,
            &session_id,
            &new_refresh_hash,
            &login_ip,
            None, // device
            None, // browser
            None, // os
            user_agent.as_deref(),
        )
        .await?;
        tx.commit().await?;

        // 5) Redis에서 기존 리프레시 토큰 삭제, 새 리프레시 토큰 저장 (커밋 성공 후)
        let _: () = redis_conn
            .del(format!("ak:refresh:{}", old_refresh_hash))
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

        // 6) 새 액세스 토큰 생성
        let access_token_res =
            jwt::create_token(user_id, st.cfg.jwt_access_ttl_min, &st.cfg.jwt_secret)?;

        // (A) refresh cookie 설정
        let mut refresh_cookie =
            Cookie::new(st.cfg.refresh_cookie_name.clone(), new_refresh_token_value);
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
            RefreshRes {
                access_token: access_token_res.access_token,
                expires_in: access_token_res.expires_in,
            },
            refresh_cookie.into_owned(),
            refresh_ttl_secs,
        ))
    }

    // 로그아웃 서비스
    pub async fn logout(
        st: &AppState,
        refresh_token: Option<&str>,
        login_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<LogoutRes> {
        let Some(refresh_token) = refresh_token else {
            return Ok(LogoutRes { ok: true }); // No refresh token, no-op
        };

        let refresh_hash = Self::hash_refresh_token(refresh_token)?;

        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // 1) Redis에서 세션 ID 조회
        let session_id: String = redis_conn
            .get(format!("ak:refresh:{}", refresh_hash))
            .await
            .map_err(|_| AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

        // 2) Redis에서 사용자 ID 조회
        let user_id: i64 = match redis_conn.get(format!("ak:session:{}", session_id)).await {
            Ok(uid) => uid,
            Err(_) => return Ok(LogoutRes { ok: true }), // Already logged out or invalid, no-op
        };

        // 3) DB 로그인 상태 업데이트 및 로그 기록 (트랜잭션)
        let mut tx = st.db.begin().await?;
        AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "logged_out").await?;
        AuthRepo::insert_logout_log_tx(
            &mut tx,
            user_id,
            &session_id,
            &refresh_hash,
            &login_ip,
            user_agent.as_deref(),
        )
        .await?;
        tx.commit().await?;

        // 4) Redis 키 삭제 (커밋 성공 후)
        let _: () = redis_conn
            .del(format!("ak:refresh:{}", refresh_hash))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis_conn
            .del(format!("ak:session:{}", session_id))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis_conn
            .srem(format!("ak:user_sessions:{}", user_id), &session_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(LogoutRes { ok: true })
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
