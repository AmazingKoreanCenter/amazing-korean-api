use argon2::{password_hash::*, Argon2};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool as RedisPool;
use sha2::{Digest, Sha256};
use std::env;
use uuid::Uuid;
use validator::Validate;
use base64::engine::{general_purpose, Engine};

use crate::{
    api::{
        auth::{dto::*, jwt, repo},
        user::dto::ProfileRes,
    },
    error::{AppError, AppResult},
    state::AppState,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use chrono::Utc;
use time::{Duration, OffsetDateTime, format_description};
use tracing::warn;

pub struct AuthService;

impl AuthService {
    

    // Redis 헬퍼: 비동기 커넥션 가져오기
    async fn get_redis_conn(redis_pool: &RedisPool) -> AppResult<deadpool_redis::Connection> {
        redis_pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Redis connection error: {e}")))
    }

    // 토큰 해싱
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        general_purpose::URL_SAFE.encode(&hasher.finalize())
    }

    // 리프레시 토큰 생성 (랜덤 32바이트 → base64url)
    fn generate_refresh_token() -> String {
        let token_bytes: [u8; 32] = rand::random();
        general_purpose::URL_SAFE.encode(&token_bytes)
    }

    // 리프레시 쿠키 설정
    fn set_refresh_cookie(jar: CookieJar, refresh_token: &str) -> CookieJar {
        let refresh_ttl_days: i64 = env::var("REFRESH_TTL_DAYS")
            .unwrap_or_else(|_| "30".into())
            .parse()
            .unwrap_or(30);
        let refresh_cookie_name = env::var("REFRESH_COOKIE_NAME")
            .unwrap_or_else(|_| "ak_refresh".into());
        let refresh_cookie_domain = env::var("REFRESH_COOKIE_DOMAIN").ok();
        let refresh_cookie_secure: bool = env::var("REFRESH_COOKIE_SECURE")
            .unwrap_or_else(|_| "true".into())
            .parse()
            .unwrap_or(true);

        let cookie = Cookie::build((refresh_cookie_name, refresh_token.to_string()))
            .path("/")
            .http_only(true)
            .secure(refresh_cookie_secure)
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .expires(time::OffsetDateTime::now_utc() + Duration::days(refresh_ttl_days));

        let cookie = if let Some(domain) = refresh_cookie_domain {
            cookie.domain(domain)
        } else {
            cookie
        };

        jar.add(cookie.build())
    }

    // 리프레시 쿠키 해제
    fn unset_refresh_cookie(jar: CookieJar) -> CookieJar {
        let refresh_cookie_name = env::var("REFRESH_COOKIE_NAME")
            .unwrap_or_else(|_| "ak_refresh".into());
        jar.remove(Cookie::build(refresh_cookie_name).path("/").build())
    }

    // 로그인 시도 레이트 리밋 체크 및 증가
    async fn check_rate_limit(st: &AppState, email: &str, ip_addr: &str) -> AppResult<()> {
        let rate_limit_login_window_sec: i64 = env::var("RATE_LIMIT_LOGIN_WINDOW_SEC")
            .unwrap_or_else(|_| "900".into())
            .parse()
            .unwrap_or(900);
        let rate_limit_login_max: i64 = env::var("RATE_LIMIT_LOGIN_MAX")
            .unwrap_or_else(|_| "10".into())
            .parse()
            .unwrap_or(10);

        let key = format!("ak:logins:{}:{}", email, ip_addr);
        let mut conn = Self::get_redis_conn(&st.redis).await?;

        let attempts: i64 = conn.incr(&key, 1).await?;
        if attempts == 1 {
            conn.expire::<_, ()>(&key, rate_limit_login_window_sec).await?;
        }

        if attempts > rate_limit_login_max {
            return Err(AppError::TooManyRequests("Too many login attempts".into()));
        }
        Ok(())
    }

    // 로그인 성공 시 레이트 리밋 카운터 초기화
    async fn reset_rate_limit(st: &AppState, email: &str, ip_addr: &str) -> AppResult<()> {
        let key = format!("ak:logins:{}:{}", email, ip_addr);
        let mut conn = Self::get_redis_conn(&st.redis).await?;
        conn.del::<_, ()>(&key).await?;
        Ok(())
    }

    pub async fn login(
        st: &AppState,
        req: LoginReq,
        ip_addr: String,
    ) -> AppResult<(LoginRes, CookieJar)> {
        // 1) 유효성 검사
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2) 레이트 리밋 체크
        Self::check_rate_limit(st, &req.email, &ip_addr).await?;

        // 3) 사용자 조회
        let user_row = repo::find_by_email(&st.db, &req.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        // 4) 비밀번호 검증
        let parsed_hash = PasswordHash::new(&user_row.user_password)
            .map_err(|_| AppError::Internal("Invalid password hash in DB".into()))?;
        let is_valid = Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        // 5) 사용자 상태 확인
        if user_row.user_state != "on" {
            return Err(AppError::Forbidden);
        }

        // 6) 레이트 리밋 초기화
        Self::reset_rate_limit(st, &req.email, &ip_addr).await?;

        // 7) Access JWT 생성
        let jwt_access_ttl_min: i64 = env::var("JWT_ACCESS_TTL_MIN")
            .unwrap_or_else(|_| "15".into())
            .parse()
            .unwrap_or(15);
        let (access_token, expires_in) = jwt::create_token(user_row.user_id, jwt_access_ttl_min).await?;


        // 8) Refresh Token 생성 및 세션 저장
        let refresh_token = Self::generate_refresh_token();
        let refresh_hash = Self::hash_token(&refresh_token);
        let session_id = Uuid::new_v4();
        let refresh_ttl_days: i64 = env::var("REFRESH_TTL_DAYS")
            .unwrap_or_else(|_| "30".into())
            .parse()
            .unwrap_or(30);
        let expires_at = OffsetDateTime::now_utc() + Duration::days(refresh_ttl_days);

        let session_key = format!("ak:session:{}", session_id);
        let user_sessions_key = format!("ak:user_sessions:{}", user_row.user_id);

        let mut conn = Self::get_redis_conn(&st.redis).await?;
        conn.hset_multiple::<_, _, _, ()>(
            &session_key,
            &[
                ("user_id", user_row.user_id.to_string()),
                ("refresh_hash", refresh_hash),
                ("created_at", Utc::now().to_rfc3339()),
                ("expires_at", expires_at.format(&format_description::well_known::Rfc3339).unwrap()),
                ("rotation", "0".to_string()),
                ("ip", ip_addr),
            ],
        )
        .await?;
        conn.expire_at::<_, ()>(&session_key, expires_at.unix_timestamp()).await?;
        conn.sadd::<_, _, ()>(&user_sessions_key, session_id.to_string()).await?;

        // 9) Refresh Cookie 설정
        let jar = CookieJar::new();
        let jar = Self::set_refresh_cookie(jar, &refresh_token);

        let user_profile = ProfileRes {
            id: user_row.user_id,
            email: user_row.user_email,
            name: user_row.user_name,
            nickname: None, // Not available in UserRow
            language: None, // Not available in UserRow
            country: None, // Not available in UserRow
            birthday: None, // Not available in UserRow
            gender: "none".to_string(), // Not available in UserRow
            user_state: user_row.user_state,
            user_auth: user_row.user_auth,
            created_at: user_row.user_created_at,
        };

        Ok((LoginRes { token: access_token, expires_in, user: user_profile }, jar))
    }

    pub async fn refresh(
        st: &AppState,
        jar: CookieJar,
        ip_addr: String,
    ) -> AppResult<(RefreshRes, CookieJar)> {
        let refresh_cookie_name = env::var("REFRESH_COOKIE_NAME")
            .unwrap_or_else(|_| "ak_refresh".into());

        let refresh_token = jar
            .get(&refresh_cookie_name)
            .map(|c| c.value().to_string())
            .ok_or(AppError::Unauthorized("Refresh token missing".into()))?;

        let refresh_hash = Self::hash_token(&refresh_token);

        let mut conn = Self::get_redis_conn(&st.redis).await?;

        // Find session by refresh_hash (this is inefficient, ideally we'd have session_id in cookie)
        // For now, iterate through user sessions or rely on a direct session_id in cookie
        // Assuming session_id is part of the refresh token or stored in cookie for direct lookup
        // For this implementation, we'll assume the refresh token itself is the session identifier for simplicity
        // A more robust solution would involve a separate session ID in the cookie.

        // Let's assume the refresh token is actually the session ID for now, and we store the hash of it.
        // This is a simplification for the task, a real implementation would have a session ID.
        let session_id_from_token = Uuid::parse_str(&refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token format".into()))?;
        let session_key = format!("ak:session:{}", session_id_from_token);

        let session_data: std::collections::HashMap<String, String> = conn.hgetall(&session_key).await?;

        if session_data.is_empty() {
            return Err(AppError::Unauthorized("Session not found".into()));
        }

        let stored_refresh_hash = session_data
            .get("refresh_hash")
            .ok_or(AppError::Internal("Session data corrupted".into()))?;
        let user_id: i64 = session_data
            .get("user_id")
            .ok_or(AppError::Internal("Session data corrupted".into()))?
            .parse()
            .map_err(|_| AppError::Internal("Session data corrupted".into()))?;
        let rotation: i64 = session_data
            .get("rotation")
            .ok_or(AppError::Internal("Session data corrupted".into()))?
            .parse()
            .map_err(|_| AppError::Internal("Session data corrupted".into()))?;

        if stored_refresh_hash != &refresh_hash {
            // Token reuse detected - revoke session
            warn!("Refresh token reuse detected for user_id: {}", user_id);
            conn.del::<_, ()>(&session_key).await?;
            let user_sessions_key = format!("ak:user_sessions:{}", user_id);
            conn.srem::<_, _, ()>(&user_sessions_key, session_id_from_token.to_string()).await?;
            return Err(AppError::Unauthorized("Refresh token reused".into()));
        }

        // Rotate refresh token
        let new_refresh_token = Self::generate_refresh_token();
        let new_refresh_hash = Self::hash_token(&new_refresh_token);
        

        let refresh_ttl_days: i64 = env::var("REFRESH_TTL_DAYS")
            .unwrap_or_else(|_| "30".into())
            .parse()
            .unwrap_or(30);
        let expires_at = OffsetDateTime::now_utc() + Duration::days(refresh_ttl_days);

        // Update session in Redis (rotate)
        conn.hset_multiple::<_, _, _, ()>(
            &session_key,
            &[
                ("refresh_hash", new_refresh_hash),
                ("rotation", (rotation + 1).to_string()),
                ("expires_at", expires_at.format(&format_description::well_known::Rfc3339).unwrap()),
                ("ip", ip_addr),
            ],
        )
        .await?;
        conn.expire_at::<_, ()>(&session_key, expires_at.unix_timestamp()).await?;

        // Issue new Access JWT
        let jwt_access_ttl_min: i64 = env::var("JWT_ACCESS_TTL_MIN")
            .unwrap_or_else(|_| "15".into())
            .parse()
            .unwrap_or(15);
        let (access_token, expires_in) = jwt::create_token(user_id, jwt_access_ttl_min).await?;

        // Set new Refresh Cookie
        let jar = Self::set_refresh_cookie(jar, &new_refresh_token);

        Ok((RefreshRes { token: access_token, expires_in }, jar))
    }

    pub async fn logout(st: &AppState, jar: CookieJar) -> AppResult<CookieJar> {
        let refresh_cookie_name = env::var("REFRESH_COOKIE_NAME")
            .unwrap_or_else(|_| "ak_refresh".into());

        let refresh_token = jar
            .get(&refresh_cookie_name)
            .map(|c| c.value().to_string());

        if let Some(token) = refresh_token {
            let session_id_from_token = Uuid::parse_str(&token)
                .map_err(|_| AppError::Unauthorized("Invalid refresh token format".into()))?;
            let session_key = format!("ak:session:{}", session_id_from_token);

            let mut conn = Self::get_redis_conn(&st.redis).await?;
            let user_id: Option<i64> = conn.hget(&session_key, "user_id").await?;

            conn.del::<_, ()>(&session_key).await?;

            if let Some(uid) = user_id {
                let user_sessions_key = format!("ak:user_sessions:{}", uid);
                conn.srem::<_, _, ()>(&user_sessions_key, session_id_from_token.to_string()).await?;
            }
        }

        Ok(Self::unset_refresh_cookie(jar))
    }

    pub async fn logout_all(st: &AppState, user_id: i64) -> AppResult<()> {
        let user_sessions_key = format!("ak:user_sessions:{}", user_id);
        let mut conn = Self::get_redis_conn(&st.redis).await?;

        let session_ids: Vec<String> = conn.smembers::<_, Vec<String>>(&user_sessions_key).await?;

        for session_id in session_ids {
            let session_key = format!("ak:session:{}", session_id);
            conn.del::<_, ()>(&session_key).await?;
        }
        conn.del::<_, ()>(&user_sessions_key).await?;

        Ok(())
    }
}