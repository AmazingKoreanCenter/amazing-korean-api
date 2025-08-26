use argon2::{password_hash::*, Argon2};
use base64::engine::{general_purpose, Engine};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool as RedisPool;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::{
        auth::{dto::*, jwt, repo, token_utils},
        user::dto::ProfileRes,
    },
    error::{AppError, AppResult},
    state::AppState,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use chrono::Utc;
use time::{format_description, Duration, OffsetDateTime};
use tracing::warn;

pub struct AuthService;

impl AuthService {
    // Redis Key Helpers
    fn key_session(session_id: &Uuid) -> String {
        format!("ak:session:{}", session_id)
    }
    fn key_refresh_map(hash_b64: &str) -> String {
        format!("ak:refresh:{}", hash_b64)
    }

    // Redis 헬퍼: 비동기 커넥션 가져오기
    async fn get_redis_conn(redis_pool: &RedisPool) -> AppResult<deadpool_redis::Connection> {
        redis_pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Redis connection error: {e}")))
    }

    // 토큰 해싱
    fn hash_token(token_bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token_bytes);
        general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
    }

    // 리프레시 쿠키 설정
    fn set_refresh_cookie(jar: CookieJar, st: &AppState, refresh_cookie_value: &str) -> CookieJar {
        let cfg = &st.cfg;

        let expires_at = OffsetDateTime::now_utc() + Duration::days(cfg.refresh_ttl_days);
        let format = format_description::parse(
            "[weekday], [day] [month] [year] [hour]:[minute]:[second] GMT",
        )
        .unwrap();
        let expires_http_date = expires_at.format(&format).unwrap();

        let mut cookie_str = format!(
            "{}={}; HttpOnly; Path=/; SameSite={}; Expires={}",
            cfg.refresh_cookie_name,
            refresh_cookie_value,
            cfg.refresh_cookie_samesite_or("Lax"),
            expires_http_date
        );

        if cfg.refresh_cookie_secure {
            cookie_str.push_str("; Secure");
        }
        if let Some(d) = &cfg.refresh_cookie_domain {
            cookie_str.push_str(&format!("; Domain={}", d));
        }

        jar.add(Cookie::parse(cookie_str).unwrap())
    }

    // 리프레시 쿠키 해제
    fn unset_refresh_cookie(jar: CookieJar, st: &AppState) -> CookieJar {
        let cfg = &st.cfg;
        jar.remove(
            Cookie::build(cfg.refresh_cookie_name.clone())
                .path("/")
                .build(),
        )
    }

    // 로그인 시도 레이트 리밋 체크 및 증가
    async fn check_rate_limit(st: &AppState, email: &str, ip_addr: &str) -> AppResult<()> {
        let cfg = &st.cfg;
        let key = format!("ak:logins:{}:{}", email, ip_addr);
        let mut conn = Self::get_redis_conn(&st.redis).await?;

        let attempts: i64 = conn.incr(&key, 1).await?;
        if attempts == 1 {
            conn.expire::<_, ()>(&key, cfg.rate_limit_login_window_sec)
                .await?;
        }

        if attempts > cfg.rate_limit_login_max {
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
        let (access_token, expires_in) =
            jwt::create_token(user_row.user_id, st.cfg.jwt_access_ttl_min).await?;

        // 8) Refresh Token 생성 및 세션 저장
        let (refresh_cookie_value, raw_refresh_token_bytes) =
            token_utils::generate_refresh_cookie_value();
        let refresh_hash = Self::hash_token(&raw_refresh_token_bytes);
        let session_id = Uuid::new_v4();
        let expires_at = OffsetDateTime::now_utc() + Duration::days(st.cfg.refresh_ttl_days);

        let session_key = Self::key_session(&session_id);
        let user_sessions_key = format!("ak:user_sessions:{}", user_row.user_id);

        let mut conn = Self::get_redis_conn(&st.redis).await?;

        // Use pipeline for atomicity
        let expires_at_unix = expires_at.unix_timestamp();
        let ttl_secs = (expires_at_unix - OffsetDateTime::now_utc().unix_timestamp()).max(1);

        redis::pipe()
            .atomic()
            .hset_multiple(
                &session_key,
                &[
                    ("user_id", user_row.user_id.to_string()),
                    ("refresh_hash", refresh_hash.clone()),
                    ("created_at", Utc::now().to_rfc3339()),
                    (
                        "expires_at",
                        expires_at
                            .format(&format_description::well_known::Rfc3339)
                            .unwrap(),
                    ),
                    ("rotation", "0".to_string()),
                    ("ip", ip_addr),
                ],
            )
            .ignore()
            .expire_at(&session_key, expires_at_unix)
            .ignore()
            .sadd(&user_sessions_key, session_id.to_string())
            .ignore()
            .set(Self::key_refresh_map(&refresh_hash), session_id.to_string())
            .ignore()
            .expire(Self::key_refresh_map(&refresh_hash), ttl_secs)
            .ignore()
            .query_async::<()>(&mut conn)
            .await?;

        // 9) Refresh Cookie 설정
        let jar = CookieJar::new();
        let jar = Self::set_refresh_cookie(jar, st, &refresh_cookie_value);

        let user_profile = ProfileRes {
            id: user_row.user_id,
            email: user_row.user_email,
            name: user_row.user_name,
            nickname: None,             // Not available in UserRow
            language: None,             // Not available in UserRow
            country: None,              // Not available in UserRow
            birthday: None,             // Not available in UserRow
            gender: "none".to_string(), // Not available in UserRow
            user_state: user_row.user_state,
            user_auth: user_row.user_auth,
            created_at: user_row.user_created_at,
        };

        Ok((
            LoginRes {
                token: access_token,
                expires_in,
                user: user_profile,
            },
            jar,
        ))
    }

    pub async fn refresh(
        st: &AppState,
        jar: CookieJar,
        ip_addr: String,
    ) -> AppResult<(RefreshRes, CookieJar)> {
        let refresh_cookie_name = &st.cfg.refresh_cookie_name;

        let refresh_token_from_cookie = jar
            .get(refresh_cookie_name)
            .map(|c| c.value().to_string())
            .ok_or(AppError::Unauthorized("Refresh token missing".into()))?;

        let raw_refresh_token_bytes =
            token_utils::parse_refresh_token_bytes(&refresh_token_from_cookie)?;
        let old_refresh_hash = Self::hash_token(&raw_refresh_token_bytes);

        let mut conn = Self::get_redis_conn(&st.redis).await?;

        // 1. Get session_id from refresh_hash mapping
        let session_id_str: String = conn
            .get::<_, Option<String>>(&Self::key_refresh_map(&old_refresh_hash))
            .await?
            .ok_or(AppError::Unauthorized(
                "Invalid or expired refresh token".into(),
            ))?;
        let session_id = Uuid::parse_str(&session_id_str)
            .map_err(|_| AppError::Internal("Invalid session ID in Redis mapping".into()))?;

        let session_key = Self::key_session(&session_id);

        let session_data: std::collections::HashMap<String, String> =
            conn.hgetall(&session_key).await?;

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

        if stored_refresh_hash != &old_refresh_hash {
            // Token reuse detected - revoke session
            warn!("Refresh token reuse detected for user_id: {}", user_id);
            conn.del::<_, ()>(&session_key).await?;
            let user_sessions_key = format!("ak:user_sessions:{}", user_id);
            conn.srem::<_, _, ()>(&user_sessions_key, session_id.to_string())
                .await?;
            // Also delete the old refresh map entry if it still exists (shouldn't if it was reused)
            conn.del::<_, ()>(&Self::key_refresh_map(&old_refresh_hash))
                .await?;
            return Err(AppError::Unauthorized("Refresh token reused".into()));
        }

        // Rotate refresh token
        let (new_refresh_cookie_value, new_raw_refresh_token_bytes) =
            token_utils::generate_refresh_cookie_value();
        let new_refresh_hash = Self::hash_token(&new_raw_refresh_token_bytes);

        let expires_at = OffsetDateTime::now_utc() + Duration::days(st.cfg.refresh_ttl_days);
        let expires_at_unix = expires_at.unix_timestamp();
        let ttl_secs = (expires_at_unix - OffsetDateTime::now_utc().unix_timestamp()).max(1);

        // Atomic update for session and refresh map
        redis::pipe()
            .atomic()
            .hset(&session_key, "refresh_hash", &new_refresh_hash)
            .ignore()
            .hset(&session_key, "rotation", (rotation + 1).to_string())
            .ignore()
            .hset(
                &session_key,
                "expires_at",
                expires_at
                    .format(&format_description::well_known::Rfc3339)
                    .unwrap(),
            )
            .ignore()
            .hset(&session_key, "ip", ip_addr)
            .ignore()
            .expire_at(&session_key, expires_at_unix)
            .ignore()
            .del(Self::key_refresh_map(&old_refresh_hash))
            .ignore()
            .set(
                Self::key_refresh_map(&new_refresh_hash),
                session_id.to_string(),
            )
            .ignore()
            .expire(Self::key_refresh_map(&new_refresh_hash), ttl_secs)
            .ignore()
            .query_async::<()>(&mut conn)
            .await?;

        // Issue new Access JWT
        let (access_token, expires_in) =
            jwt::create_token(user_id, st.cfg.jwt_access_ttl_min).await?;

        // Set new Refresh Cookie
        let jar = Self::set_refresh_cookie(jar, st, &new_refresh_cookie_value);

        Ok((
            RefreshRes {
                token: access_token,
                expires_in,
            },
            jar,
        ))
    }

    pub async fn logout(st: &AppState, jar: CookieJar) -> AppResult<CookieJar> {
        let refresh_cookie_name = &st.cfg.refresh_cookie_name;

        let refresh_token_from_cookie = jar.get(refresh_cookie_name).map(|c| c.value().to_string());

        if let Some(token) = refresh_token_from_cookie {
            let raw_refresh_token_bytes = token_utils::parse_refresh_token_bytes(&token)?;
            let refresh_hash = Self::hash_token(&raw_refresh_token_bytes);

            let mut conn = Self::get_redis_conn(&st.redis).await?;

            let session_id_str: Option<String> =
                conn.get(Self::key_refresh_map(&refresh_hash)).await?;

            if let Some(session_id_str) = session_id_str {
                let session_id = Uuid::parse_str(&session_id_str).map_err(|_| {
                    AppError::Internal("Invalid session ID in Redis mapping".into())
                })?;
                let session_key = Self::key_session(&session_id);

                let user_id: Option<i64> = conn.hget(&session_key, "user_id").await?;

                redis::pipe()
                    .atomic()
                    .del(&session_key)
                    .ignore()
                    .del(Self::key_refresh_map(&refresh_hash))
                    .ignore()
                    .query_async::<()>(&mut conn)
                    .await?;

                if let Some(uid) = user_id {
                    let user_sessions_key = format!("ak:user_sessions:{}", uid);
                    conn.srem::<_, _, ()>(&user_sessions_key, session_id.to_string())
                        .await?;
                }
            } else {
                // If refresh map entry doesn't exist, token is already invalid or expired.
                // Just unset the cookie.
                warn!(
                    "Logout: Refresh token map entry not found for hash: {}",
                    refresh_hash
                );
            }
        }

        Ok(Self::unset_refresh_cookie(jar, st))
    }

    pub async fn logout_all(st: &AppState, user_id: i64) -> AppResult<()> {
        let user_sessions_key = format!("ak:user_sessions:{}", user_id);
        let mut conn = Self::get_redis_conn(&st.redis).await?;

        let session_ids: Vec<String> = conn.smembers::<_, Vec<String>>(&user_sessions_key).await?;

        for session_id_str in session_ids {
            let session_id = Uuid::parse_str(&session_id_str).map_err(|_| {
                AppError::Internal("Invalid session ID in user sessions set".into())
            })?;
            let session_key = Self::key_session(&session_id);

            // Get refresh_hash from session to delete the refresh map entry
            let refresh_hash: Option<String> = conn.hget(&session_key, "refresh_hash").await?;

            redis::pipe()
                .atomic()
                .del(&session_key)
                .ignore()
                .query_async::<()>(&mut conn)
                .await?;

            if let Some(hash) = refresh_hash {
                conn.del::<_, ()>(Self::key_refresh_map(&hash)).await?;
            }
        }
        conn.del::<_, ()>(&user_sessions_key).await?;

        Ok(())
    }
}
