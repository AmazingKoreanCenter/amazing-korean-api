//! 관리자 초대/승격 서비스
//!
//! - 초대 코드 생성 및 이메일 발송
//! - 초대 코드 검증
//! - 관리자 계정 생성

use crate::api::auth::password;
use crate::crypto::CryptoService;
use chrono::{Duration, Utc};
use redis::AsyncCommands;
use tracing::{info, warn};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::admin::upgrade::dto::*,
    api::user::repo as user_repo,
    error::{AppError, AppResult},
    external::email::EmailTemplate,
    state::AppState,
    types::UserAuth,
};

/// 초대 코드 TTL (10분)
const INVITE_CODE_TTL_SEC: i64 = 600;

/// Redis 키 접두사
const REDIS_PREFIX: &str = "ak:upgrade:";

pub struct UpgradeService;

impl UpgradeService {
    // =========================================================================
    // Helper Functions
    // =========================================================================

    /// 비밀번호 정책 검증 (8자 이상, 영문+숫자 포함)
    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }


    /// 초대 코드 생성
    fn generate_invite_code() -> String {
        format!("ak_upgrade_{}", Uuid::new_v4())
    }

    /// RBAC 검증: 요청자가 해당 role을 초대할 수 있는지 확인
    /// - HYMN -> admin, manager
    /// - Admin -> manager
    /// - Manager -> 불가
    fn can_invite_role(actor_auth: &UserAuth, target_role: &str) -> bool {
        match (actor_auth, target_role) {
            (UserAuth::Hymn, "admin") => true,
            (UserAuth::Hymn, "manager") => true,
            (UserAuth::Admin, "manager") => true,
            _ => false,
        }
    }

    /// role 문자열을 UserAuth로 변환
    fn role_to_user_auth(role: &str) -> AppResult<UserAuth> {
        match role {
            "admin" => Ok(UserAuth::Admin),
            "manager" => Ok(UserAuth::Manager),
            _ => Err(AppError::BadRequest("Invalid role".into())),
        }
    }

    // =========================================================================
    // 7-68: 관리자 초대 (POST /admin/upgrade)
    // =========================================================================

    pub async fn create_invite(
        st: &AppState,
        actor_user_id: i64,
        req: UpgradeInviteReq,
    ) -> AppResult<UpgradeInviteRes> {
        // [Step 1] Input Validation
        req.validate()
            .map_err(|e| AppError::BadRequest(format!("UPGRADE_400_INVALID_INPUT: {}", e)))?;

        let email = req.email.trim().to_lowercase();
        let role = req.role.trim().to_lowercase();

        // [Step 2] Actor 권한 확인
        let actor = user_repo::find_user(&st.db, actor_user_id)
            .await?
            .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

        // [Step 3] RBAC 검증
        if !Self::can_invite_role(&actor.user_auth, &role) {
            warn!(
                actor_id = actor_user_id,
                actor_auth = ?actor.user_auth,
                target_role = %role,
                "Unauthorized invite attempt"
            );
            return Err(AppError::Forbidden(
                "UPGRADE_403_INSUFFICIENT_PERMISSION".into(),
            ));
        }

        // [Step 4] 이메일 중복 체크
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let email_idx_check = crypto.blind_index(&email)?;
        let existing = user_repo::find_user_by_email_idx(&st.db, &email_idx_check).await?;
        if existing.is_some() {
            return Err(AppError::Conflict(
                "UPGRADE_409_EMAIL_ALREADY_EXISTS".into(),
            ));
        }

        // Decrypt actor email for display
        let actor_email_plain = crypto.decrypt(&actor.email, "users.user_email")?;

        // [Step 5] 초대 코드 생성
        let invite_code = Self::generate_invite_code();
        let expires_at = Utc::now() + Duration::seconds(INVITE_CODE_TTL_SEC);

        // [Step 6] Redis에 초대 정보 저장
        let invite_data = InviteData {
            email: email.clone(),
            role: role.clone(),
            invited_by_id: actor_user_id,
            invited_by_email: actor_email_plain.clone(),
            created_at: Utc::now(),
        };
        let invite_json = serde_json::to_string(&invite_data)
            .map_err(|e| AppError::Internal(format!("Failed to serialize invite data: {}", e)))?;

        let redis_key = format!("{}{}", REDIS_PREFIX, invite_code);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: () = redis_conn
            .set_ex(&redis_key, &invite_json, INVITE_CODE_TTL_SEC as u64)
            .await?;

        // [Step 7] 이메일 발송
        let invite_url = format!(
            "{}/admin/upgrade/join?code={}",
            st.cfg.frontend_url, invite_code
        );

        let email_sender = st.email.as_ref()
            .ok_or_else(|| AppError::ServiceUnavailable("Email service not configured".into()))?;
        crate::external::email::send_templated(
            email_sender.as_ref(),
            &email,
            EmailTemplate::AdminInvite {
                invite_url,
                role: role.clone(),
                invited_by: actor_email_plain.clone(),
                expires_in_min: (INVITE_CODE_TTL_SEC / 60) as i32,
            },
        )
        .await?;

        info!(
            actor_id = actor_user_id,
            target_email = %email,
            target_role = %role,
            "Admin invite created"
        );

        Ok(UpgradeInviteRes {
            message: "Invitation sent successfully".into(),
            expires_at,
        })
    }

    // =========================================================================
    // 7-69: 초대 코드 검증 (GET /admin/upgrade/verify)
    // =========================================================================

    pub async fn verify_invite(st: &AppState, code: &str) -> AppResult<UpgradeVerifyRes> {
        // [Step 1] Redis에서 초대 정보 조회
        let redis_key = format!("{}{}", REDIS_PREFIX, code);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let invite_json: Option<String> = redis_conn.get(&redis_key).await?;

        let invite_json =
            invite_json.ok_or(AppError::Unauthorized("UPGRADE_401_INVALID_CODE".into()))?;

        // [Step 2] 초대 정보 파싱
        let invite_data: InviteData = serde_json::from_str(&invite_json)
            .map_err(|_| AppError::Internal("Failed to parse invite data".into()))?;

        // [Step 3] 만료 시간 계산
        let expires_at = invite_data.created_at + Duration::seconds(INVITE_CODE_TTL_SEC);

        Ok(UpgradeVerifyRes {
            email: invite_data.email,
            role: invite_data.role,
            invited_by: invite_data.invited_by_email,
            expires_at,
        })
    }

    // =========================================================================
    // 7-70: 관리자 계정 생성 (POST /admin/upgrade/accept)
    // =========================================================================

    pub async fn accept_invite(st: &AppState, req: UpgradeAcceptReq) -> AppResult<UpgradeAcceptRes> {
        // [Step 1] Input Validation
        req.validate()
            .map_err(|e| AppError::BadRequest(format!("UPGRADE_400_INVALID_INPUT: {}", e)))?;

        // [Step 2] 비밀번호 정책 검증
        if !Self::validate_password_policy(&req.password) {
            return Err(AppError::Unprocessable(
                "UPGRADE_422_WEAK_PASSWORD: Password must contain letters and numbers".into(),
            ));
        }

        // [Step 3] Redis에서 초대 정보 조회
        let redis_key = format!("{}{}", REDIS_PREFIX, req.code);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let invite_json: Option<String> = redis_conn.get(&redis_key).await?;

        let invite_json =
            invite_json.ok_or(AppError::Unauthorized("UPGRADE_401_INVALID_CODE".into()))?;

        let invite_data: InviteData = serde_json::from_str(&invite_json)
            .map_err(|_| AppError::Internal("Failed to parse invite data".into()))?;

        // [Step 4] 이메일 중복 체크 (레이스 컨디션 방지)
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let email_idx_check = crypto.blind_index(&invite_data.email)?;
        let existing = user_repo::find_user_by_email_idx(&st.db, &email_idx_check).await?;
        if existing.is_some() {
            let _: () = redis_conn.del(&redis_key).await?;
            return Err(AppError::Conflict(
                "UPGRADE_409_EMAIL_ALREADY_EXISTS".into(),
            ));
        }

        // [Step 5] 닉네임 중복 체크
        if let Some(_existing) = user_repo::find_user_by_nickname(&st.db, &req.nickname).await? {
            return Err(AppError::Conflict(
                "UPGRADE_409_NICKNAME_ALREADY_EXISTS".into(),
            ));
        }

        // [Step 6] 비밀번호 해싱
        let password_hash = password::hash_password(&req.password)?;

        // [Step 7] 사용자 생성
        let user_auth = Self::role_to_user_auth(&invite_data.role)?;

        // Field Encryption
        let email_enc = crypto.encrypt(&invite_data.email, "users.user_email")?;
        let email_idx = crypto.blind_index(&invite_data.email)?;
        let name_enc = crypto.encrypt(&req.name, "users.user_name")?;
        let name_idx = crypto.blind_index(&req.name)?;
        let birthday_enc = crypto.encrypt(&req.birthday.to_string(), "users.user_birthday")?;

        let user_id = user_repo::create_admin_user(
            &st.db,
            &email_enc,
            &password_hash,
            &name_enc,
            &req.nickname,
            &req.country,
            &birthday_enc,
            req.gender,
            req.language,
            user_auth.clone(),
            &email_idx,
            &name_idx,
        )
        .await?;

        // [Step 8] 초대 코드 삭제 (일회용)
        let _: () = redis_conn.del(&redis_key).await?;

        info!(
            user_id = user_id,
            email = %invite_data.email,
            role = %invite_data.role,
            invited_by_id = invite_data.invited_by_id,
            "Admin account created via invite"
        );

        Ok(UpgradeAcceptRes {
            user_id,
            email: invite_data.email,
            user_auth,
            message: "Admin account created successfully".into(),
        })
    }
}
