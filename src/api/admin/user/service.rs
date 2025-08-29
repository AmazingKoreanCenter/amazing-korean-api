use crate::{
    error::{AppError, AppResult},
    state::AppState,
    types::{UserAuth as GlobalUserAuth, UserState as GlobalUserState},
};
use sqlx::PgPool;

use super::{
    dto::{AdminListUsersRes, AdminUpdateUserReq, AdminUserRes},
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

    // RBAC check helper
    async fn check_admin_rbac(pool: &PgPool, actor_user_id: i64) -> AppResult<GlobalUserAuth> {
        let actor = crate::api::user::repo::find_by_id(pool, actor_user_id)
            .await?
            .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

        let actor_auth: GlobalUserAuth = actor.user_auth;

        match actor_auth {
            GlobalUserAuth::Hymn | GlobalUserAuth::Admin | GlobalUserAuth::Manager => {
                Ok(actor_auth)
            }
            _ => Err(AppError::Forbidden), // 403
        }
    }

    // Target user RBAC check helper for update operations
    async fn check_target_user_rbac(
        actor_auth: GlobalUserAuth,
        _target_user_id: i64,
        target_user_auth: GlobalUserAuth,
    ) -> AppResult<()> {
        if actor_auth == GlobalUserAuth::Manager && target_user_auth == GlobalUserAuth::Hymn {
            return Err(AppError::Forbidden);
        }
        if actor_auth == GlobalUserAuth::Admin && target_user_auth == GlobalUserAuth::Hymn {
            return Err(AppError::Forbidden);
        }
        // Add more specific rules if needed, e.g., preventing self-demotion
        Ok(())
    }

    pub async fn admin_list(
        st: &AppState,
        actor_user_id: i64,
        query: Option<String>,
        state: Option<GlobalUserState>,
        page: Option<i64>,
        size: Option<i64>,
    ) -> AppResult<AdminListUsersRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let page = page.unwrap_or(1).max(1);
        let size = size.unwrap_or(20).clamp(1, 100);

        let (total, items) =
            repo::admin_list_users(&st.db, query.as_deref(), state, page, size).await?;

        Ok(AdminListUsersRes { total, items })
    }

    pub async fn admin_get(
        st: &AppState,
        actor_user_id: i64,
        target_user_id: i64,
    ) -> AppResult<AdminUserRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let user = repo::admin_get_user(&st.db, target_user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn admin_update(
        st: &AppState,
        actor_user_id: i64,
        target_user_id: i64,
        mut req: AdminUpdateUserReq,
    ) -> AppResult<AdminUserRes> {
        let actor_auth = Self::check_admin_rbac(&st.db, actor_user_id).await?;

        // Fetch current target user for RBAC and before snapshot
        let current_target_user = repo::admin_get_user(&st.db, target_user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let target_user_auth: GlobalUserAuth = current_target_user.user_auth;

        Self::check_target_user_rbac(actor_auth, target_user_id, target_user_auth).await?;

        // Normalize email if provided
        if let Some(email) = &mut req.email {
            *email = email.trim().to_lowercase();
        }

        let res = repo::admin_update_user(&st.db, actor_user_id, target_user_id, &req).await;

        match res {
            Ok(user) => {
                // Optionally log user_state changes to user_log
                if let Some(new_state) = &req.user_state {
                    if new_state != &current_target_user.user_state {
                        if let Err(le) = crate::api::user::repo::insert_user_log_after(
                            &st.db,
                            "admin_state_change",
                            Some(actor_user_id),
                            &crate::api::user::dto::ProfileRes {
                                id: user.id,
                                email: user.email.clone(),
                                name: user.name.clone(),
                                nickname: user.nickname.clone(),
                                language: user.language.clone(),
                                country: user.country.clone(),
                                birthday: user.birthday,
                                gender: user.gender,
                                user_state: user.user_state,
                                user_auth: user.user_auth,
                                created_at: user.created_at,
                            },
                        )
                        .await
                        {
                            warn!(error=?le, actor_user_id = actor_user_id, target_user_id = target_user_id, "user_log(admin_state_change) insert failed");
                        }
                    }
                }
                Ok(user)
            }
            Err(e) if Self::is_unique_violation(&e) => {
                Err(AppError::BadRequest("Email already exists".into())) // 400
            }
            Err(e) => Err(e),
        }
    }
}
