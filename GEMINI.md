# Amazing Korean – Backend Context (Axum + SQLx)

## Goal
- Rust + Axum 기반 API 서버의 기능 확장과 리팩터링을 Gemini CLI와 함께 수행.
- 우선 과제: `GET /users/me`, `PUT /users/me` 구현(+ JWT), Swagger(/docs) 정합성.

## Stack
- Rust (Axum 0.8, Tokio, SQLx(PostgreSQL), utoipa/Swagger)
- DB: PostgreSQL (도커 컨테이너: `amk-pg`, 포트 5432)
  * 적용된 마이그레이션 수정/개명 금지, 변경은 새 마이그레이션으로 *
- 실행: `cargo run` (bind: 0.0.0.0:3000), OpenAPI: `/api-docs/openapi.json`, Swagger UI: `/docs`

## Current APIs
- `POST /users` (회원가입)
- `GET /users/me` (JWT 인증)
- `PUT /users/me` (JWT 인증, 닉네임/언어/국가/생일/성별 업데이트)

### Health Endpoints
- `GET /health/live` : liveness (초경량, DB 미접속), 200 OK만 응답
- `GET /health/ready`: readiness (DB 등 핵심 의존성 체크; 실패시 503)
- 기존 `GET /healthz`는 `/health/live` alias로 유지

## Conventions
- 인증: JWT HS256 (`JWT_SECRET`, `JWT_EXPIRE_HOURS`) – **미구현/이번 태스크에서 처리**
- 사용자: `users.email` UNIQUE 위반(23505) → 409
- 문서화: utoipa v5, 보안 스키마는 Bearer

### Error Policy (v2025-08-20)
- 200 OK / 201 Created / 202 Accepted / 204 No Content
- 400 Bad Request (DTO/형식 검증 실패)
- 401 Unauthorized (토큰 없음/무효)
- 403 Forbidden (상태/권한 불가; 예: `user_state!='on'`)
- 404 Not Found
- 409 Conflict (DB 무결성/충돌; 23505 UNIQUE, 23503 FK, 23514 CHECK 등)
- 412 Precondition Failed (ETag/If-Match 사용 시)
- 415/406 (필요시)
- 422 Unprocessable Entity (필요시; 팀 합의 없으면 400 유지)
- 429 Too Many Requests (레이트리밋/쿨다운)
- 500/502/503/504

### Error Body (Global)
```json
{
  "error": {
    "code": "STRING_CONSTANT",          // 예: USER_EMAIL_TAKEN, AUTH_INVALID_TOKEN
    "http_status": 409,
    "message": "Human readable message.",
    "details": { "field": "email" },    // 선택: 필드/규칙/SQLSTATE 등
    "trace_id": "req-20250820-abcdef"   // 로그/분산추적 연계
  }
}
```

## Files (참조용 앵커)
- @./src/api/user/repo.rs
- @./src/api/user/service.rs
- @./src/api/user/dto.rs
- @./src/api/user/handler.rs
- @./Cargo.toml
- @./.env.example

## Tasks for Gemini
1) `/users/me`(GET): Bearer 토큰 파싱 → user_state=="on"만 200(프로필), 아니면 401/403.
2) `/users/me`(PUT): 닉네임/언어/국가/생일/성별 업데이트. DTO-검증-서비스-리포지토리 계층 분리.
3) Swagger(/docs) 스펙 반영 및 예제 응답 추가.
4) 단위 테스트(서비스/리포) & 핸들러 통합 테스트.

## Done = Definition
- 핸들러/서비스/리포/DTO 구현 + 테스트 통과
- OpenAPI 스펙에 엔드포인트/스키마 반영
- 문서: 변경사항 요약, 마이그레이션/ENV 안내 업데이트

## User Module – Spec (v2025-08-20)

### Boundary (auth, user)
- **auth**: 로그인/로그아웃, 비번 해시·검증, JWT 발급·검증, 세션/블랙리스트, 비번 재설정, 마지막 로그인 기록.
- **user**: 회원가입, 프로필 조회/수정, 동의/정책 이력, 환경설정(다국어/알림), 계정 비활성화/삭제, 변경 로그, (관리자) 사용자 관리, 개인정보 내보내기.

### Conventions
- **전 계층 동일 어근**(예: `signup`) + **함수 = snake_case**, **타입 = PascalCase**.
- 에러 매핑: 201/200, 400(검증), 401(무토큰/무효), 403(`user_state!='on'`), 404, 409(UNIQUE/23505), 500.
- 이메일은 **소문자 정규화** 후 저장. 비번은 **Argon2 해시**(`auth` 유틸 사용).
- 상태: `user_state in ('on','off')` (확장: `'blocked','deleted','pending_email'` 등 필요 시 추가).
- 문서화: utoipa(Req/Res ToSchema, #[utoipa::path]) + 예제 응답.
- Note: ProfileRes.language는 계정 기본 언어(프로필)이며, UI 언어 및 학습 선호 언어(복수/우선순위)는 /users/me/settings에서 관리한다.

---

## Endpoints & Naming (우선순위 포함)

1) **회원가입**
- `POST /users`
- **H/S/R**: `signup` → `signup` → `create_user` (또는 `repo::signup`)
- **DTO**: `SignupReq → SignupRes{ user_id }`
- **Status**: 201, 409, 400, 500
- **Rule**: 이메일 UNIQUE(23505→409), validator, 약관 필수, (이메일 인증 도입 시) 완료 후 `user_state='on'`, 성공 시 `USER_LOG` 스냅샷 기록.

2) **내 프로필 조회**
- `GET /users/me`  (Bearer)
- **H/S/R**: `get_me` → `get_me` → `find_by_id`
- **DTO**: `— → ProfileRes`
- **Status**: 200, 401, 403(`user_state!='on'`), 404, 500

3) **내 프로필 수정**
- `PUT /users/me`  (Bearer)
- **H/S/R**: `update_me` → `update_me` → `update_profile`
- **DTO**: `UpdateReq → ProfileRes`
- **Status**: 200, 400, 401, 403, 404, 500
- **Rule**: 성공 시 `USER_LOG` 스냅샷 기록.

4) **사용자 스냅샷 기록**
- `POST /users`, `PUT /user/me`
- **H/S/R**
    - 가입: `signup` → `signup`(성공 후 insert_user_log("create")) → `create_user`, `insert_user_log`
    - 수정: `update_me` → `update_me`(성공 후 insert_user_log("update")) → `update_profile`, `insert_user_log`
- **DTO** : (내부 후처리); 스냅샷 소스 = ProfileRes(After)
- **Status**
    - 가입: 201/400/409/500
    - 수정: 200/400/401/403/404/500, 로깅 실패 시 상태 변화 없음
-  **Rule** 
    - action: 가입 시 "create", 수정 시 "update" 로 기록
    - updated_by_user_id: 가입은 신규 사용자 ID, 수정은 JWT 주체 ID
    - user_password_log: 항상 NULL(민감정보 미기록)
    - 로깅 에러는 잡아서 warn 로그만 남기고 요청 처리는 계속 진행

5) **환경 설정(계정/학습/알림)**
- `GET /users/me/settings`, `PUT /users/me/settings`
- **H/S/R**: `update_settings` → `update_settings` → `update_settings`
- **DTO**: `SettingsRes`, `SettingsUpdateReq → SettingsRes`
- **Status**: 200/204, 400, 401, 403
- **Note**: 7개국어 지원. UI 언어(단일) + 학습 선호 언어(복수, 우선순위).

6) **관리자: 목록/조회/수정**
- `GET /admin/users?query=&state=&page=&size=`
- `GET /admin/users/{user_id}`
- `PUT /admin/users/{user_id}`
- **H/S/R**: `admin_list` / `admin_get` / `admin_update` (service/repo 동일 어근)
- **DTO**: `— → AdminListUsersRes`, `— → AdminUserRes`, `AdminUpdateUserReq → AdminUserRes`
- **Status**: 200, 400, 403(권한부족), 404
- **Rule**: RBAC(`user_auth in ('HYMN','admin','manager')`), 감사 로그 남김.

7) **개인정보 내보내기(Export, 비동기)**
- `POST /users/me/export`
- **H/S/R**: `export` → `export` → `create_export_job`
- **DTO**: `ExportReq? → ExportAcceptedRes{ job_id }`
- **Status**: 202(권장) 또는 200
- **Flow**: Job 등록 → 워커가 ZIP(JSON/CSV) 생성 → 서명 URL + 만료시간 저장/반환.

---

## DTOs (요약)
```text
SignupReq {
  email, password, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  agree_terms_service: bool, agree_terms_personal: bool
}

SignupRes { user_id: i64 }

ProfileRes {
  id, email, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  user_state: String, user_auth: String
}

UpdateReq {
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String
}

SettingsUpdateReq {
  ui_language?: String,                        # ISO 639-1 (예: 'en','ko','ne','si','id','vi','th')
  study_languages?: [ { lang_code: String, priority: i32, is_primary: bool } ],
  timezone?: String,
  notifications_email?: bool,
  notifications_push?: bool
}

SettingsRes = SettingsUpdateReq + { user_id }

