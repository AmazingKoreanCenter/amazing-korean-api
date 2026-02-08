use chrono::NaiveDate;
use std::net::IpAddr;

use validator::Validate;

use crate::{
    api::auth::password,
    crypto::CryptoService,
    error::{AppError, AppResult},
    state::AppState,
    types::{UserAuth as GlobalUserAuth, UserGender},
};

use super::{
    dto::{
        AdminBulkCreateReq, AdminBulkCreateRes, AdminBulkUpdateReq,
        AdminBulkUpdateRes, AdminCreateUserReq, AdminUpdateUserReq, AdminUserListMeta,
        AdminUserListReq, AdminUserListRes, AdminUserRes, BulkItemError, BulkItemResult,
        BulkSummary, BulkUpdateItemResult,
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
            _ => Err(AppError::Forbidden("Forbidden".to_string())),
        }
    }

    async fn check_target_user_rbac(
        actor_auth: GlobalUserAuth,
        target_user_auth: GlobalUserAuth,
    ) -> AppResult<()> {
        if actor_auth == GlobalUserAuth::Manager
            && (target_user_auth == GlobalUserAuth::Admin
                || target_user_auth == GlobalUserAuth::Hymn)
        {
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }
        if actor_auth == GlobalUserAuth::Admin && target_user_auth == GlobalUserAuth::Hymn {
            return Err(AppError::Forbidden("Forbidden".to_string()));
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
        if !matches!(sort, "id" | "created_at" | "email" | "nickname" | "role") {
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

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        repo::create_audit_log(
            &st.db,
            actor_user_id,
            "LIST_USERS",
            Some("users"),
            None,
            &details,
            ip_enc.as_deref(),
            user_agent.as_deref(),
        )
        .await?;
        let email_idx = if let Some(ref keyword) = req.q {
            if keyword.contains('@') {
                Some(crypto.blind_index(keyword)?)
            } else {
                None
            }
        } else {
            None
        };

        let (total_count, mut items) =
            repo::admin_list_users(&st.db, req.q.as_deref(), email_idx.as_deref(), page, size, sort, order).await?;

        // 이메일 복호화
        for item in &mut items {
            item.email = crypto.decrypt(&item.email, "users.user_email")?;
        }

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

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        repo::create_audit_log(
            &st.db,
            actor_user_id,
            "GET_USER",
            Some("users"),
            Some(user_id),
            &details,
            ip_enc.as_deref(),
            user_agent.as_deref(),
        )
        .await?;

        let mut user = repo::admin_get_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Decrypt encrypted fields
        user.email = crypto.decrypt(&user.email, "users.user_email")?;
        user.name = crypto.decrypt(&user.name, "users.user_name")?;
        if let Some(ref bday) = user.birthday {
            user.birthday = Some(crypto.decrypt(bday, "users.user_birthday")?);
        }

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

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let birthday = req.birthday.unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        let email_enc = crypto.encrypt(&req.email, "users.user_email")?;
        let email_idx = crypto.blind_index(&req.email)?;
        let name_enc = crypto.encrypt(&req.name, "users.user_name")?;
        let name_idx = crypto.blind_index(&req.name)?;
        let birthday_enc = crypto.encrypt(&birthday.to_string(), "users.user_birthday")?;

        // 이메일 중복 체크
        if repo::exists_email_idx(&st.db, &email_idx).await? {
            return Err(AppError::Conflict("email already exists".into()));
        }

        if !Self::validate_password_policy(&req.password) {
            return Err(AppError::Unprocessable(
                "password policy violation".into(),
            ));
        }

        let user_auth = Self::parse_user_auth(req.user_auth.as_deref())?;
        let user_auth_str = user_auth.to_string();

        let password_hash = password::hash_password(&req.password)?;

        // DTO에서 받은 값 또는 기본값 사용
        let language = req.language.as_deref().unwrap_or("ko");
        let country = req.country.as_deref().unwrap_or("KR");
        let gender = req.gender.unwrap_or(UserGender::None);

        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        let res = repo::admin_create_user(
            &st.db,
            &email_enc,
            &password_hash,
            &name_enc,
            &req.nickname,
            &user_auth_str,
            language,
            country,
            &birthday_enc,
            gender,
            false,
            false,
            actor_user_id,
            ip_enc.as_deref(),
            user_agent.as_deref(),
            true,
            &email_idx,
            &name_idx,
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

    pub async fn admin_create_users_bulk(
        st: &AppState,
        actor_user_id: i64,
        req: AdminBulkCreateReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<(bool, AdminBulkCreateRes)> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        let mut results = Vec::with_capacity(req.items.len());
        let mut success = 0i64;
        let mut failure = 0i64;

        for item in req.items {
            let email = item.email.trim().to_lowercase();
            let mut item = item;
            item.email = email.clone();

            let outcome = Self::admin_create_user_single(
                st,
                actor_user_id,
                item,
                ip_address,
                user_agent.as_deref(),
            )
            .await;

            match outcome {
                Ok(user) => {
                    success += 1;
                    results.push(BulkItemResult {
                        email,
                        status: 201,
                        data: Some(user),
                        error: None,
                    });
                }
                Err((status, code, message)) => {
                    failure += 1;
                    results.push(BulkItemResult {
                        email,
                        status,
                        data: None,
                        error: Some(BulkItemError { code, message }),
                    });
                }
            }
        }

        let summary = BulkSummary {
            total: success + failure,
            success,
            failure,
        };

        let details = serde_json::json!({
            "total": summary.total,
            "success": summary.success,
            "failure": summary.failure
        });

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        repo::create_audit_log(
            &st.db,
            actor_user_id,
            "BULK_CREATE_USERS",
            Some("users"),
            None,
            &details,
            ip_enc.as_deref(),
            user_agent.as_deref(),
        )
        .await?;

        let all_success = failure == 0;

        Ok((
            all_success,
            AdminBulkCreateRes {
                summary,
                results,
            },
        ))
    }

    async fn admin_create_user_single(
        st: &AppState,
        actor_user_id: i64,
        mut req: AdminCreateUserReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<&str>,
    ) -> Result<AdminUserRes, (i32, String, String)> {
        if let Err(e) = req.validate() {
            return Err((400, "BAD_REQUEST".to_string(), e.to_string()));
        }

        req.email = req.email.trim().to_lowercase();

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let birthday = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let email_enc = crypto.encrypt(&req.email, "users.user_email")
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".into(), e.to_string()))?;
        let email_idx = crypto.blind_index(&req.email)
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".into(), e.to_string()))?;
        let name_enc = crypto.encrypt(&req.name, "users.user_name")
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".into(), e.to_string()))?;
        let name_idx = crypto.blind_index(&req.name)
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".into(), e.to_string()))?;
        let birthday_enc = crypto.encrypt(&birthday.to_string(), "users.user_birthday")
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".into(), e.to_string()))?;

        // 이메일 중복 체크
        let idx_exists = match repo::exists_email_idx(&st.db, &email_idx).await {
            Ok(v) => v,
            Err(e) => return Err((500, "INTERNAL_SERVER_ERROR".to_string(), e.to_string())),
        };
        if idx_exists {
            return Err((409, "CONFLICT".to_string(), "email already exists".to_string()));
        }

        if !Self::validate_password_policy(&req.password) {
            return Err((
                422,
                "UNPROCESSABLE_ENTITY".to_string(),
                "password policy violation".to_string(),
            ));
        }

        let user_auth = match Self::parse_user_auth(req.user_auth.as_deref()) {
            Ok(val) => val,
            Err(_) => {
                return Err((
                    422,
                    "UNPROCESSABLE_ENTITY".to_string(),
                    "invalid user_auth".to_string(),
                ))
            }
        };
        let user_auth_str = user_auth.to_string();

        let password_hash = match password::hash_password(&req.password) {
            Ok(hash) => hash,
            Err(e) => {
                return Err((500, "INTERNAL_SERVER_ERROR".to_string(), e.to_string()));
            }
        };

        let language = "ko";
        let country = "KR";
        let gender = UserGender::None;

        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()
            .map_err(|e| (500, "INTERNAL_SERVER_ERROR".to_string(), e.to_string()))?;

        match repo::admin_create_user(
            &st.db,
            &email_enc,
            &password_hash,
            &name_enc,
            &req.nickname,
            &user_auth_str,
            language,
            country,
            &birthday_enc,
            gender,
            false,
            false,
            actor_user_id,
            ip_enc.as_deref(),
            user_agent,
            false,
            &email_idx,
            &name_idx,
        )
        .await
        {
            Ok(user) => Ok(user),
            Err(e) if Self::is_unique_violation(&e) => Err((
                409,
                "CONFLICT".to_string(),
                "email already exists".to_string(),
            )),
            Err(e) => Err((500, "INTERNAL_SERVER_ERROR".to_string(), e.to_string())),
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

        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);

        if let Some(email) = &mut req.email {
            *email = email.trim().to_lowercase();
            let current_email = crypto.decrypt(&current_target_user.email, "users.user_email")?;
            if email.to_lowercase() != current_email.to_lowercase() {
                let check_idx = crypto.blind_index(email)?;
                if repo::exists_email_idx(&st.db, &check_idx).await? {
                    return Err(AppError::Conflict("email already exists".into()));
                }
            }
        }

        let password_hash = if let Some(password) = req.password.as_deref() {
            if !Self::validate_password_policy(password) {
                return Err(AppError::Unprocessable(
                    "password policy violation".into(),
                ));
            }
            Some(password::hash_password(password)?)
        } else {
            None
        };

        let (email_enc, email_idx) = if let Some(ref email) = req.email {
            (Some(crypto.encrypt(email, "users.user_email")?), Some(crypto.blind_index(email)?))
        } else { (None, None) };
        let (name_enc, name_idx) = if let Some(ref name) = req.name {
            (Some(crypto.encrypt(name, "users.user_name")?), Some(crypto.blind_index(name)?))
        } else { (None, None) };
        let birthday_enc = req.birthday
            .map(|b| crypto.encrypt(&b.to_string(), "users.user_birthday"))
            .transpose()?;

        let details = serde_json::json!({
            "target_user_id": user_id
        });

        let mut tx = st.db.begin().await?;

        let updated = repo::admin_update_user(
            &mut tx,
            user_id,
            &req,
            password_hash.as_deref(),
            email_enc.as_deref(),
            email_idx.as_deref(),
            name_enc.as_deref(),
            name_idx.as_deref(),
            birthday_enc.as_deref(),
        )
        .await?;

        let before_val = serde_json::to_value(&current_target_user).unwrap_or_default();
        let after_val = serde_json::to_value(&updated).unwrap_or_default();

        repo::create_history_log(
            &mut tx,
            actor_user_id,
            updated.id,
            "update",
            Some(&before_val),
            Some(&after_val),
        )
        .await?;

        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        repo::create_audit_log_tx(
            &mut tx,
            actor_user_id,
            "UPDATE_USER",
            Some("users"),
            Some(updated.id),
            &details,
            ip_enc.as_deref(),
            user_agent.as_deref(),
        )
        .await?;

        tx.commit().await?;

        if let Some(new_state) = req.user_state {
            if new_state != current_target_user.user_state {
                if let Err(le) = crate::api::user::repo::insert_user_log_after(
                    &st.db,
                    &crypto,
                    Some(actor_user_id),
                    updated.id,
                    "update",
                    true,
                )
                .await
                {
                    warn!(error=?le, actor_user_id = actor_user_id, target_user_id = user_id, "public.users_log(admin_state_change) insert failed");
                }
            }
        }

        Ok(updated)
    }

    pub async fn admin_update_users_bulk(
        st: &AppState,
        actor_user_id: i64,
        req: AdminBulkUpdateReq,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> AppResult<(bool, AdminBulkUpdateRes)> {
        
        // 1. 요청자(Actor) 권한 확인
        let actor = super::repo::admin_get_user(&st.db, actor_user_id)
            .await?
            .ok_or(AppError::NotFound)?; 
    
        match actor.user_auth.to_string().as_str() {
            "admin" | "hymn" | "manager" => {} 
            _ => return Err(AppError::Forbidden("Forbidden".to_string())), 
        }
    
        let mut results = Vec::new();
        let mut success_count = 0i64;
        let mut failure_count = 0i64;
    
        // 2. 루프 시작
        for item in req.items {
            let process_result = async {
                // 2-1. 읽기 작업: Pool(&st.db) 사용
                // 트랜잭션 시작 전, 가벼운 조회는 Pool로 처리
                let target_user = super::repo::admin_get_user(&st.db, item.id)
                    .await?
                    .ok_or(AppError::NotFound)?;
    
                // RBAC 체크
                let actor_role = actor.user_auth.to_string();
                let target_role = target_user.user_auth.to_string();
                if actor_role == "manager" && (target_role == "admin" || target_role == "hymn") {
                     return Err(AppError::Forbidden("Forbidden".to_string()));
                }
    
                let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
                let (email_enc, email_idx) = if let Some(ref email) = item.email {
                    let enc = crypto.encrypt(email, "users.user_email")?;
                    let idx = crypto.blind_index(email)?;
                    (Some(enc), Some(idx))
                } else { (None, None) };
                let (name_enc, name_idx) = if let Some(ref name) = item.name {
                    let enc = crypto.encrypt(name, "users.user_name")?;
                    let idx = crypto.blind_index(name)?;
                    (Some(enc), Some(idx))
                } else { (None, None) };
                let birthday_enc = if let Some(birthday) = item.birthday {
                    Some(crypto.encrypt(&birthday.to_string(), "users.user_birthday")?)
                } else { None };

                if let Some(new_email) = &item.email {
                    let new_email_lower = new_email.trim().to_lowercase();
                    let current_email = crypto.decrypt(&target_user.email, "users.user_email")?;
                    if new_email_lower != current_email.to_lowercase() {
                        if let Some(ref idx) = email_idx {
                            if super::repo::exists_email_idx(&st.db, idx).await? {
                                return Err(AppError::Conflict("Email exists".into()));
                            }
                        }
                    }
                }

                // [수정] 2-2. 비밀번호 해싱
                let password_hash = if let Some(pw) = &item.password {
                    Some(crate::api::auth::password::hash_password(pw)?)
                } else {
                    None
                };

                // 2-3. DTO 생성
                // [수정] password는 None으로 설정 (해시값은 별도 인자로 전달하므로 중복 방지)
                let update_req = AdminUpdateUserReq {
                    email: item.email.clone(),
                    password: None, // 여기엔 넣지 않습니다.
                    nickname: item.nickname.clone(),
                    name: item.name.clone(),
                    language: item.language.clone(),
                    country: item.country.clone(),
                    birthday: item.birthday,
                    gender: item.gender.clone(),
                    user_state: item.user_state,
                    user_auth: item.user_auth.clone(),
                };

                // 2-4. 쓰기 작업: Transaction 시작
                let mut tx = st.db.begin().await?;

                let updated_user = super::repo::admin_update_user(
                    &mut tx,
                    item.id,
                    &update_req,
                    password_hash.as_deref(),
                    email_enc.as_deref(),
                    email_idx.as_deref(),
                    name_enc.as_deref(),
                    name_idx.as_deref(),
                    birthday_enc.as_deref(),
                ).await?;
    
                // 2-5. 커밋
                tx.commit().await?;
                
                Ok(updated_user)
            }.await;
    
            match process_result {
                Ok(user) => {
                    success_count += 1;
                    results.push(BulkUpdateItemResult {
                        id: user.id,
                        status: 200,
                        data: Some(user),
                        error: None,
                    });
                }
                Err(e) => {
                    failure_count += 1;
                    let (status, msg) = match e {
                        AppError::NotFound => (404, "User not found".to_string()),
                        AppError::Forbidden(_) => (403, "Forbidden".to_string()),
                        AppError::Conflict(m) => (409, m),
                        AppError::BadRequest(m) => (400, m),
                        _ => (500, "Internal Server Error".to_string()),
                    };
                    
                    results.push(BulkUpdateItemResult {
                        id: item.id,
                        status,
                        data: None,
                        error: Some(BulkItemError { code: "ERR".into(), message: msg }),
                    });
                }
            }
        }
    
        // 4. Audit Log
        let summary = BulkSummary {
            total: (success_count + failure_count) as i64,
            success: success_count,
            failure: failure_count,
        };
        
        let details = serde_json::json!({
            "total": summary.total,
            "success": summary.success,
            "failure": summary.failure
        });
    
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let ip_enc = ip_address
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        super::repo::create_audit_log(
            &st.db,
            actor_user_id,
            "BULK_UPDATE_USERS",
            Some("users"),
            None,
            &details,
            ip_enc.as_deref(),
            user_agent.as_deref(),
        ).await?;
    
        let all_success = failure_count == 0;

        Ok((all_success, AdminBulkUpdateRes {
            summary,
            results,
        }))
    }

    // ==========================================
    // User Logs API
    // ==========================================

    /// 관리자가 변경한 유저 로그 조회
    pub async fn admin_get_user_logs(
        st: &AppState,
        _admin_sub: i64,
        target_user_id: i64,
        page: Option<i64>,
        size: Option<i64>,
    ) -> AppResult<super::dto::AdminUserLogsRes> {
        let page = page.unwrap_or(1).max(1);
        let size = size.unwrap_or(20).clamp(1, 100);

        let (mut items, total_count) =
            repo::admin_get_user_logs(&st.db, target_user_id, page, size).await?;

        // Decrypt admin_email
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        for item in &mut items {
            if let Some(ref email) = item.admin_email {
                item.admin_email = Some(crypto.decrypt(email, "users.user_email")?);
            }
        }

        let total_pages = (total_count as f64 / size as f64).ceil() as i64;

        Ok(super::dto::AdminUserLogsRes {
            items,
            meta: AdminUserListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }

    /// 유저 본인이 변경한 로그 조회
    pub async fn admin_get_user_self_logs(
        st: &AppState,
        _admin_sub: i64,
        user_id: i64,
        page: Option<i64>,
        size: Option<i64>,
    ) -> AppResult<super::dto::UserLogsRes> {
        let page = page.unwrap_or(1).max(1);
        let size = size.unwrap_or(20).clamp(1, 100);

        let (items, total_count) =
            repo::admin_get_user_self_logs(&st.db, user_id, page, size).await?;

        let total_pages = (total_count as f64 / size as f64).ceil() as i64;

        Ok(super::dto::UserLogsRes {
            items,
            meta: AdminUserListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }
}
