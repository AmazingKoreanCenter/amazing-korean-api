# Amazing Korean – Backend Context (Axum + SQLx)

> 업데이트: 2025-09-04

## Goal

- Rust + Axum 기반 API 서버를 Gemini CLI로 확장/리팩터링
- 우선 과제였던 유저/설정/관리자 기능을 일관된 라우팅·문서화 규칙으로 정비

## Stack

- **Rust**: Axum 0.8, Tokio, SQLx (PostgreSQL), utoipa (Swagger UI)
- **DB**: PostgreSQL (Docker: `amk-pg`, port 5432)  
  - 모든 시간 컬럼은 **TIMESTAMPTZ(UTC)**, DB 레벨 `DEFAULT now()`
  - 마이그레이션은 **수정/개명 금지**; 변경은 **새 파일 추가** 원칙
  - `users_setting`, `users_language_pref`, `user_export_job`의 `updated_at`은 트리거로 자동 갱신
- **Redis**: (Docker: `amk-redis`, port 6379)  
  - 세션/리프레시 토큰 저장: `ak:session:<sid>`, `ak:refresh:<hash> -> <sid>`, `ak:user_sessions:<uid>`
- **Run**: `cargo run` (bind: `0.0.0.0:3000`), OpenAPI: `/api-docs/openapi.json`, Swagger UI: `/docs`

## Current APIs

1. **회원가입**
   - `POST /users`
   - 이메일/비밀번호/이름/약관 필수, 선택 필드(닉네임/언어/국가/생일/성별)
   - **성공 시 `USERS_LOG` 스냅샷 기록** (비밀번호는 항상 NULL)

2. **내 프로필**
   - `GET /users/me` (JWT)
     - 로그인한 사용자 정보 반환
   - `PUT /users/me` (JWT)
     - 닉네임/언어/국가/생일/성별 업데이트
     - **성공 시 `USERS_LOG` 스냅샷 기록**

3. **사용자 설정**
   - `GET /users/me/settings` (JWT)
   - `PUT /users/me/settings` (JWT, 부분 업데이트 / 학습 언어는 전체 교체)
   - 테이블명 **`public.users_setting`** 기준으로 통일

4. **관리자 (RBAC: HYMN/admin/manager)**
   - `GET /admin/users`
   - `GET /admin/users/{user_id}`
   - `PUT /admin/users/{user_id}` (수정 시 감사 로그 기록)

5. **개인정보 내보내기 (준비 중)**
   - `POST /users/me/export` → 202 Accepted (비동기 처리 예정)

## Router Architecture

- **User**: `/users`, `/users/me`, `/users/me/settings`
- **Admin (Aggregator)**: `api/admin/router.rs`에서 카테고리별 라우터를 **집약**하여 `/admin`에 한 번만 mount

```rust
// src/api/admin/router.rs
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .nest("/users", admin_user_router())
        // .nest("/courses", admin_course_router())
        // .nest("/reports", admin_report_router())
}
```
- 하위 라우터는 **상대 경로**만 사용

```rust
// src/api/admin/user/router.rs
pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_users))
        .route("/{user_id}", get(admin_get_user).put(admin_update_user))
}
```
- 앱 합성:

```rust
// src/api/mod.rs
Router::new()
    .merge(user_router())
    .merge(auth_router())
    .merge(course_router())
    .nest("/admin", admin_router()); // /admin 접두어는 한 번만
```

## Error Policy (v2025-08-29)

- 200 OK / 201 Created / 202 Accepted / 204 No Content
- 400 Bad Request (DTO/형식 검증 실패)
- 401 Unauthorized (토큰 없음/무효)
- 403 Forbidden (상태/권한 불가; 예: `user_state!='on'`)
- 404 Not Found
- 409 Conflict (무결성/충돌; 23505 UNIQUE, 23503 FK, 23514 CHECK 등)
- 412 Precondition Failed (ETag/If-Match 시)
- 415/406 (필요 시)
- 422 Unprocessable Entity (팀 합의 없으면 400 유지)
- 429 Too Many Requests (레이트리밋/쿨다운)
- 500/502/503/504

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

- `@./src/api/user/repo.rs`
- `@./src/api/user/service.rs`
- `@./src/api/user/dto.rs`
- `@./src/api/user/handler.rs`
- `@./Cargo.toml`
- `@./.env.example`
- `@./migrations/*_amk_users.sql` (2025-08-29, TIMESTAMPTZ/트리거 반영)

## Endpoints & Naming (구현 현황)

1) **회원가입**
- `POST /users`  
  **H/S/R**: `signup` → `signup` → `create_user`  
  **DTO**: `SignupReq → SignupRes{ user_id }`  
  **Status**: 201, 409, 400, 500  
  **Rule**: 이메일 UNIQUE(23505→409), validator, 약관 필수, **성공 시 USERS_LOG 기록**

2) **내 프로필 조회**
- `GET /users/me` (Bearer)  
  **H/S/R**: `get_me` → `get_me` → `find_user`  
  **DTO**: `— → ProfileRes`  
  **Status**: 200, 401, 403(`user_state!='on'`), 404, 500

3) **내 프로필 수정**
- `PUT /users/me` (Bearer)  
  **H/S/R**: `update_me` → `update_me` → `update_user`  
  **DTO**: `UpdateReq → ProfileRes`  
  **Status**: 200, 400, 401, 403, 404, 500  
  **Rule**: **성공 시 USERS_LOG 기록**

4) **사용자 스냅샷 기록 (USERS_LOG)**
- 트리거: `POST /users`, `PUT /users/me` 성공 직후  
- **전략**: `INSERT … SELECT`로 **DB의 실제 저장값**을 스냅샷 (비밀번호는 항상 NULL)  
- **규칙**:  
  - `action`: 가입=`"create"`, 수정=`"update"`  
  - `updated_by_user_id`: 가입=신규 사용자 ID, 수정=JWT 주체 ID  
  - 로깅 실패는 `warn` 후 **본 흐름 유지**

5) **환경 설정(계정/학습/알림)**
- `GET /users/me/settings`, `PUT /users/me/settings` (Bearer)  
  **H/S/R**: `get_settings` / `update_users_setting` → 同 → `find_users_setting` / `upsert_settings`  
  **DTO**: `SettingsRes`, `SettingsUpdateReq → SettingsRes`  
  **동작**:  
  - `users_setting`: 제공된 필드만 **부분 업데이트**  
  - `users_language_pref`: `study_languages`가 **요청에 포함된 경우**에 한해 **전체 교체(Replace-all)**, priority는 1..N  
  **검증**:  
  - 언어코드 허용: `en, ko, ne, si, id, vi, th`  
  - `is_primary`는 0~1개, `priority >= 1`

6) **관리자: 목록/조회/수정**
- `GET /admin/users?query=&state=&page=&size=`  
- `GET /admin/users/{user_id}`  
- `PUT /admin/users/{user_id}`  
  **H/S/R**: `admin_list` / `admin_get` / `admin_update`  
  **DTO**: `— → AdminListUsersRes`, `— → AdminUserRes`, `AdminUpdateUserReq → AdminUserRes`  
  **RBAC**: `user_auth ∈ { HYMN, admin, manager }`만 접근 허용  
  - 추가 정책: `manager/admin → HYMN 대상 변경 불가`  
  **감사 로그**: `admin_user_action_log(action='admin_update', before, after)` 기록  
  - (**TODO**) `actor_user_id`를 **레포 단에서 트랜잭션과 함께 전달**하도록 시그니처 보강

7) **개인정보 내보내기(Export, 비동기)** — 스펙만, 구현 대기
- `POST /users/me/export` → 202 Accepted + `job_id`  
- 워커가 ZIP(JSON/CSV) 생성 후 signed URL + 만료 저장

## DTOs (요약 · 구현 기준 반영)

```text
SignupReq {
  email, password, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  terms_service: bool, terms_personal: bool
}
SignupRes { user_id: i64 }

ProfileRes {
  id, email, name,
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String,
  user_state: String, user_auth: String, created_at: DateTime<Utc>
}

UpdateReq {
  nickname?, language?, country?, birthday?: NaiveDate, gender?: String
}

SettingsUpdateReq {
  ui_language?: String,  # ISO 639-1 (en, ko, ne, si, id, vi, th)
  study_languages?: [ { lang_code: String, priority: i32, is_primary: bool } ],
  timezone?: String,
  notifications_email?: bool,
  notifications_push?: bool
}

SettingsRes = SettingsUpdateReq + { user_id: i64 }
```

## Swagger

- Bearer 보안 스키마 적용
- 관리자 경로는 문서상 **절대 경로** 사용: `"/admin/users"`, `"/admin/users/{user_id}"`
- 경로 파라미터는 Axum 0.8 문법(`{user_id}`)

---

## Video APIs (V1 — Streaming & Progress)

> 목표: 공개/유료/비공개 접근 범위에 맞춰 **목록/상세/자막/진척**을 우선 구현하고, 이후 **운영(관리자)** 과 **통계**로 확장한다.  
> 스키마: `video`, `video_log`, `video_caption`, `video_tag(_map)`, `video_stat_daily` (UTC TIMESTAMPTZ)

### 엔드포인트 일람 (핵심 → 운영/관리 → 통계)

| 구분 | 메서드 | 경로 | 인증 | 목적/설명 | 주요 쿼리/바디 |
|---|---|---|---|---|---|
| 목록 | GET | `/videos` | 선택 | 공개 목록 조회(기본: `state=open`, `access=public`) | `q, tag, lang, access, state, limit, offset, sort, order` |
| 상세 | GET | `/videos/{id}` | 선택 | 단건 메타(+ `vimeo_video_id`) | — |
| 자막목록 | GET | `/videos/{id}/captions` | 선택 | 해당 영상 자막 트랙 리스트 | — |
| 진척조회 | GET | `/videos/{id}/progress` | JWT | 사용자 개인 진척(없으면 200 + 기본값) | — |
| 진척업서트 | PUT | `/videos/{id}/progress` | JWT | 시청 위치/퍼센트/완료 업데이트(디바운스 5–10s) | JSON 바디 |
| **관리** | POST | `/admin/videos` | Admin | 영상 생성 | JSON |
| 〃 | PUT | `/admin/videos/{id}` | Admin | 영상 수정 | JSON |
| 〃 | DELETE | `/admin/videos/{id}` | Admin | 소프트삭제(`deleted_at`) | — |
| 〃 | POST | `/admin/videos/{id}/ban` | Admin | 차단/해제(사유 포함) | `{reason}` |
| 〃 | POST | `/admin/videos/{id}/captions` | Admin | 자막 추가(Vimeo/S3/URL) | JSON |
| 〃 | PUT | `/admin/videos/{id}/captions/{caption_id}` | Admin | 자막 수정/기본지정/비활성 | JSON |
| 〃 | DELETE | `/admin/videos/{id}/captions/{caption_id}` | Admin | 자막 삭제 | — |
| 〃 | POST | `/admin/videos/{id}/tags` | Admin | 태그 매핑 추가 | `{tag_key}` |
| 〃 | DELETE | `/admin/videos/{id}/tags/{tag_key}` | Admin | 태그 매핑 제거 | — |
| **통계** | GET | `/admin/videos/{id}/stats/daily` | Admin | 일별 조회/완료 | `from,to` |
| 〃 | GET | `/admin/videos/stats/top` | Admin | 구간 상위 N(views/completes) | `from,to,metric,limit` |

**게이트 규칙(요약)**  
- 게스트: `state=open` + `access=public`만 열람  
- 로그인 사용자: 위 + 개인 진척 API 사용 가능  
- `access=paid`: **결제 연동 후** 인가 미들웨어에서 허용 (임시로 게스트 비노출)  
- `access=private`와 `state=ready/close`: Admin만

---

## Gemini Prompt Playbook (단계별/게이트 방식)

> **원칙:** “작은 단계 → 컴파일/Swagger/런타임 검증 → 다음 단계”. 한 단계가 100% 통과되면 **그때만** 다음 단계 프롬프트를 보낸다.

### 공통 템플릿 (각 단계 프롬프트 앞머리에 항상 포함)

```text
[역할/맥락]
- 너는 Rust(Axum 0.8) + SQLx(PostgreSQL) + utoipa v5 기반 백엔드에서 "한 단계"만 정확히 구현한다.
- 절대 범위를 넘지 말 것. 이 단계가 100% 통과되면 내가 다음 프롬프트로 다음 단계를 줄 것이다.

[프로젝트 규칙]
- Router<AppState> 패턴, 최상위 Router는 .with_state(state).
- 에러 타입: AppError / AppResult<T>, HTTP status 매핑 준수.
- JWT 인증 미들웨어 존재 가정(Bearer). DB 시간은 UTC TIMESTAMPTZ.
- SQLx는 query_as::<_, T>() 우선, DTO-컬럼 매칭 엄격.
- Swagger(utoipa v5): #[utoipa::path] 문서화, /docs 노출.
- 컴파일 제약: cargo check, cargo fmt -- --check, cargo clippy -- -D warnings 모두 통과.

[출력 형식]
- 수정/신규 파일은 **전체 내용(완본)** 으로 제공(패치 대신 교체본).
- 단계 범위를 벗어난 구현/리팩터링 금지.

[검증 커맨드]
- cargo check
- cargo fmt -- --check
- cargo clippy -- -D warnings
- sqlx migrate run        # DB 연결 가능 시
- (권한 필요 시) curl 스모크 테스트 포함

[성공 기준]
- 빌드/클리피 경고 0.
- Swagger에 해당 엔드포인트가 정확히 노출되고 2xx 응답.
- 내가 제공하는 cURL 예시가 2xx를 받는다(권한 요구 시 401/403 포함).

[실패 시]
- 원인 요약 → 정확한 수정 패치(다시 전체 파일 교체본).
```

### 단계 A (핵심) — 목록/상세/자막/진척

**A1. 스캐폴딩/라우터 마운트/Swagger 태그**

```text
[목표]
- /videos 네임스페이스 스켈레톤 생성: src/api/video/{router.rs,handler.rs,service.rs,repo.rs,dto.rs,mod.rs}
- GET /videos/health → 200 {"ok":true}, tags=["videos"], 최상위 라우터에 .nest("/videos", video::router()) 마운트
[검증]
- cargo check && cargo clippy -- -D warnings
- curl -sS http://localhost:3000/videos/health | jq .
```

**A2. GET /videos (목록)**

```text
[목표]
- 기본 필터: state=open, access=public
- 쿼리: q, tag(복수), lang, state?, access?, limit, offset, sort(created_at|popular|complete_rate), order(asc|desc)
- 응답: { video_id, video_idx, title, subtitle, duration_seconds, language, thumbnail_url, state, access, tags[], has_captions, created_at }
- popular/complete_rate는 TODO 주석(후속 단계에서 통계 조인)
[검증]
- cargo check && cargo clippy -- -D warnings
- curl 'http://localhost:3000/videos?limit=5' -i
```

**A3. GET /videos/{id} (상세)**

```text
[목표]
- 단건 메타 + vimeo_video_id + tags[] + has_captions
- 게이트: state=open && access=public 이외 접근은 403
[검증]
- curl 'http://localhost:3000/videos/{id}' -i
```

**A4. GET /videos/{id}/captions (자막)**

```text
[목표]
- 응답: [{caption_id, lang_code, label, kind, is_default, is_active}]
- 게이트: video가 열람 가능한 경우에만 노출
[검증]
- curl 'http://localhost:3000/videos/{id}/captions' -i
```

**A5. GET/PUT /videos/{id}/progress (개인 진척)**

```text
[목표]
- GET: 개인 진척 반환(없으면 기본값 0)
- PUT: {last_position_seconds, total_duration_seconds?, progress?, completed?}
- 서버는 PL/pgSQL 함수 api_upsert_video_progress(...) 호출 후 최신 video_log 반환
[검증]
- export TOKEN='Bearer <JWT>'
- curl -H "Authorization: $TOKEN" 'http://localhost:3000/videos/{id}/progress' -i
- curl -X PUT -H "Authorization: $TOKEN" -H "Content-Type: application/json"   -d '{"last_position_seconds":120,"total_duration_seconds":600,"progress":20,"completed":false}'   'http://localhost:3000/videos/{id}/progress' -i
```

> **주의:** A단계 완료 전에는 B/C 단계 요청 금지. A5까지 모두 통과하면 다음 단계 프롬프트를 보낸다.

---

### 단계 B (운영/관리) — Admin 전용

- **B1.** `POST /admin/videos` (생성)  
- **B2.** `PUT /admin/videos/{id}` (수정)  
- **B3.** `DELETE /admin/videos/{id}` (소프트삭제)  
- **B4.** `/admin/videos/{id}/captions` (POST/PUT/DELETE) — Vimeo/S3 겸용, `is_default` 부분 유니크 준수  
- **B5.** `/admin/videos/{id}/tags` (POST/DELETE) — `tag_key` 매핑  
  - 각 단계마다 RBAC(권한) 검증 cURL 포함

### 단계 C (통계)

- **C1.** `GET /admin/videos/{id}/stats/daily?from=&to=` — 일별 조회/완료  
- **C2.** `GET /admin/videos/stats/top?metric=views|completes&from=&to=&limit=` — 상위 N

---

## 구현 메모 (운영 규칙)

- 완료 판정: `last_position_seconds >= 0.9 * duration` **또는** Vimeo `ended` 이벤트 수신
- `PUT /progress`는 5–10초 디바운스; 값은 **최댓값 유지**, `progress`는 [0,100] 클램프
- 자막 기본 선택: `VIDEO_CAPTION.is_default=true`가 있으면 우선, 없으면 `VIDEO.video_language`와 사용자 UI 언어 매칭
- 목록 정렬 `popular/complete_rate`는 `video_stat_daily` 조인으로 후속 구현
- 접근권한: 결제 연동 전에는 `access=paid` **게스트 비노출**, 로그인 사용자에 한해 상세 열람 가능하도록 임시 분기 (결제 연동 시 미들웨어로 대체)