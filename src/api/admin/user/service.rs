use chrono::NaiveDate;
use std::net::IpAddr;

use validator::Validate;

use crate::{
    api::auth::password,
    error::{AppError, AppResult},
    state::AppState,
    types::{UserAuth as GlobalUserAuth, UserGender},
};

use super::{
    dto::{
        AdminCreateUserReq, AdminUpdateUserReq, AdminUserListMeta, AdminUserListReq,
        AdminUserListRes, AdminUserRes,
    },
    repo,
};

// 로깅 실패 무시용
use tracing::warn;

pub struct AdminUserService;

impl AdminUserService {
    const PG_UNIQUE_VIOLATION: &'static str = "23505";

    #[inline]
    fn is_unique_violation(err: &AppError) -> bool {
        if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
            db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
        } else {
            false
        }
    }

    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }

    fn parse_user_auth(raw: Option<&str>) -> AppResult<GlobalUserAuth> {
        let key = raw.unwrap_or("learner").to_lowercase();
        match key.as_str() {
            "user" | "learner" => Ok(GlobalUserAuth::Learner),
            "admin" => Ok(GlobalUserAuth::Admin),
            "manager" => Ok(GlobalUserAuth::Manager),
            "hymn" | "HYMN" => Ok(GlobalUserAuth::Hymn),
            _ => Err(AppError::Unprocessable("invalid user_auth".into())),
        }
    }

    async fn check_admin_rbac(
        pool: &sqlx::PgPool,
        actor_user_id: i64,
    ) -> AppResult<GlobalUserAuth> {
        let actor = crate::api::user::repo::find_user(pool, actor_user_id)
            .await?
            .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

        let actor_auth: GlobalUserAuth = actor.user_auth;

        match actor_auth {
            GlobalUserAuth::Hymn | GlobalUserAuth::Admin | GlobalUserAuth::Manager => {
                Ok(actor_auth)
            }
            _ => Err(AppError::Forbidden),
        }
    }

    async fn check_target_user_rbac(
        actor_auth: GlobalUserAuth,
        target_user_auth: GlobalUserAuth,
    ) -> AppResult<()> {
        if actor_auth == GlobalUserAuth::Manager && target_user_auth == GlobalUserAuth::Hymn {
            return Err(AppError::Forbidden);
        }
        if actor_auth == GlobalUserAuth::Admin && target_user_auth == GlobalUserAuth::Hymn {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }

    pub async fn admin_list_users(
        st: &AppState,
        actor_user_id: i64,
        req: AdminUserListReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<AdminUserListRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let page = req.page.unwrap_or(1);
        if page < 1 {
            return Err(AppError::BadRequest("page must be >= 1".into()));
        }

        let size = req.size.unwrap_or(20);
        if size < 1 {
            return Err(AppError::BadRequest("size must be >= 1".into()));
        }
        if size > 100 {
            return Err(AppError::Unprocessable("size exceeds 100".into()));
        }

        let sort = req.sort.as_deref().unwrap_or("created_at");
        if !matches!(sort, "created_at" | "email" | "nickname") {
            return Err(AppError::Unprocessable("invalid sort".into()));
        }

        let order = req.order.as_deref().unwrap_or("desc");
        if !matches!(order, "asc" | "desc") {
            return Err(AppError::Unprocessable("invalid order".into()));
        }

        let details = serde_json::json!({
            "q": req.q.as_deref(),
            "page": page,
            "size": size,
            "sort": sort,
            "order": order
        });

        repo::create_audit_log(
            &st.db,
            actor_user_id,
            "LIST_USERS",
            Some("users"),
            None,
            &details,
            ip_address,
            user_agent.as_deref(),
        )
        .await?;

        let (total_count, items) =
            repo::admin_list_users(&st.db, req.q.as_deref(), page, size, sort, order).await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + size - 1) / size
        };

        Ok(AdminUserListRes {
            items,
            meta: AdminUserListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }

    pub async fn admin_get_user(
        st: &AppState,
        actor_user_id: i64,
        user_id: i64,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<AdminUserRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let details = serde_json::json!({
            "target_user_id": user_id
        });

        repo::create_audit_log(
            &st.db,
            actor_user_id,
            "GET_USER",
            Some("users"),
            Some(user_id),
            &details,
            ip_address,
            user_agent.as_deref(),
        )
        .await?;

        let user = repo::admin_get_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn admin_create_user(
        st: &AppState,
        actor_user_id: i64,
        mut req: AdminCreateUserReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<AdminUserRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        req.email = req.email.trim().to_lowercase();

        if repo::exists_email(&st.db, &req.email).await? {
            return Err(AppError::Conflict("email already exists".into()));
        }

        if !Self::validate_password_policy(&req.password) {
            return Err(AppError::Unprocessable(
                "password policy violation".into(),
            ));
        }

        let user_auth = Self::parse_user_auth(req.user_auth.as_deref())?;
        let user_auth_str = user_auth.to_string();

        let password_hash = password::hash(&req.password)?;

        let language = "ko";
        let country = "ko";
        let birthday = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let gender = UserGender::None;

        let res = repo::admin_create_user(
            &st.db,
            &req.email,
            &password_hash,
            &req.name,
            &req.nickname,
            &user_auth_str,
            language,
            country,
            birthday,
            gender,
            false,
            false,
            actor_user_id,
            ip_address,
            user_agent.as_deref(),
        )
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(e) if Self::is_unique_violation(&e) => {
                Err(AppError::Conflict("email already exists".into()))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn admin_update_user(
        st: &AppState,
        actor_user_id: i64,
        user_id: i64,
        mut req: AdminUpdateUserReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<AdminUserRes> {
        let actor_auth = Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let current_target_user = repo::admin_get_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let target_user_auth: GlobalUserAuth = current_target_user.user_auth;

        Self::check_target_user_rbac(actor_auth, target_user_auth).await?;

        if let Some(email) = &mut req.email {
            *email = email.trim().to_lowercase();
        }

        let res = repo::admin_update_user(
            &st.db,
            actor_user_id,
            user_id,
            &req,
            ip_address,
            user_agent.as_deref(),
        )
        .await;

        match res {
            Ok(user) => {
                if let Some(new_state) = &req.user_state {
                    if new_state != &current_target_user.user_state {
                        if let Err(le) = crate::api::user::repo::insert_user_log_after(
                            &st.db,
                            Some(actor_user_id),
                            user.id,
                            "update",
                            true,
                        )
                        .await
                        {
                            warn!(error=?le, actor_user_id = actor_user_id, target_user_id = user_id, "public.users_log(admin_state_change) insert failed");
                        }
                    }
                }
                Ok(user)
            }
            Err(e) if Self::is_unique_violation(&e) => {
                Err(AppError::BadRequest("Email already exists".into()))
            }
            Err(e) => Err(e),
        }
    }
}
