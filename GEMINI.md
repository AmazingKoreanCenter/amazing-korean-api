# Amazing Korean – Backend Context (Axum + SQLx)

> 업데이트: 2025-08-29

## Goal

* Rust + Axum 기반 API 서버를 Gemini CLI로 확장/리팩터링.
* 우선 과제였던 유저/설정/관리자 기능을 일관된 라우팅·문서화 규칙으로 정비.

## Stack

* Rust (Axum 0.8, Tokio, SQLx(PostgreSQL), utoipa/Swagger)
* DB: PostgreSQL (도커 컨테이너: `amk-pg`, 포트 5432)

  * **시간 컬럼 전부 `TIMESTAMPTZ` / UTC**, DB 레벨 `DEFAULT now()`
  * 마이그레이션은 **수정/개명 금지**, 변경은 새 파일 추가 원칙
  * `users_setting`, `users_language_pref`, `user_export_job`의 `updated_at`은 트리거로 자동 갱신
* Redis (컨테이너: `amk-redis`, 포트 6379)

  * 세션/리프레시 토큰 저장: `ak:session:<sid>`, `ak:refresh:<hash> -> <sid>`, `ak:user_sessions:<uid>`
* 실행: `cargo run` (bind: 0.0.0.0:3000), OpenAPI: `/api-docs/openapi.json`, Swagger UI: `/docs`

## Current APIs

1. **회원가입**

   * `POST /users`
   * 이메일/비밀번호/이름/약관 필수, 선택 필드(닉네임/언어/국가/생일/성별)
   * **성공 시 `USERS_LOG` 스냅샷 기록** (비밀번호는 항상 NULL)

2. **내 프로필**

   * `GET /users/me` (JWT 인증)

   * 로그인한 사용자 정보 반환

   * `PUT /users/me` (JWT 인증)

   * 닉네임/언어/국가/생일/성별 업데이트

   * **성공 시 `USERS_LOG` 스냅샷 기록**

3. **사용자 설정**

   * `GET /users/me/settings` (JWT 인증)
   * `PUT /users/me/settings` (JWT 인증, 부분 업데이트 / 학습 언어는 전체 교체)
   * 테이블 명 **`public.users_setting`** 기준으로 통일 완료

4. **관리자 (RBAC: HYMN/admin/manager)**

   * `GET /admin/users`
   * `GET /admin/users/{user_id}`
   * `PUT /admin/users/{user_id}` (수정 시 감사 로그 기록)

5. **개인정보 내보내기 (준비 중)**

   * `POST /users/me/export` → 202 Accepted (비동기 처리 예정)

## Router Architecture

* **User**: `/users`, `/users/me`, `/users/me/settings`
* **Admin (Aggregator)**: `api/admin/router.rs`에서 카테고리별 라우터를 **집약**해 `/admin`에 한 번만 mount

  ```rust
  // src/api/admin/router.rs
  pub fn admin_router() -> Router<AppState> {{
      Router::new()
        .nest("/users", admin_user_router())
        // .nest("/courses", admin_course_router())
        // .nest("/reports", admin_report_router())
  }}
  ```
* 하위 라우터는 **상대 경로**만 사용

  ```rust
  // src/api/admin/user/router.rs
  pub fn admin_user_router() -> Router<AppState> {{
      Router::new()
        .route("/", get(admin_list_users))
        .route("/{user_id}", get(admin_get_user).put(admin_update_user))
  }}
  ```
* 앱 합성:

  ```rust
  // src/api/mod.rs
  .merge(user_router())
  .merge(auth_router())
  .merge(course_router())
  .nest("/admin", admin_router()) // /admin 접두어는 한 번만
  ```

## Error Policy (v2025-08-29)

* 200 OK / 201 Created / 202 Accepted / 204 No Content
* 400 Bad Request (DTO/형식 검증 실패)
* 401 Unauthorized (토큰 없음/무효)
* 403 Forbidden (상태/권한 불가; 예: `user_state!='on'`)
* 404 Not Found
* 409 Conflict (DB 무결성/충돌; 23505 UNIQUE, 23503 FK, 23514 CHECK 등)
* 412 Precondition Failed (ETag/If-Match 사용 시)
* 415/406 (필요시)
* 422 Unprocessable Entity (필요시; 팀 합의 없으면 400 유지)
* 429 Too Many Requests (레이트리밋/쿨다운)
* 500/502/503/504

### Error Body (Global)

```json
{
  "error": {
    "code": "STRING_CONSTANT",
    "http_status": 409,
    "message": "Human readable message.",
    "details": { "field": "email" },
    "trace_id": "req-20250829-abcdef"
  }
}
```

## Files (참조용 앵커)

* @./src/api/user/repo.rs
* @./src/api/user/service.rs
* @./src/api/user/dto.rs
* @./src/api/user/handler.rs
* @./Cargo.toml
* @./.env.example
* @./migrations/\*\_amk\_users.sql (2025-08-29, TIMESTAMPTZ/트리거 반영)

## Endpoints & Naming (구현 현황)

1. **회원가입**

* `POST /users`
* **H/S/R**: `signup` → `signup` → `create_user`
* **DTO**: `SignupReq → SignupRes{ user_id }`
* **Status**: 201, 409, 400, 500
* **Rule**: 이메일 UNIQUE(23505→409), validator, 약관 필수, **성공 시 USERS\_LOG 기록**

2. **내 프로필 조회**

* `GET /users/me` (Bearer)
* **H/S/R**: `get_me` → `get_me` → `find_user`
* **DTO**: `— → ProfileRes`
* **Status**: 200, 401, 403(`user_state!='on'`), 404, 500

3. **내 프로필 수정**

* `PUT /users/me` (Bearer)
* **H/S/R**: `update_me` → `update_me` → `update_user`
* **DTO**: `UpdateReq → ProfileRes`
* **Status**: 200, 400, 401, 403, 404, 500
* **Rule**: **성공 시 USERS\_LOG 기록**

4. **사용자 스냅샷 기록 (USERS\_LOG)**

* 트리거: `POST /users`, `PUT /users/me` 성공 직후
* **전략**: `INSERT … SELECT`로 **DB의 실제 저장값**을 스냅샷 (비밀번호는 항상 NULL)
* **규칙**:

  * `action`: 가입=`"create"`, 수정=`"update"`
  * `updated_by_user_id`: 가입=신규 사용자 ID, 수정=JWT 주체 ID
  * 로깅 실패는 `warn` 후 **본 흐름 유지**

5. **환경 설정(계정/학습/알림)**

* `GET /users/me/settings`, `PUT /users/me/settings` (Bearer)
* **H/S/R**: `get_settings` / `update_users_setting` → 同 → `find_users_setting` / `upsert_settings`
* **DTO**: `SettingsRes`, `SettingsUpdateReq → SettingsRes`
* **동작**:

  * `users_setting`: 제공된 필드만 **부분 업데이트**
  * `users_language_pref`: `study_languages`가 **요청에 포함된 경우**에 한해 **전체 교체(Replace-all)**, priority는 1..N
* **검증**:

  * 언어코드 허용: `en, ko, ne, si, id, vi, th`
  * `is_primary`는 0\~1개, `priority >= 1`

6. **관리자: 목록/조회/수정**

* `GET /admin/users?query=&state=&page=&size=`
* `GET /admin/users/{user_id}`
* `PUT /admin/users/{user_id}`
* **H/S/R**: `admin_list` / `admin_get` / `admin_update`
* **DTO**: `— → AdminListUsersRes`, `— → AdminUserRes`, `AdminUpdateUserReq → AdminUserRes`
* **RBAC**: `user_auth ∈ { HYMN, admin, manager }`만 접근 허용

  * 추가 정책: `manager/admin → HYMN 대상 변경 불가`
* **감사 로그**: `admin_user_action_log(action='admin_update', before, after)` 기록

  * (**TODO**) `actor_user_id`를 **레포 단에서 트랜잭션과 함께 전달**하도록 시그니처 보강

7. **개인정보 내보내기(Export, 비동기)** — 스펙만, 구현 대기

* `POST /users/me/export` → 202 Accepted + `job_id`
* 워커가 ZIP(JSON/CSV) 생성 후 signed URL + 만료 저장

## DTOs (요약 · 구현 기준 반영)

```text
SignupReq {{
  email, password, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  terms_service: bool, terms_personal: bool
}}
SignupRes {{ user_id: i64 }}

ProfileRes {{
  id, email, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  user_state: String, user_auth: String, created_at: DateTime<Utc>
}}

UpdateReq {{
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String
}}

SettingsUpdateReq {{
  ui_language?: String,  # ISO 639-1 (en, ko, ne, si, id, vi, th)
  study_languages?: [ {{ lang_code: String, priority: i32, is_primary: bool }} ],
  timezone?: String,
  notifications_email?: bool,
  notifications_push?: bool
}}

SettingsRes = SettingsUpdateReq + {{ user_id: i64 }}
```

## Swagger

* Bearer 보안 스키마 적용
* 관리자 경로는 문서상 **절대 경로** 사용: `"/admin/users"`, `"/admin/users/{user_id}"`
* 경로 파라미터는 Axum 0.7+ 문법(`{user_id}`)

## Known Gaps / TODO

* `admin_user_action_log`에 **actor\_user\_id 전달**(repo 시그니처 보강)
* USER\_LOG 조회 API(`/users/me/logs`) 및 Swagger 추가
* 공통 RBAC/Audit 헬퍼(`admin/authz.rs`, `admin/audit.rs`) 분리
* 테스트 케이스 정비(통합 테스트 포함)
* Health 엔드포인트 통일(현재 `/healthz` 유지; `/health/live|ready` 전환 검토)
* Redis 운영 스크립트: 강제 로그아웃/세션 전부 삭제/토큰 회전 검증 커맨드 정리 (로컬 명령 세트는 별도 README에 첨부)