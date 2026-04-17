# AMK Code Patterns (Best Practices)

> 이 문서는 **AMK_API_MASTER.md**에서 분리된 코드 예시 섹션입니다.
> 규칙/스펙은 [AMK_API_MASTER.md](./AMK_API_MASTER.md), 배포/운영은 [AMK_DEPLOY_OPS.md](./AMK_DEPLOY_OPS.md)를 참조하세요.

> 이 섹션의 코드는 **실제 프로젝트에서 검증된 패턴**입니다.
> AI 에이전트에게 새 기능 요청 시 "AMK_CODE_PATTERNS.md의 패턴 X 사용" 지시하면 일관된 코드 생성 가능.

---

## 0. 엔지니어링 원칙 (작업 방식 가이드)

> AMK_API_MASTER.md §7에서 이관됨. 작업 원칙, 개발 플로우, DTO/검증, 서비스 계층, 트랜잭션 패턴, 테스트.

### 0.1 작업 원칙

1. **문서 우선** — 스펙/기능/규칙은 항상 AMK_API_MASTER.md를 기준으로 한다.
2. **정적 가드 필수** — `cargo fmt -- --check` + `cargo clippy -- -D warnings` + `cargo check` 모두 통과해야 PR/머지 가능.
3. **마이그레이션 규칙** — 이미 적용된 마이그레이션은 수정/이름 변경 금지. 변경은 항상 새 마이그레이션 추가. SQLx 마커는 `--! up` / `--! down`만 사용.
4. **로그/감사** — 도메인별: `USERS_LOG`, `LOGIN_LOG`, `VIDEO_LOG`, `STUDY_TASK_LOG`. 관리자: `ADMIN_USERS_LOG`, `ADMIN_VIDEO_LOG`, `ADMIN_STUDY_LOG`, `ADMIN_LESSON_LOG`.
5. **보안**
   - `user_state == 'on'` 인 사용자만 로그인/액세스 허용
   - `Authorization: Bearer <ACCESS_TOKEN>` 필수, 토큰 `sub`(user_id)만 신뢰
   - 비밀번호 Argon2 해시, 원문 절대 저장/로그 금지
   - 세션/리프레시 Redis 키(`ak:session:*`, `ak:refresh:*`), rotate-on-use
   - 관리자 RBAC: HYMN(전체) > admin(전체 읽기/쓰기) > manager(담당 class, 향후) > learner(자신만)
   - 구현: `src/api/admin/role_guard.rs`, `admin_ip_guard.rs`, 프론트 `AdminRoute`
   - HTTPS 필수, 토큰/세션 ID를 URL에 노출 금지
   - **Fail-closed 원칙**: 세션/인증 데이터 파싱 실패 시 반드시 접근 거부 (`if let Ok(...)` 패턴으로 실패 무시 금지)
   - **암호화 에러 불투명화**: `CryptoError` variant(InvalidFormat/DecryptionFailed/Internal)는 내부 로깅용으로만 구분, HTTP 응답은 전부 500 + generic 메시지 (format oracle 방지, OWASP CWE-209/203)

### 0.2 개발 플로우

1. 문서 확인 (AMK_API_MASTER.md + 관련 파일)
2. 기존: 문서 참조 후 작업 / 신규: API 명시 → 문서 업데이트 → 작업
3. 코드/마이그레이션 생성 (이 문서의 패턴 참조)
4. 정적 가드 + 스모크 테스트
5. 로드맵 체크박스 업데이트 + 문서 동기화

### 0.3 DTO/검증 규칙

- HTTP 경계에서 항상 DTO(struct) 사용, 내부 도메인 타입과 분리
- 문자열: `trim` 후 검증, 길이 제한 명시, 공백만 있으면 빈 값 → 400
- 이메일: RFC 형식 검증 (`validator` 크레이트), 대소문자 미구분
- 비밀번호: 최소 8자+, DTO에서 길이만 검증, 해시는 service 계층
- 날짜: `chrono::NaiveDate`, DB에 `DATE`/`TIMESTAMPTZ`
- Enum: 정의되지 않은 값 → 400, AMK_API_MASTER.md §4 기준
- ID: 음수 불가 (0 또는 음수 → 400), 페이징: `page >= 1`, `1 <= size <= 100`

### 0.4 서비스 계층 및 파일 구조

```
api/{domain}/
├── dto.rs       # 요청/응답 DTO (HTTP 경계 타입)
├── handler.rs   # Axum 핸들러 (파싱 → service 호출, 비즈니스 로직 없음)
├── service.rs   # 비즈니스 로직 (트랜잭션, 검증, repo 호출)
├── repo.rs      # DB 접근 (sqlx 쿼리, 비즈니스 규칙 없음)
├── router.rs    # 라우트 + 미들웨어 바인딩
└── mod.rs       # 모듈 선언 + re-export
```

특수 모듈:
- `api/auth/`: `extractor.rs` (Claims), `jwt.rs`, `token_utils.rs`
- `api/health/`: `handler.rs` (헬스체크)
- `api/scripts/db_fastcheck.sh`: DB 준비 상태 점검

### 0.5 트랜잭션 패턴

**원칙**: handler는 트랜잭션 열지 않음 → **service에서 시작/커밋/롤백** → repo는 넘겨받은 executor에서 쿼리만 실행.

**순서**: 검증 → 메인 변경 → 로그 → (통계/파생) → 커밋

```rust
pub async fn update_user_and_log(state: &AppState, input: UpdateUserInput, actor_id: i64) -> AppResult<User> {
    let mut tx = state.db.begin().await?;
    let user = user_repo::update_user(&mut tx, &input).await?;
    user_repo::insert_users_log(&mut tx, &user, actor_id, "update_profile").await?;
    tx.commit().await?;
    Ok(user)
}
```

**repo 제네릭 executor 패턴**: `&PgPool` / `&mut Transaction<'_, Postgres>` 모두 수용.

**API upsert 패턴** (예: 비디오 진도): DB 함수(`api_upsert_video_progress`)에서 INSERT/UPDATE + 로그를 한 번에 처리.

### 0.6 테스트 & 자동화

1. **최소 정적 가드** (로컬 + CI): `cargo fmt -- --check` + `cargo clippy -- -D warnings` + `cargo check`
2. **스모크 테스트**: `scripts/smoke_*.sh` (cURL 기반, 성공 + 대표 에러 케이스)
3. **향후**: CI 자동화 (PR 시 정적 가드 + 스모크) → K6 부하 테스트

---

## 📋 목차 (Table of Contents)

- [0. 엔지니어링 원칙 (작업 방식 가이드)](#0-엔지니어링-원칙-작업-방식-가이드)
- [1. 백엔드 패턴 (Rust/Axum)](#1-백엔드-패턴-rustaxum)
  - [1.0 공용 코드 (Common Code)](#10-공용-코드-common-code)
  - [1.1 dto.rs](#11-dtors)
  - [1.2 repo.rs](#12-repors)
  - [1.3 service.rs](#13-servicers)
  - [1.4 handler.rs](#14-handlerrs)
  - [1.5 router.rs](#15-routerrs)
  - [1.6 기타 파일들 (Auth 유틸리티)](#16-기타-파일들-auth-유틸리티)
- [2. 프론트엔드 패턴 (React/TypeScript)](#2-프론트엔드-패턴-reacttypescript)
  - [2.1 types.ts (Zod 스키마 & 타입 정의)](#21-typests-zod-스키마--타입-정의)
  - [2.2 *_api.ts (API 함수)](#22-_apits-api-함수)
  - [2.3 hook/*.ts (TanStack Query 훅)](#23-hookts-tanstack-query-훅)
  - [2.4 page/*.tsx (페이지 컴포넌트)](#24-pagetsx-페이지-컴포넌트)
  - [2.5 공용 인프라 (Shared Infrastructure)](#25-공용-인프라-shared-infrastructure)
  - [2.6 프론트엔드 데이터 흐름 (Data Flow)](#26-프론트엔드-데이터-흐름-data-flow)

---

## 1. 백엔드 패턴 (Rust/Axum)

---

### 1.0 공용 코드 (Common Code)

> **📋 SSoT 검증 완료** (2026-01-22)
> 아래 내용은 실제 코드 기반으로 검증되었습니다.

#### 파일 목록 및 역할

| 파일 | 역할 | 의존 관계 |
|------|------|----------|
| `src/config.rs` | 런타임 설정 SSoT (환경변수 파싱) | dotenvy |
| `src/state.rs` | AppState 의존성 컨테이너 | config.rs |
| `src/error.rs` | 전역 에러 타입 + HTTP 응답 표준화 | 독립 |
| `src/types.rs` | DB enum ↔ Rust enum ↔ JSON 매핑 | 독립 |
| `src/docs.rs` | OpenAPI 문서 집계 + 보안 스키마 | 도메인 핸들러들 |
| `src/main.rs` | 부트스트랩 (리소스 생성 → 서버 실행) | 모든 모듈 |
| `src/api/mod.rs` | 도메인 라우터 조립 | 도메인 라우터들 |

---

#### 1️⃣ `src/config.rs` — 런타임 설정 SSoT

**역할**: 환경변수 기반 설정의 **단일 진입점**. 모든 런타임 파라미터가 이 파일에서 관리됨.

```rust
use std::env;

#[derive(Clone)]
pub struct Config {
    // 필수 인프라
    pub database_url: String,
    pub bind_addr: String,
    pub redis_url: String,

    // JWT 설정 (필수)
    pub jwt_secret: String,
    pub jwt_expire_hours: i64,
    pub jwt_access_ttl_min: i64,

    // Refresh Token 설정
    pub refresh_ttl_days: i64,
    pub refresh_cookie_name: String,
    pub refresh_cookie_domain: Option<String>,
    pub refresh_cookie_secure: bool,
    pub refresh_cookie_samesite: String,

    // 기능 토글
    pub enable_docs: bool,
    pub skip_db: bool,

    // Rate Limit
    pub rate_limit_login_window_sec: i64,
    pub rate_limit_login_max: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // JWT_SECRET만 필수 (expect)
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        // 나머지는 기본값 제공
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://...".into());
        // ... 생략
    }
}
```

**🔑 핵심 포인트**:
- `JWT_SECRET`만 **필수** (없으면 panic) — 보안 강제
- 나머지 설정은 **기본값 제공** — 로컬 개발 편의성
- `refresh_cookie_samesite_or()` 헬퍼로 빈 문자열 처리

**⚠️ 규칙**:
- 새 환경변수 추가 시 → **반드시 Config에 필드 추가**
- 기본값 결정 시 → 로컬 개발 편의 vs 프로덕션 안전성 고려

---

#### 2️⃣ `src/state.rs` — AppState 의존성 컨테이너

**역할**: 핸들러/서비스/레포에서 공통 접근하는 **의존성 묶음**

```rust
use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;

use crate::config::Config;
use crate::external::email::EmailSender;
use crate::external::ipgeo::IpGeoClient;
use crate::external::payment::PaymentProvider;
use crate::external::revenuecat::RevenueCatClient;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,                                    // Postgres 커넥션 풀
    pub redis: RedisPool,                              // Redis 커넥션 풀
    pub cfg: Config,                                   // 런타임 설정
    pub started_at: Instant,                           // 서버 시작 시간
    pub email: Option<Arc<dyn EmailSender>>,           // 이메일 (Resend)
    pub ipgeo: Arc<IpGeoClient>,                       // IP Geolocation
    pub payment: Option<Arc<dyn PaymentProvider>>,     // 결제 (Paddle)
    pub revenuecat: Option<Arc<dyn RevenueCatClient>>, // 모바일 IAP (RevenueCat)
}
```

**🔑 핵심 포인트**:
- `#[derive(Clone, FromRef)]` → State 추출 + 부분 추출 가능
- 핸들러에서: `State(state): State<AppState>`
- 서브스테이트 추출: `State(db): State<Pool<Postgres>>`

**⚠️ 규칙**:
- 핸들러 → `&state.db`, `&state.redis`, `&state.cfg`로 하위 레이어에 전달
- 새 전역 리소스 추가 시 → AppState에 필드 추가 + main.rs에서 초기화

---

#### 3️⃣ `src/error.rs` — 전역 에러 타입 + HTTP 응답 표준화

**역할**: 모든 레이어의 에러를 **통일된 HTTP 응답**으로 변환

```rust
use thiserror::Error;
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, Error)]
pub enum AppError {
    // 비즈니스 에러
    #[error("Internal server error")]
    Internal(String),
    #[error("Health check failed: {0}")]
    HealthInternal(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unprocessable entity: {0}")]
    Unprocessable(String),         // 422
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),              // 409
    #[error("Too many requests: {0}")]
    TooManyRequests(String),       // 429
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),    // 503
    #[error("External service error: {0}")]
    External(String),              // 502

    // 인프라 에러 (자동 변환)
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

// 전역 Result 타입
pub type AppResult<T> = Result<T, AppError>;
```

**HTTP 응답 표준화**:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details, retry_after) = match &self {
            AppError::BadRequest(msg) =>
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone(), None, None),
            AppError::Unauthorized(msg) =>
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone(), None, None),
            AppError::TooManyRequests(msg) =>
                (StatusCode::TOO_MANY_REQUESTS, "TOO_MANY_REQUESTS", msg.clone(), None, Some(60)),
            // ... 기타 매칭
        };

        // 표준 에러 바디
        let body = serde_json::json!({
            "error": {
                "code": code,
                "http_status": status.as_u16(),
                "message": message,
                "details": details,
                "trace_id": crate::trace_id::current().unwrap_or_else(|| "unknown".to_string()),  // src/trace_id.rs 의 task_local! 에서 주입 (UUID v7 또는 업스트림 x-request-id 승계)
            }
        });

        let mut res = (status, Json(body)).into_response();
        if let Some(sec) = retry_after {
            res.headers_mut().insert(
                header::RETRY_AFTER,
                sec.to_string().parse().unwrap()
            );
        }
        res
    }
}
```

**🔑 핵심 포인트**:
- `AppResult<T>` = `Result<T, AppError>` — 전 레이어 공용
- `?` 연산자로 에러 전파 → 자동으로 HTTP 응답 변환
- `#[from]` 어트리뷰트로 인프라 에러 자동 래핑
- 429 응답 시 `Retry-After` 헤더 자동 추가
- **trace_id**: `src/trace_id.rs` 미들웨어가 요청별 UUID v7 을 `tokio::task_local!` 에 주입.
  `AppError::into_response` 에서 `trace_id::current()` 로 동기 조회.
  - 들어오는 `x-request-id` 헤더가 유효(ASCII alphanumeric/`-`/`_`, ≤128자)하면 승계, 없으면 UUID v7 생성
  - 응답 헤더 `x-request-id` 로 에코백 (CORS `expose_headers` 로 브라우저에도 노출)
  - 핸들러에서 `Extension<TraceId>` 로 직접 추출도 가능 (로깅 span, 감사 로그 기록 시 사용)
- **요청 바디 파싱**: 추출기는 항상 `AppJson<T>` (src/extract.rs) 사용. axum 기본
  `Json<T>` 는 JsonRejection 시 `text/plain` 응답으로 표준 envelope + trace_id 를
  우회하므로 요청 파싱 용도로는 **금지**. 응답 직렬화용 `Json(res)` 는 그대로 OK.
  - `AppJson<T>` 가 JsonRejection 을 AppError 로 매핑:
    JsonDataError → Unprocessable(422), JsonSyntaxError/MissingContentType/
    BytesRejection → BadRequest(400). 전 경로 envelope + trace_id 통일.

**⚠️ 규칙**:
- 새 에러 타입 필요 시 → `AppError` variant 추가
- 서비스/레포에서 `Err(AppError::NotFound)` 형태로 반환
- 프론트엔드는 `error.code` 필드로 에러 종류 판단
- 요청 바디 파싱 핸들러 파라미터는 `AppJson(x): AppJson<T>` 형태 고정

---

#### 4️⃣ `src/types.rs` — DB enum ↔ Rust enum ↔ JSON 매핑

**역할**: DB enum 타입의 **단일 정의** (중복 enum 금지)

```rust
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use std::fmt;

// Triple Derive 패턴: sqlx + serde + utoipa (예시 — 실제 UserAuth는 Hymn/Admin/Manager/Learner)
#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "user_auth")]        // DB enum 이름
#[serde(rename_all = "lowercase")]      // JSON: "google", "email"
pub enum UserAuth {
    Google,
    Apple,
    Email,
}

// Display 구현 (로깅용)
impl fmt::Display for UserAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAuth::Google => write!(f, "google"),
            UserAuth::Apple => write!(f, "apple"),
            UserAuth::Email => write!(f, "email"),
        }
    }
}

// 특수 케이스: DB와 API 이름이 다를 때
#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "lesson_type")]
#[serde(rename_all = "lowercase")]
pub enum LessonType {
    Video,
    #[sqlx(rename = "HYMN")]           // DB에는 대문자로 저장
    #[serde(rename = "hymn")]          // JSON에는 소문자로 노출
    Hymn,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "user_level")]
pub enum UserLevel {
    #[serde(rename = "basic_900")]     // JSON: "basic_900"
    Basic900,
    #[serde(rename = "basic_1800")]
    Basic1800,
    // ...
}
```

**🔑 핵심 포인트**:
- **Triple Derive**: `sqlx::Type` + `serde` + `utoipa::ToSchema`
- DB enum 이름: `#[sqlx(type_name = "...")]`
- JSON 직렬화: `#[serde(rename_all = "...")]` 또는 개별 `#[serde(rename = "...")]`
- 예외 케이스: `#[sqlx(rename = "...")]`로 DB 값 명시

**⚠️ 규칙**:
- **중복 enum 정의 금지** — 모든 도메인에서 `crate::types::*` import
- 새 DB enum 추가 시 → 여기에 정의 + 마이그레이션 작성
- Swagger에 자동 노출됨 (ToSchema)

---

#### 5️⃣ `src/docs.rs` — OpenAPI 문서 집계

**역할**: 모든 API 경로와 스키마를 **단일 OpenAPI 문서**로 집계

```rust
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

// 보안 스키마 등록
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());

        // JWT Bearer Auth
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        // Refresh Token Cookie
        components.add_security_scheme(
            "refreshCookie",
            SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                utoipa::openapi::security::ApiKeyValue::new("ak_refresh"),
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Amazing Korean API", version = "0.1.0"),
    paths(
        // 모든 핸들러 함수 나열
        crate::api::auth::handler::login,
        crate::api::auth::handler::refresh,
        crate::api::user::handler::get_me,
        crate::api::video::handler::get_video_detail,
        // ... 전체 paths
    ),
    components(schemas(
        // 모든 DTO/Enum 나열
        crate::api::auth::dto::LoginRequest,
        crate::api::auth::dto::LoginResponse,
        crate::types::UserAuth,
        crate::error::ErrorBody,
        // ... 전체 schemas
    )),
    tags(
        (name = "Auth", description = "인증 관련 API"),
        (name = "User", description = "사용자 관련 API"),
        (name = "Video", description = "비디오 관련 API"),
        // ... 전체 tags
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
```

**🔑 핵심 포인트**:
- `paths(...)`: 문서화할 핸들러 함수 목록
- `components(schemas(...))`: 문서화할 DTO/Enum 목록
- `tags(...)`: Swagger UI 그룹핑
- `SecurityAddon`: `bearerAuth` + `refreshCookie` 스키마 등록

**⚠️ 규칙**:
- 새 핸들러 추가 시 → `paths(...)`에 등록 **필수**
- 새 DTO 추가 시 → `components(schemas(...))`에 등록 **필수**
- 핸들러에 `#[utoipa::path(...)]` 매크로 필수

---

#### 6️⃣ `src/main.rs` — 부트스트랩

**역할**: 애플리케이션 시작 순서 정의

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 설정 로드
    let cfg = Config::from_env();

    // 2) Tracing 초기화
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "amazing_korean_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3) Postgres 풀 생성
    let database_url = if cfg.database_url.contains("?") {
        cfg.database_url.clone()
    } else {
        format!("{}?connect_timeout=5", cfg.database_url)
    };

    let pool: Pool<Postgres> = if std::env::var("DB_EAGER").ok().as_deref() == Some("1") {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect(&database_url)      // 즉시 연결 시도
            .await?
    } else {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(&database_url)?  // 첫 쿼리 시 연결
    };

    // 4) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(cfg.redis_url.clone());
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 5) 외부 서비스 클라이언트 초기화 (Email, Payment, RevenueCat 등)
    // ... (main.rs 참조 — provider 분기 + Arc 래핑)

    // 6) AppState 생성
    let app_state = AppState {
        db: pool,
        redis,
        cfg: cfg.clone(),
        started_at: Instant::now(),
        email,        // Option<Arc<dyn EmailSender>>
        ipgeo,        // Arc<IpGeoClient>
        payment,      // Option<Arc<dyn PaymentProvider>>
        revenuecat,   // Option<Arc<dyn RevenueCatClient>>
    };

    // 6) CORS 설정
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT,
                       Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);  // 쿠키 교환 허용

    // 7) 라우터 조립 + CORS 레이어
    let app = api::app_router(app_state).layer(cors);

    // 8) 서버 시작
    let listener = TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("✅ Server listening on http://{}", cfg.bind_addr);

    axum::serve(listener, app).await?;
    Ok(())
}
```

**🔑 핵심 포인트**:
- **부트스트랩 순서**: Config → Tracing → DB Pool → Redis Pool → AppState → CORS → Router → Serve
- `DB_EAGER=1`: 즉시 DB 연결 (CI/프로덕션 권장)
- `connect_lazy()`: 첫 쿼리 시 연결 (로컬 개발 빠른 시작)
- `allow_credentials(true)`: Refresh Token 쿠키 교환 필수

**⚠️ 규칙**:
- 새 전역 리소스 추가 시 → main.rs에서 초기화 + AppState에 주입
- CORS origin 추가 필요 시 → `allow_origin()` 수정

---

#### 7️⃣ `src/api/mod.rs` — 도메인 라우터 조립

**역할**: 모든 도메인 라우터를 **최종 조립**

```rust
use crate::state::AppState;
use axum::routing::get;

use crate::docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// 도메인 모듈 선언
pub mod admin;
pub mod auth;
pub mod course;
pub mod health;
pub mod lesson;
pub mod study;
pub mod user;
pub mod video;

// 도메인 라우터 import
use self::admin::router::admin_router;
use self::auth::router::auth_router;
use self::course::router::course_router;
use self::lesson::router::router as lesson_router;
use self::study::router::router as study_router;
use self::user::router::user_router;
use self::video::router::router as video_router;

pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        // merge: 루트 레벨 라우터 결합
        .merge(course_router())
        .merge(user_router())
        // nest: URL prefix 분리
        .nest("/auth", auth_router())
        .nest("/admin", admin_router())
        .nest("/lessons", lesson_router())
        .nest("/videos", video_router())
        .nest("/studies", study_router())
        // Health check (직접 등록)
        .route("/healthz", get(health::handler::health))
        .route("/health", get(health::handler::health))
        .route("/ready", get(health::handler::ready))
        // Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // 전역 상태 주입
        .with_state(state)
}
```

**🔑 핵심 포인트**:
- `merge()`: 루트 레벨 경로 결합 (예: `/users`, `/courses`)
- `nest("/prefix", router)`: URL prefix 추가 (예: `/auth/login`, `/videos/123`)
- `.with_state(state)`: **마지막에 한 번만** 호출
- 도메인 라우터는 상태 없이 경로만 정의

**⚠️ 규칙**:
- 새 도메인 추가 시 → `pub mod xxx;` + `use self::xxx::router::xxx_router;` + `merge()` 또는 `nest()`
- Swagger에 노출할 경로만 docs.rs에 등록

---

#### 📊 공통 패턴 요약

| 관심사 | 파일 | 패턴 |
|--------|------|------|
| 설정 | `config.rs` | 환경변수 → Config 구조체 |
| 상태 | `state.rs` | AppState + FromRef |
| 에러 | `error.rs` | AppError + IntoResponse |
| 타입 | `types.rs` | Triple Derive (sqlx + serde + utoipa) |
| 문서 | `docs.rs` | utoipa OpenApi derive |
| 조립 | `api/mod.rs` | merge/nest + with_state |

#### 🔄 레이어 간 데이터 흐름

```
[HTTP Request]
      ↓
[Router] → Path 매칭
      ↓
[Handler] → State<AppState> 주입
      ↓
[Service] → 비즈니스 로직, &state.db 사용
      ↓
[Repo] → sqlx 쿼리, AppResult<T> 반환
      ↓
[Handler] → AppResult<Json<Response>>
      ↓
[AppError::IntoResponse] → 표준 JSON 에러
      ↓
[HTTP Response]
```

---

### 1.1 dto.rs

> **📋 SSoT 검증 완료** (2026-01-22)
> 아래 내용은 실제 코드 기반으로 검증되었습니다.

#### 파일 목록 및 역할

| 파일 | 역할 | 특징 |
|------|------|------|
| `src/api/auth/dto.rs` | 인증 요청/응답 (로그인, 토큰 등) | `#[schema(example)]` 적극 사용 |
| `src/api/lesson/dto.rs` | 레슨 목록/상세/진도 | `IntoParams` 사용, `sqlx::FromRow` |
| `src/api/study/dto.rs` | 학습 목록/문제/제출 | Tagged Union, types.rs enum 재사용 |
| `src/api/user/dto.rs` | 회원가입/프로필/설정 | PATCH 패턴, 자동 로그인 응답 |
| `src/api/video/dto.rs` | 비디오 목록/상세/진도 | JSONB 매핑, default 함수 |

---

#### dto.rs의 역할 (AMK 기준)

**API 경계 타입**: handler가 받는 입력(Query/Path/Json Body)과 반환(응답 바디)의 **"계약(Contract)"**을 정의

**문서화/검증의 중심**:
- `utoipa::ToSchema` / `IntoParams`로 OpenAPI 스키마 생성
- `validator::Validate`로 입력 검증 (특히 Body DTO)

**DB 스키마와의 관계**:
- 1:1로 같을 필요 없음 (보안/UX 목적에 따라 축약·가공 가능)
- 단, DB enum은 `crate::types::*`를 재사용해서 불일치/파싱 비용 최소화

---

#### 1️⃣ `src/api/auth/dto.rs` — 인증 요청/응답

**역할**: 로그인, 토큰 갱신, 아이디 찾기, 비밀번호 재설정, 로그아웃

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "email": "front@front.com",
    "password": "front123!",
    "device": "web",
    "browser": "chrome",
    "os": "linux"
}))]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 72))]
    pub password: String,

    // 클라이언트가 명시적으로 보낼 경우를 위해 Option 유지
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub browser: Option<String>,
    #[serde(default)]
    pub os: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct RefreshReq {
    // 쿠키를 사용할 수 없는 환경(앱 등)을 위해 바디로도 받을 수 있게 유지
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 액세스 토큰 공통 규격
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String, // "Bearer" 고정
    pub expires_in: i64,    // 초 단위
    pub expires_at: String, // 프론트엔드 편의용 ISO String
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct LoginRes {
    pub user_id: i64,
    pub access: AccessTokenRes,
    pub session_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct LogoutRes {
    pub ok: bool,
}
```

**🔑 핵심 포인트**:
- **Request/Response 섹션 분리**: 주석으로 명확히 구분
- **`#[schema(example = json!(...))]`**: Swagger UI에서 즉시 테스트 가능
- **`#[serde(default)]`**: 클라이언트가 보내지 않아도 OK (Option + default)
- **`AccessTokenRes` 공통화**: `LoginRes`, `RefreshRes` 등에서 재사용

**⚠️ 규칙**:
- `expires_at`은 프론트엔드 편의를 위해 String 유지 (ISO 8601 형식)
- `json!` 매크로 사용 시 파일 상단에 `use serde_json::json;` 확인

---

#### 2️⃣ `src/api/user/dto.rs` — 회원가입/프로필/설정

**역할**: 회원가입 (자동 로그인 포함), 프로필 CRUD, 환경설정

```rust
use crate::api::auth::dto::AccessTokenRes;
use crate::types::{UserAuth, UserGender};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 회원가입 요청
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignupReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 50))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub nickname: String,

    /// ISO 639-1 언어 코드 (예: "ko", "en")
    #[validate(length(min = 2, max = 2))]
    pub language: String,

    /// ISO 3166-1 alpha-2 국가 코드 (예: "KR", "US")
    #[validate(length(min = 2, max = 50))]
    pub country: String,

    #[schema(value_type = String, format = "date")]
    pub birthday: NaiveDate,

    pub gender: UserGender, // Enum: male, female, other, none

    pub terms_service: bool,
    pub terms_personal: bool,
}

/// 프로필 수정 요청 (PATCH)
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileUpdateReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub nickname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<UserGender>,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 회원가입 완료 응답 (자동 로그인 처리됨)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignupRes {
    pub user_id: i64,
    pub email: String,
    pub name: String,
    pub nickname: String,
    // ... 기타 필드

    /// 자동 발급된 액세스 토큰
    pub access: AccessTokenRes,
    /// 현재 세션 ID
    pub session_id: String,
}

/// 사용자 프로필 정보
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileRes {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,
    pub gender: UserGender,
    pub user_state: bool,
    pub user_auth: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    /// 비밀번호 설정 여부 (OAuth 전용 계정은 false)
    pub has_password: bool,
}
```

**🔑 핵심 포인트**:
- **`crate::types::*` enum 재사용**: `UserGender`, `UserAuth` 등 DB enum과 일치
- **PATCH 패턴**: `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`
- **자동 로그인 응답**: `SignupRes`에 `AccessTokenRes` + `session_id` 포함
- **`#[schema(value_type = String, format = "date")]`**: Swagger에서 날짜 형식 표시
- **`sqlx::FromRow`**: DB 조회 결과 직접 매핑 가능

**⚠️ 규칙**:
- enum 필드는 **반드시** `crate::types::*` 사용 (String 금지)
- 날짜 필드에 `#[schema(value_type = String, format = "...")]` 필수

---

#### 3️⃣ `src/api/video/dto.rs` — 비디오 목록/상세/진도

**역할**: 비디오 검색/목록, 상세 정보, 학습 진도 관리

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 비디오 목록 조회 및 검색 요청 (Query String)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListReq {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u64,

    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: u64,

    pub q: Option<String>,          // 검색어
    pub tag: Option<String>,        // 태그 필터
    pub state: Option<String>,      // 상태 필터
    pub sort: Option<String>,       // 정렬
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

/// 학습 진도 업데이트 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoProgressUpdateReq {
    #[validate(range(min = 0, max = 100))]
    pub progress_rate: i32,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 목록 페이징 메타데이터
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}

/// 비디오 목록 응답 (Data + Meta)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListRes {
    pub meta: VideoListMeta,
    pub data: Vec<VideoListItem>,  // ⭐ 표준: { meta, data }
}

/// 상세 태그 정보 (JSONB 구조)
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub struct VideoTagDetail {
    pub key: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

/// 비디오 상세 정보
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoDetailRes {
    pub video_id: i64,
    pub video_url_vimeo: String,
    pub video_state: String,

    // DB의 JSONB 타입을 Rust 구조체로 매핑
    #[schema(value_type = Vec<VideoTagDetail>)]
    pub tags: Json<Vec<VideoTagDetail>>,

    pub created_at: DateTime<Utc>,
}

/// 학습 진도 조회 응답
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoProgressRes {
    pub video_id: i64,

    #[sqlx(rename = "video_progress_log")]
    pub progress_rate: i32,

    #[sqlx(rename = "video_completed_log")]
    pub is_completed: bool,
}
```

**🔑 핵심 포인트**:
- **`#[serde(default = "함수명")]`**: 페이징 기본값 설정
- **`#[validate(range(min = 0, max = 100))]`**: 범위 검증
- **JSONB 매핑**: `sqlx::types::Json<Vec<T>>` + `#[schema(value_type = Vec<T>)]`
- **`#[sqlx(rename = "...")]`**: DB 컬럼명 ↔ DTO 필드명 매핑
- **응답 표준**: `{ meta, data }` 구조 ⭐

**⚠️ 규칙**:
- 페이징 응답은 **`{ meta, data }`** 형태로 통일 권장
- JSONB 필드는 `#[schema(value_type = ...)]`로 Swagger 문서화

---

#### 4️⃣ `src/api/study/dto.rs` — 학습 목록/문제/제출

**역할**: 학습 프로그램 목록, 문제 상세, 정답 제출, 해설/상태 조회

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::types::{StudyProgram, StudyState, StudyTaskKind};

// =========================================================================
// Request DTOs (요청)
// =========================================================================

/// 학습 목록 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListReq {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub program: Option<String>,
    pub sort: Option<String>,
}

/// 정렬 옵션 (서비스에서 파싱)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyListSort {
    Latest,
    Oldest,
    Alphabetical,
}

impl StudyListSort {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "latest" => Some(Self::Latest),
            "oldest" => Some(Self::Oldest),
            "alphabetical" => Some(Self::Alphabetical),
            _ => None,
        }
    }
}

/// 정답 제출 요청 (JSON Body) - Tagged Union
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubmitAnswerReq {
    Choice { pick: i32 },
    Typing { text: String },
    Voice { text: String },
}

// =========================================================================
// Response DTOs (응답)
// =========================================================================

/// 학습 목록 아이템 (DB Row)
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudySummaryDto {
    pub study_id: i32,
    pub study_idx: String,
    pub program: StudyProgram,     // ⭐ types.rs enum 재사용
    pub title: Option<String>,
    pub state: StudyState,         // ⭐ types.rs enum 재사용
    pub created_at: DateTime<Utc>,
}

/// 학습 목록 전체 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListResp {
    pub list: Vec<StudySummaryDto>,  // 참고: video는 data, lesson은 items
    pub meta: StudyListMeta,
}

/// 학습 문제 상세 정보 (Payload 포함)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyTaskDetailRes {
    pub task_id: i32,
    pub study_id: i32,
    pub kind: StudyTaskKind,       // ⭐ types.rs enum 재사용
    pub seq: i32,
    pub created_at: DateTime<Utc>,
    pub payload: TaskPayload,
}

/// 문제 유형별 페이로드 (Untagged Union)
#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum TaskPayload {
    Choice(ChoicePayload),
    Typing(TypingPayload),
    Voice(VoicePayload),
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChoicePayload {
    pub question: String,
    pub choice_1: String,
    pub choice_2: String,
    pub choice_3: String,
    pub choice_4: String,
    pub audio_url: Option<String>,
    pub image_url: Option<String>,
}

/// 정답 제출 결과
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitAnswerRes {
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
```

**🔑 핵심 포인트**:
- **`crate::types::*` enum 적극 재사용**: `StudyProgram`, `StudyState`, `StudyTaskKind`
- **Tagged Union**: `#[serde(tag = "kind")]`로 요청 다형성 처리
- **Untagged Union**: `#[serde(untagged)]`로 응답 페이로드 구분
- **별도 파싱 enum**: `StudyListSort::parse()`로 정렬 옵션 처리
- **`#[serde(skip_serializing_if = "Option::is_none")]`**: null 필드 생략

**⚠️ 규칙**:
- 문제 유형별 로직은 서비스에서 `match` 처리
- DB enum은 **절대** String으로 받지 않음

---

#### 5️⃣ `src/api/lesson/dto.rs` — 레슨 목록/상세/진도

**역할**: 레슨 목록, 레슨 상세 (아이템 포함), 학습 진도

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// Query DTO - IntoParams 사용
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct LessonListReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
}

// Response DTO - sqlx::FromRow 직접 derive
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonRes {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub lesson_idx: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListRes {
    pub items: Vec<LessonRes>,    // 참고: video는 data, study는 list
    pub meta: LessonListMeta,
}

/// 레슨 아이템 (비디오 or 과제)
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonItemRes {
    pub seq: i32,
    pub kind: crate::types::LessonItemKind,  // ⭐ types.rs enum 재사용
    pub video_id: Option<i64>,
    pub task_id: Option<i64>,
}

/// 학습 진도 조회 응답
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonProgressRes {
    pub percent: i32,
    pub last_seq: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 진도 업데이트 요청
#[derive(Debug, Deserialize, ToSchema)]
pub struct LessonProgressUpdateReq {
    pub percent: i32,
    pub last_seq: Option<i32>,
}
```

**🔑 핵심 포인트**:
- **`IntoParams`**: Query String 파라미터 Swagger 문서화
- **`sqlx::FromRow` 직접 derive**: DTO가 DB Row와 1:1 매핑
- **`crate::types::LessonItemKind`**: enum 재사용으로 타입 안전성 확보

**⚠️ 규칙**:
- lesson은 `#[serde(rename_all = "snake_case")]` 생략됨 (필드명이 이미 snake_case)
- 모든 DTO에 `rename_all` 명시 권장 (안전성)

---

#### 📊 DTO 공통 패턴 요약

| 패턴 | 설명 | 예시 |
|------|------|------|
| Request/Response 분리 | 주석으로 섹션 구분 | `// ===== Request DTOs =====` |
| snake_case 강제 | `#[serde(rename_all = "snake_case")]` | 모든 DTO에 적용 권장 |
| enum 재사용 | `crate::types::*` import | `UserGender`, `StudyTaskKind` 등 |
| 검증 | `validator::Validate` + 속성 | `#[validate(email)]`, `#[validate(range(...))]` |
| Swagger 예시 | `#[schema(example = json!(...))]` | Request DTO에 적용 |
| 날짜 형식 | `#[schema(value_type = String, format = "date")]` | `NaiveDate`, `DateTime<Utc>` |
| JSONB 매핑 | `Json<T>` + `#[schema(value_type = T)]` | `tags: Json<Vec<VideoTagDetail>>` |
| PATCH 패턴 | `Option<T>` + `skip_serializing_if` | `ProfileUpdateReq` |
| Tagged Union | `#[serde(tag = "kind")]` | `SubmitAnswerReq` |
| DB 컬럼 매핑 | `#[sqlx(rename = "...")]` | `VideoProgressRes` |

---

#### ⚠️ 현재 불일치 사항 (개선 권장)

| 항목 | 현재 상태 | 권장 |
|------|----------|------|
| **응답 배열 키** | video: `data`, lesson: `items`, study: `list` | `{ meta, data }`로 통일 |
| **enum 사용** | user/video 일부에서 String 사용 | 모두 `crate::types::*` enum으로 |
| **rename_all** | lesson만 생략 | 모든 DTO에 명시 |
| **IntoParams** | lesson만 사용 | Query DTO 전체에 적용 권장 |

---

#### 📋 dto.rs 표준 템플릿

```rust
// dto.rs (Best Practices Template)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

// DB enum 재사용 (도메인별로 필요한 것만)
use crate::types::*;

// =====================================================================
// Request DTOs (Query / Path / Body)
// =====================================================================

/// (Query) List pagination
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListReq {
    #[serde(default = "default_page")]
    pub page: Option<u32>,
    #[serde(default = "default_per_page")]
    pub per_page: Option<u32>,
}

fn default_page() -> Option<u32> { Some(1) }
fn default_per_page() -> Option<u32> { Some(20) }

/// (Body) Create/Update - Validate 필수
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateReq {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

// =====================================================================
// Response DTOs
// =====================================================================

/// Pagination meta (표준)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageMeta {
    pub total_count: i64,
    pub total_pages: u32,
    pub current_page: u32,
    pub per_page: u32,
}

/// List response (표준: meta + data)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListRes<T> {
    pub meta: PageMeta,
    pub data: Vec<T>,
}
```

---

#### 🔄 DTO ↔ DB 데이터 흐름

```
[HTTP Request]
      ↓
[DTO Request] → validator::Validate 검증
      ↓
[Service] → 비즈니스 로직
      ↓
[Repo] → sqlx 쿼리
      ↓
[DTO Response] ← sqlx::FromRow 또는 수동 매핑
      ↓
[HTTP Response] → serde::Serialize → JSON
```

---

### 1.2 repo.rs
> **Claude 코드 분석 기반** (최종 갱신 2026-04-08)

#### 📁 파일 개요

| 파일 | 라인수 | 구조 | 에러 타입 | 특징 |
|------|--------|------|-----------|------|
| [auth/repo.rs](src/api/auth/repo.rs) | 476 | `struct AuthRepo;` (stateless) | `AppResult` | TX 분리, FOR UPDATE |
| [user/repo.rs](src/api/user/repo.rs) | 286 | standalone functions | `AppResult` | RETURNING, audit log |
| [video/repo.rs](src/api/video/repo.rs) | 254 | `struct VideoRepo;` (stateless) | `AppResult` | QueryBuilder, JSONB |
| [study/repo.rs](src/api/study/repo.rs) | 467 | `struct StudyRepo;` (stateless) | `AppResult` | query_as! macro, Row→DTO 변환 |
| [lesson/repo.rs](src/api/lesson/repo.rs) | 232 | `struct LessonRepo { pool }` ⚠️ | `sqlx::Error` ⚠️ | Upsert, stateful |

#### 1️⃣ Auth Domain ([auth/repo.rs](src/api/auth/repo.rs))

**역할**: 로그인 세션 관리, 사용자 인증 정보 조회

**핵심 패턴**:

```rust
// 1. Internal Row Model (DB 전용 구조체)
#[derive(Debug, sqlx::FromRow)]
pub struct LoginRecord {
    pub user_id: i64,
    pub session_id: String,      // UUID → String 변환
    pub refresh_hash: String,
    pub login_ip: Option<String>, // Inet → String
    pub login_device: String,     // Enum → String
    // ...
}

// 2. TX vs Pool 함수 분리
pub async fn find_login_by_session_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> { /* ... */ }

pub async fn find_login_by_session_id(
    pool: &PgPool,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> { /* ... */ }

// 3. FOR UPDATE Lock 패턴
pub async fn find_login_by_session_id_for_update_tx(
    tx: &mut Transaction<'_, Postgres>,
    session_id: &str,
) -> AppResult<Option<LoginRecord>> {
    sqlx::query_as::<_, LoginRecord>(r#"
        SELECT /* ... */
        FROM public.login
        WHERE login_session_id = CAST($1 AS uuid)
        FOR UPDATE  -- 동시성 제어
    "#)
    .bind(session_id)
    .fetch_optional(&mut **tx)
    .await?
}

// 4. Enum 안전 매핑 (CASE WHEN)
INSERT INTO public.login (/* ... */)
VALUES (
    $1,
    CASE lower($3)
        WHEN 'mobile'  THEN 'mobile'::login_device_enum
        WHEN 'tablet'  THEN 'tablet'::login_device_enum
        WHEN 'desktop' THEN 'desktop'::login_device_enum
        WHEN 'web'     THEN 'desktop'::login_device_enum
        ELSE 'other'::login_device_enum
    END,
    /* ... */
)
```

#### 2️⃣ User Domain ([user/repo.rs](src/api/user/repo.rs))

**역할**: 회원가입, 프로필/설정 CRUD, 감사 로그

**핵심 패턴**:

```rust
// 1. INSERT + RETURNING으로 즉시 응답 구성
pub async fn signup_tx(
    tx: &mut Transaction<'_, Postgres>,
    /* params... */
) -> AppResult<ProfileRes> {
    let res = sqlx::query_as::<_, ProfileRes>(r#"
        INSERT INTO users (/* ... */)
        VALUES ($1, $2, $3::user_language_enum, $4::user_gender_enum, /* ... */)
        RETURNING
            user_id as id,
            user_email as email,
            user_language::TEXT as language,  -- DB Enum → String
            user_gender as gender,             -- DB Enum → Rust Enum
            /* ... */
    "#)
    .bind(/* ... */)
    .fetch_one(&mut **tx)
    .await?;
    Ok(res)
}

// 2. PATCH 업데이트 (COALESCE 패턴)
pub async fn update_profile_tx(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    req: &ProfileUpdateReq,
) -> AppResult<Option<ProfileRes>> {
    sqlx::query_as::<_, ProfileRes>(r#"
        UPDATE users SET
            user_nickname = COALESCE($2, user_nickname),
            user_language = COALESCE($3::user_language_enum, user_language),
            user_country  = COALESCE($4, user_country)
        WHERE user_id = $1
        RETURNING /* ... */
    "#)
    .bind(user_id)
    .bind(req.nickname.as_ref())  // Option → bind
    .bind(req.language.as_ref())
    /* ... */
}

// 3. 감사 로그 (현재 row를 SELECT로 복사)
pub async fn insert_user_log_after_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: Option<i64>,
    user_id: i64,
    action: &str,
    success: bool,
) -> AppResult<()> {
    sqlx::query(r#"
        INSERT INTO public.users_log (
            updated_by_user_id, user_action_log, /* ... */
        )
        SELECT
            $1, CAST($2 AS user_action_log_enum), $3, u.user_id,
            u.user_auth, u.user_state, u.user_email, /* ... */
        FROM public.users u
        WHERE u.user_id = $4
    "#)
    /* ... */
}
```

#### 3️⃣ Video Domain ([video/repo.rs](src/api/video/repo.rs))

**역할**: 비디오 목록/상세 조회, 학습 진도 관리

**핵심 패턴**:

```rust
// 1. QueryBuilder로 동적 쿼리
pub async fn list_videos(
    pool: &PgPool,
    req: &VideoListReq,
) -> AppResult<(Vec<VideoListItem>, i64)> {
    let mut qb = QueryBuilder::new(r#"
        SELECT
            v.video_id::bigint as video_id,  -- INT4 → INT8 캐스팅
            COUNT(*) OVER() as total_count   -- Window Function
        FROM video v
        WHERE 1=1
    "#);

    // 동적 필터 추가
    if let Some(state) = &req.state {
        qb.push(" AND v.video_state = ");
        qb.push_bind(state);
        qb.push("::video_state_enum");
    }

    if let Some(q) = &req.q {
        qb.push(" AND (title ILIKE ");
        qb.push_bind(format!("%{}%", q));
        qb.push(")");
    }

    // 페이징
    qb.push(" LIMIT ").push_bind(req.per_page as i64);
    qb.push(" OFFSET ").push_bind(offset);

    let rows = qb.build().fetch_all(pool).await?;
    /* ... */
}

// 2. JSONB 집계
pub async fn get_video_detail(/* ... */) -> AppResult<Option<VideoDetailRes>> {
    sqlx::query_as::<_, VideoDetailRes>(r#"
        SELECT
            COALESCE(
                jsonb_agg(
                    jsonb_build_object(
                        'key', vt.video_tag_key,
                        'title', vt.video_tag_title
                    )
                ) FILTER (WHERE vt.video_tag_id IS NOT NULL),
                '[]'::jsonb
            ) as tags
        FROM video v
        LEFT JOIN video_tag vt ON /* ... */
        GROUP BY v.video_id
    "#)
}

// 3. Upsert + 조건부 업데이트
pub async fn update_progress(/* ... */) -> AppResult<VideoProgressRes> {
    sqlx::query_as::<_, VideoProgressRes>(r#"
        INSERT INTO video_log (user_id, video_id, video_progress_log, video_completed_log)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, video_id) DO UPDATE SET
            video_progress_log = EXCLUDED.video_progress_log,
            video_completed_log = CASE
                WHEN video_log.video_completed_log = true THEN true  -- 한번 완료면 유지
                ELSE EXCLUDED.video_completed_log
            END
        RETURNING /* ... */
    "#)
}
```

#### 4️⃣ Study Domain ([study/repo.rs](src/api/study/repo.rs))

**역할**: 학습 과제 조회, 채점, 상태 관리

**핵심 패턴**:

```rust
// 1. 내부 Row → DTO 변환 패턴
#[derive(sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i32,
    kind: StudyTaskKind,
    // LEFT JOIN 필드들 (모두 nullable)
    choice_1: Option<String>,
    typing_image_url: Option<String>,
    voice_audio_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn map_to_res(self) -> Option<StudyTaskDetailRes> {
        let payload = match self.kind {
            StudyTaskKind::Choice => TaskPayload::Choice(ChoicePayload { /* ... */ }),
            StudyTaskKind::Typing => TaskPayload::Typing(TypingPayload { /* ... */ }),
            StudyTaskKind::Voice  => TaskPayload::Voice(VoicePayload { /* ... */ }),
        };
        Some(StudyTaskDetailRes { /* ... */ })
    }
}

// 2. sqlx::query_as! 매크로 (타입 명시)
pub async fn find_task_detail(
    pool: &PgPool,
    task_id: i64,
) -> AppResult<Option<StudyTaskDetailRes>> {
    let row = sqlx::query_as!(
        StudyTaskDetailRow,
        r#"
        SELECT
            t.study_task_id::INT AS task_id,
            t.study_task_kind AS "kind!: StudyTaskKind",       -- "!" = non-null 강제
            stc.study_task_choice_1::TEXT AS "choice_1?",      -- "?" = nullable 명시
            stt.study_task_typing_image_url::TEXT AS "typing_image_url?",
            stv.study_task_voice_audio_url::TEXT AS "voice_audio_url?"
        FROM study_task t
        LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
        LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
        LEFT JOIN study_task_voice stv ON t.study_task_id = stv.study_task_id
        WHERE t.study_task_id = $1
        "#,
        task_id as i32  // input 캐스팅
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(r.map_to_res()),
        None => Ok(None),
    }
}

// 3. Count + List 2쿼리 패턴
pub async fn find_open_studies(
    pool: &PgPool,
    page: u32,
    per_page: u32,
) -> AppResult<(Vec<StudySummaryDto>, i64)> {
    // A. Count
    let count: i64 = QueryBuilder::new("SELECT COUNT(*) FROM study WHERE ...")
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    // B. List
    let list = QueryBuilder::new("SELECT /* ... */ FROM study WHERE ...")
        .push(" ORDER BY ...")
        .push(" LIMIT ").push_bind(per_page)
        .push(" OFFSET ").push_bind(offset)
        .build_query_as::<StudySummaryDto>()
        .fetch_all(pool)
        .await?;

    Ok((list, count))
}
```

#### 5️⃣ Lesson Domain ([lesson/repo.rs](src/api/lesson/repo.rs))

**역할**: 레슨 목록/상세, 아이템 조회, 진도 관리

**핵심 패턴**:

```rust
// 1. Upsert (ON CONFLICT)
pub async fn upsert_progress(
    &self,
    lesson_id: i64,
    user_id: i64,
    percent: i32,
    last_seq: Option<i32>,
) -> Result<LessonProgressRes, sqlx::Error> {
    sqlx::query_as::<_, LessonProgressRes>(r#"
        INSERT INTO lesson_progress (
            lesson_id, user_id, lesson_progress_percent,
            lesson_progress_last_item_seq, lesson_progress_last_progress_at
        )
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (lesson_id, user_id) DO UPDATE SET
            lesson_progress_percent = EXCLUDED.lesson_progress_percent,
            lesson_progress_last_item_seq = EXCLUDED.lesson_progress_last_item_seq,
            lesson_progress_last_progress_at = EXCLUDED.lesson_progress_last_progress_at
        RETURNING /* ... */
    "#)
    .bind(lesson_id)
    .bind(user_id)
    .bind(percent)
    .bind(last_seq)  -- Option<i32> 직접 바인딩
    .fetch_one(&self.pool)
    .await
}

// 2. EXISTS 체크
pub async fn exists_lesson(&self, lesson_id: i64) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(r#"
        SELECT EXISTS(SELECT 1 FROM lesson WHERE lesson_id = $1)
    "#)
    .bind(lesson_id)
    .fetch_one(&self.pool)
    .await
}
```

#### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 예시 |
|------|------|--------|----------|
| **TX 분리** | 동일 쿼리의 Pool/TX 버전 | auth | `_tx` suffix |
| **FOR UPDATE** | 동시성 제어 (refresh) | auth | `FOR UPDATE` lock |
| **RETURNING** | INSERT 후 즉시 반환 | user, video, lesson | `RETURNING col AS alias` |
| **COALESCE** | PATCH nullable 처리 | user, video | `COALESCE($2, col)` |
| **QueryBuilder** | 동적 WHERE/ORDER | video, study | `push_bind()` |
| **COUNT OVER()** | 1쿼리 페이징 | video | `COUNT(*) OVER()` |
| **Count+List** | 2쿼리 페이징 | study | 별도 count 쿼리 |
| **query_as!** | 타입 안전 쿼리 | study | `"field!: Type"`, `"field?"` |
| **Row→DTO** | 다형성 변환 | study | `map_to_res()` |
| **ON CONFLICT** | Upsert | video, lesson | `DO UPDATE SET` |
| **CASE WHEN** | Enum 안전 매핑 | auth | `CASE lower($x) WHEN...` |
| **JSONB agg** | 1:N 집계 | video | `jsonb_agg(jsonb_build_object())` |

#### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **에러 타입** | `sqlx::Error` | `AppResult<T>` | lesson_repo |
| **Repo 구조** | `LessonRepo { pool }` (stateful) | `struct XxxRepo;` (stateless) | lesson_repo |
| **TX 책임** | repo가 tx 시작 | service가 tx 관리 | study_repo.submit_grade_tx |
| **nullable 매핑** | `refresh_hash: String` | `Option<String>` | auth_repo.LoginRecord |

#### 📋 표준 템플릿

```rust
// repo.rs (AMK 표준 골격)
use sqlx::{PgPool, Postgres, Transaction, QueryBuilder};
use crate::error::AppResult;

// ✅ Stateless 구조체
pub struct XxxRepo;

impl XxxRepo {
    // =====================================================
    // A. 단건 조회
    // =====================================================
    pub async fn find_by_id(pool: &PgPool, id: i64) -> AppResult<Option<XxxDto>> {
        sqlx::query_as::<_, XxxDto>(r#"
            SELECT xxx_id::bigint as id, /* ... */
            FROM xxx WHERE xxx_id = $1
        "#)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // B. 존재 여부
    // =====================================================
    pub async fn exists(pool: &PgPool, id: i64) -> AppResult<bool> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM xxx WHERE xxx_id = $1)"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // C. 리스트 (Count + List 패턴)
    // =====================================================
    pub async fn list(
        pool: &PgPool,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<XxxDto>, i64)> {
        // 1) Count
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM xxx")
            .fetch_one(pool).await?;

        // 2) List
        let offset = (page as i64 - 1) * per_page as i64;
        let list = sqlx::query_as::<_, XxxDto>(r#"
            SELECT /* ... */ FROM xxx
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#)
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool).await?;

        Ok((list, count))
    }

    // =====================================================
    // D. 쓰기 (TX 버전)
    // =====================================================
    pub async fn create_tx(
        tx: &mut Transaction<'_, Postgres>,
        req: &XxxReq,
    ) -> AppResult<XxxDto> {
        sqlx::query_as::<_, XxxDto>(r#"
            INSERT INTO xxx (col1, col2)
            VALUES ($1, $2::some_enum)
            RETURNING xxx_id::bigint as id, col1, col2
        "#)
        .bind(&req.col1)
        .bind(&req.col2)
        .fetch_one(&mut **tx)
        .await
        .map_err(Into::into)
    }

    // =====================================================
    // E. Upsert
    // =====================================================
    pub async fn upsert(pool: &PgPool, key: i64, val: i32) -> AppResult<XxxDto> {
        sqlx::query_as::<_, XxxDto>(r#"
            INSERT INTO xxx (key_col, val_col)
            VALUES ($1, $2)
            ON CONFLICT (key_col) DO UPDATE SET
                val_col = EXCLUDED.val_col
            RETURNING /* ... */
        "#)
        .bind(key)
        .bind(val)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }
}
```

#### 🔄 데이터 흐름

```
[Service] → pool or tx 전달
      ↓
[Repo Function]
      ↓
sqlx::query_as / query_scalar / query
      ↓
.bind() → Parameter 바인딩
      ↓
.fetch_one/optional/all(&mut **tx) or (pool)
      ↓
Result<T, sqlx::Error> → AppResult<T> 변환
      ↓
[Service]로 반환
```

#### ⚠️ SQLx FromRow 타입 매칭 주의사항

**1. PostgreSQL INT4 → Rust i64 직접 매핑 불가**
- **증상**: `query_as::<_, SomeStruct>` → 500 DB_ERROR
- **원인**: DB 컬럼이 INT4(i32)인데 Rust 구조체 필드를 i64로 선언
- **해결**: SQL에서 `::bigint AS id` 캐스팅 추가
- 이 프로젝트의 PK들(`video_id`, `lesson_id`, `study_id`, `study_task_id`)은 모두 INT4(i32)

**2. SQLx Option\<String\> 컬럼 별칭 규칙**
- **증상**: `ColumnNotFound("detail")` 에러
- **원인**: SQL에서 `AS "detail?"` (따옴표+물음표) 사용 → SQLx가 컬럼명을 `detail?`로 인식
- **해결**: `Option<String>` 필드는 그냥 `AS detail`로 별칭 지정. SQLx가 자동으로 NULL → None 처리
- `"컬럼?"` 문법은 사용하지 말 것

---

### 1.3 service.rs
> **Claude 코드 분석 기반** (최종 갱신 2026-04-08)

#### 📁 파일 개요

| 파일 | 라인수 | 구조 | 주요 역할 | 특징 |
|------|--------|------|----------|------|
| [auth/service.rs](src/api/auth/service.rs) | 569 | `struct AuthService;` (stateless) | 로그인, 토큰 갱신, 로그아웃 | Rate limit, Refresh rotation |
| [user/service.rs](src/api/user/service.rs) | 266 | `struct UserService;` (stateless) | 회원가입, 프로필, 설정 | Auto login, Validation |
| [video/service.rs](src/api/video/service.rs) | 105 | `struct VideoService;` (stateless) | 비디오 목록, 진도 관리 | 단순 CRUD |
| [study/service.rs](src/api/study/service.rs) | 309 | `struct StudyService;` (stateless) | 학습 과제, 채점, 해설 | Enum 파싱, Optional 로깅 |
| [lesson/service.rs](src/api/lesson/service.rs) | 197 | `struct LessonService { repo }` ⚠️ | 레슨 목록, 진도 | Stateful (다른 패턴) |

#### 1️⃣ Auth Domain ([auth/service.rs](src/api/auth/service.rs))

**역할**: 인증 전체 흐름 (로그인/리프레시/로그아웃/비밀번호 재설정)

**핵심 패턴**:

```rust
// 1. 타이밍 공격 방지 (Dummy Hash)
fn dummy_password_hash() -> AppResult<PasswordHash<'static>> {
    static DUMMY_HASH: OnceLock<String> = OnceLock::new();
    let hash_str = DUMMY_HASH.get_or_init(|| {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(b"dummy_password", &salt)
            .expect("argon2 dummy hash should succeed")
            .to_string()
    });
    PasswordHash::new(hash_str).map_err(|_| AppError::Internal("...".into()))
}

// 2. Rate Limiting (Redis INCR + EXPIRE)
let rl_key = format!("rl:login:{}:{}", email, login_ip);
let mut redis_conn = st.redis.get().await?;

let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
if attempts == 1 {
    let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
}
if attempts > st.cfg.rate_limit_login_max {
    return Err(AppError::TooManyRequests("AUTH_429_TOO_MANY_ATTEMPTS".into()));
}

// 3. Refresh Token Rotation + Reuse Detection
pub async fn refresh(st: &AppState, old_refresh_token: &str, ...) -> AppResult<...> {
    let (session_id, incoming_hash) = Self::parse_refresh_token(old_refresh_token)?;

    // DB Lock (FOR UPDATE)
    let mut tx = st.db.begin().await?;
    let login_record = AuthRepo::find_login_by_session_id_for_update_tx(&mut tx, &session_id).await?;

    // Reuse Detection (Critical Security)
    if login_record.refresh_hash != incoming_hash {
        warn!("Refresh token reuse detected! Session: {}", session_id);
        AuthRepo::update_login_state_by_session_tx(&mut tx, &session_id, "compromised").await?;
        tx.commit().await?;

        // Redis 즉시 무효화
        let _ = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await;
        let _ = redis_conn.del(format!("ak:session:{}", session_id)).await;

        return Err(AppError::Conflict("AUTH_409_REUSE_DETECTED".into()));
    }

    // Rotate: 새 토큰 발급
    let (new_refresh_token, new_refresh_hash) = Self::generate_refresh_token_and_hash(&session_id);
    AuthRepo::update_login_refresh_hash_tx(&mut tx, &session_id, &new_refresh_hash).await?;
    tx.commit().await?;

    // Redis Sync (Old 삭제 → New 등록)
    let _: () = redis_conn.del(format!("ak:refresh:{}", login_record.refresh_hash)).await?;
    let _: () = redis_conn.set_ex(format!("ak:refresh:{}", new_refresh_hash), &session_id, ttl).await?;
    // ...
}

// 4. DB TX 후 Redis 반영 패턴 (Login 예시)
pub async fn login(st: &AppState, req: LoginReq, ...) -> AppResult<(LoginRes, Cookie, i64)> {
    // ... validation, password verify ...

    // [DB Transaction]
    let mut tx = st.db.begin().await?;
    AuthRepo::insert_login_record_tx(&mut tx, ...).await?;
    AuthRepo::insert_login_log_tx(&mut tx, ...).await?;
    tx.commit().await?;  // DB 먼저 커밋

    // [Redis Caching] - 커밋 후 실행
    let _: () = redis_conn.set_ex(format!("ak:session:{}", session_id), user_id, ttl).await?;
    let _: () = redis_conn.set_ex(format!("ak:refresh:{}", refresh_hash), &session_id, ttl).await?;

    // [Cookie 생성]
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), refresh_token);
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    // ...
}
```

#### 2️⃣ User Domain ([user/service.rs](src/api/user/service.rs))

**역할**: 회원가입 (자동 로그인 포함), 프로필/설정 CRUD

**핵심 패턴**:

```rust
// 1. DB Unique Violation 감지 (PG 에러 코드)
const PG_UNIQUE_VIOLATION: &'static str = "23505";

fn is_unique_violation(err: &AppError) -> bool {
    if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
        db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
    } else {
        false
    }
}

// 2. 사전 체크 + DB 최종 방어 패턴
pub async fn signup(st: &AppState, req: SignupReq, ...) -> AppResult<...> {
    // [사전 체크]
    if repo::find_user_id_by_email(&st.db, &req.email).await?.is_some() {
        return Err(AppError::Conflict("Email already exists".into()));
    }

    // [DB Insert with Unique Constraint]
    let user = match repo::signup_tx(&mut tx, ...).await {
        Ok(u) => u,
        Err(e) if Self::is_unique_violation(&e) => {
            return Err(AppError::Conflict("Email exists".into()))  // 동시 요청 방어
        },
        Err(e) => return Err(e),
    };
    // ...
}

// 3. Best-Effort 감사 로그 (실패해도 업무 흐름 유지)
if let Err(e) = repo::insert_user_log_after_tx(&mut tx, Some(user.id), user.id, "signup", true).await {
    warn!(error = ?e, user_id = user.id, "Failed to insert signup log");
    // 로그 실패해도 계속 진행
}

// 4. 기본값 Fallback (설정 없으면 기본값 반환)
pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
    let settings = repo::find_users_setting(&st.db, user_id).await?;

    Ok(settings.unwrap_or_else(|| SettingsRes {
        user_set_language: "ko".to_string(),
        user_set_timezone: "UTC".to_string(),
        user_set_note_email: false,
        user_set_note_push: false,
        updated_at: chrono::Utc::now(),
    }))
}
```

#### 3️⃣ Video Domain ([video/service.rs](src/api/video/service.rs))

**역할**: 비디오 목록/상세 조회, 진도율 관리

**핵심 패턴**:

```rust
// 1. 단순 Validation → Repo → Meta 계산 패턴
pub async fn list_videos(st: &AppState, req: VideoListReq) -> AppResult<VideoListRes> {
    // Validation
    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    // Repo 호출 (Data + Total Count)
    let (data, total_count) = VideoRepo::list_videos(&st.db, &req).await?;

    // Meta 계산
    let total_pages = if total_count == 0 { 0 }
    else { (total_count + req.per_page as i64 - 1) / req.per_page as i64 };

    Ok(VideoListRes { meta: VideoListMeta { total_count, total_pages, ... }, data })
}

// 2. 존재 확인 후 기본값 반환 패턴
pub async fn get_video_progress(st: &AppState, user_id: i64, video_id: i64) -> AppResult<VideoProgressRes> {
    // 비디오 존재 확인
    if !VideoRepo::exists_by_id(&st.db, video_id).await? {
        return Err(AppError::NotFound);
    }

    // 진도 조회 → 없으면 기본값
    let progress = VideoRepo::find_progress(&st.db, user_id, video_id).await?;
    Ok(progress.unwrap_or_else(|| VideoProgressRes {
        video_id,
        progress_rate: 0,
        is_completed: false,
        last_watched_at: None,
    }))
}
```

#### 4️⃣ Study Domain ([study/service.rs](src/api/study/service.rs))

**역할**: 학습 과제 조회, 채점, 상태/해설 관리

**핵심 패턴**:

```rust
// 1. Enum 문자열 파싱 (헬퍼 함수)
fn parse_study_program(value: &str) -> Option<StudyProgram> {
    match value {
        "basic_pronunciation" => Some(StudyProgram::BasicPronunciation),
        "basic_word" => Some(StudyProgram::BasicWord),
        "topik_read" => Some(StudyProgram::TopikRead),
        // ...
        _ => None,
    }
}

// 2. 요청 Kind ↔ DB Kind 일치 검증
pub async fn submit_answer(st: &AppState, auth: AuthUser, task_id: i32, req: SubmitAnswerReq) -> AppResult<SubmitAnswerRes> {
    let answer_key = StudyRepo::find_answer_key(&st.db, task_id).await?.ok_or(AppError::NotFound)?;

    let req_kind = match &req {
        SubmitAnswerReq::Choice { .. } => StudyTaskKind::Choice,
        SubmitAnswerReq::Typing { .. } => StudyTaskKind::Typing,
        SubmitAnswerReq::Voice { .. } => StudyTaskKind::Voice,
    };

    if req_kind != answer_key.kind {
        return Err(AppError::BadRequest("Task kind mismatch".into()));
    }
    // ... 채점 로직 ...
}

// 3. Best-Effort 액션 로깅 (실패해도 경고만)
pub async fn get_study_task(st: &AppState, task_id: i32, auth: Option<AuthUser>) -> AppResult<StudyTaskDetailRes> {
    let task = StudyRepo::find_task_detail(&st.db, i64::from(task_id)).await?.ok_or(AppError::NotFound)?;

    if let Some(AuthUser(claims)) = auth {
        if let Err(err) = StudyRepo::log_task_action(&st.db, claims.sub, &claims.session_id, task_id, StudyTaskLogAction::View).await {
            warn!(error = ?err, user_id = claims.sub, task_id, "Failed to log study task view");
            // 로그 실패해도 계속 반환
        }
    }
    Ok(task)
}

// 4. 권한 검증 (시도 횟수 기반)
pub async fn get_task_explain(st: &AppState, auth: AuthUser, task_id: i32) -> AppResult<TaskExplainRes> {
    let try_count = StudyRepo::get_try_count(&st.db, auth.0.sub, task_id).await?;
    if try_count < 1 {
        return Err(AppError::Forbidden);  // 1회 이상 시도해야 해설 조회 가능
    }
    // ...
}
```

#### 5️⃣ Lesson Domain ([lesson/service.rs](src/api/lesson/service.rs))

**역할**: 레슨 목록/상세, 진도 관리

**핵심 패턴**: ⚠️ **Stateful 구조 (다른 도메인과 다름)**

```rust
// ⚠️ 다른 Service와 달리 repo를 필드로 소유
pub struct LessonService {
    repo: LessonRepo,  // AppState가 아닌 직접 소유
}

impl LessonService {
    pub fn new(repo: LessonRepo) -> Self {
        Self { repo }
    }

    pub async fn list_lessons(&self, req: LessonListReq) -> AppResult<LessonListRes> {
        // &self.repo 사용 (AppState 미사용)
        let total_count = self.repo.count_all().await?;  // sqlx::Error 반환 ⚠️
        let items = self.repo.find_all(per_page, offset).await?;
        // ...
    }
}
```

#### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 |
|------|------|--------|------|
| **Rate Limiting** | 브루트포스 방지 | auth, user | `redis.incr()` + `expire()` |
| **Timing Attack 방어** | 로그인 보안 | auth | `dummy_password_hash()` |
| **Refresh Rotation** | 토큰 탈취 감지 | auth | FOR UPDATE + hash 비교 |
| **DB→Redis 순서** | 일관성 보장 | auth, user | `tx.commit()` 후 Redis |
| **Unique Violation** | 중복 방어 | user | `code == "23505"` |
| **Best-Effort Log** | 로깅 실패 허용 | user, study | `warn!()` + 계속 진행 |
| **Default Fallback** | 데이터 없을 때 | video, study, lesson | `unwrap_or_else(|| default)` |
| **Kind Mismatch** | 타입 검증 | study | 요청 Kind ↔ DB Kind |
| **권한 검증** | 접근 제어 | study | `try_count < 1` → Forbidden |

#### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **Refresh Token 포맷** | `session_id:uuid` (auth) vs `random_32bytes` (user) | 통일 필요 | auth vs user |
| **Service 구조** | `LessonService { repo }` (stateful) | `struct XxxService;` (stateless) | lesson |
| **에러 타입** | `sqlx::Error` 직접 반환 | `AppResult<T>` | lesson (via repo) |
| **SADD 누락** | login에서 user_sessions SADD 안함 | SADD 추가 필요 | auth |
| **set_domain 중복** | 2번 호출 | 1번으로 정리 | auth |

#### 📋 표준 템플릿

```rust
// service.rs (AMK 표준 골격)
use crate::{error::{AppError, AppResult}, state::AppState};
use super::{dto::*, repo};
use validator::Validate;
use tracing::warn;

// ✅ Stateless 구조체
pub struct XxxService;

impl XxxService {
    // =====================================================
    // A. 목록 조회 (Validation → Repo → Meta)
    // =====================================================
    pub async fn list(st: &AppState, req: XxxListReq) -> AppResult<XxxListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2. Repo 호출
        let (list, total_count) = repo::find_list(&st.db, &req).await?;

        // 3. Meta 계산
        let total_pages = if total_count == 0 { 0 }
        else { (total_count + req.per_page - 1) / req.per_page };

        Ok(XxxListRes { list, meta: XxxMeta { total_count, total_pages, ... } })
    }

    // =====================================================
    // B. 상세 조회 (Exists Check → Fetch)
    // =====================================================
    pub async fn get_detail(st: &AppState, id: i64) -> AppResult<XxxDetail> {
        let item = repo::find_by_id(&st.db, id).await?.ok_or(AppError::NotFound)?;
        Ok(item)
    }

    // =====================================================
    // C. 쓰기 (TX → Log → Commit)
    // =====================================================
    pub async fn create(st: &AppState, user_id: i64, req: XxxCreateReq) -> AppResult<XxxRes> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

        let mut tx = st.db.begin().await?;

        let result = repo::create_tx(&mut tx, &req).await?;

        // Best-effort logging
        if let Err(e) = repo::log_action_tx(&mut tx, user_id, "create").await {
            warn!(error = ?e, "Failed to log action");
        }

        tx.commit().await?;
        Ok(result)
    }
}
```

#### 🔄 데이터 흐름

```
[Handler] → AppState, AuthUser, Req DTO 추출
      ↓
[Service] → Validation → Rate Limit (Redis) → Business Logic
      ↓
[Repo] → DB Query (TX or Pool)
      ↓
[Service] → DB Commit → Redis Sync → Response 구성
      ↓
[Handler] → HTTP Response (JSON)
```

---

### 1.4 handler.rs
> **Claude 코드 분석 기반** (최종 갱신 2026-04-08)

#### 📁 파일 개요

| 파일 | 라인수 | Extractor 사용 | 주요 역할 | 특징 |
|------|--------|---------------|----------|------|
| [auth/handler.rs](src/api/auth/handler.rs) | 282 | State, HeaderMap, CookieJar, AppJson, AuthUser | 로그인, 토큰 갱신, 로그아웃 | Cookie 직접 관리 |
| [user/handler.rs](src/api/user/handler.rs) | 240 | State, HeaderMap, CookieJar, AppJson, AuthUser | 회원가입, 프로필, 설정 | 201 + Location 헤더 |
| [video/handler.rs](src/api/video/handler.rs) | 117 | State, Query, Path, AppJson, AuthUser | 비디오 목록, 진도 | 완전히 얇은 레이어 |
| [study/handler.rs](src/api/study/handler.rs) | 142 | State, Query, Path, AppJson, AuthUser, OptionalAuthUser | 학습 과제, 채점 | Optional 인증 |
| [lesson/handler.rs](src/api/lesson/handler.rs) | 150 | State, Query, Path, AppJson, AuthUser | 레슨 목록, 진도 | Service 인스턴스화 ⚠️ |

> **추출기 규약**: 요청 바디 파싱은 항상 `AppJson<T>` (src/extract.rs). 응답 직렬화는 `axum::Json` 그대로. 상세는 §1.3 에러 처리 + 아래 login 예제 주석 참조.

#### 1️⃣ Auth Domain ([auth/handler.rs](src/api/auth/handler.rs))

**역할**: 로그인, 토큰 갱신, 로그아웃, 아이디 찾기, 비밀번호 재설정

**핵심 패턴**:

```rust
// 1. Client Context 추출 헬퍼
fn extract_client_ip(headers: &HeaderMap) -> String {
    // x-forwarded-for → x-real-ip → fallback 순서
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = v.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() { return ip.to_string(); }
        }
    }
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = v.trim();
        if !ip.is_empty() { return ip.to_string(); }
    }
    // Fallback (env 설정 가능)
    "127.0.0.1".to_string()
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string())
}

// 2. Login Handler - Service가 Cookie 반환
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "Login successful", body = LoginRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    AppJson(req): AppJson<LoginReq>,  // ⚠️ 요청 파싱은 AppJson, 응답은 Json (아래 참고)
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    // Service가 Cookie까지 생성해서 반환
    let (login_res, cookie, _) = AuthService::login(&st, req, ip, ua).await?;
    let jar = jar.add(cookie);

    Ok((jar, Json(login_res)))
}

// ⚠️ 요청 바디 파싱 추출기는 반드시 `AppJson<T>` 를 쓸 것 (src/extract.rs).
// axum 기본 `Json<T>` extractor 는 JsonRejection 시 `text/plain` 응답을 직접
// 반환해 AMK_API_MASTER §3.4 표준 에러 envelope (code/http_status/message/
// details/trace_id) 과 `x-request-id` 매칭 규약을 우회한다. `AppJson` 은
// 내부적으로 `Json<T>` 를 호출하고 JsonRejection 을 AppError 로 매핑:
//   - JsonDataError (필드 누락/타입 불일치) → AppError::Unprocessable (422)
//   - JsonSyntaxError (깨진 JSON)           → AppError::BadRequest (400)
//   - MissingJsonContentType                → AppError::BadRequest (400)
//   - BytesRejection                        → AppError::BadRequest (400)
// 응답 직렬화(`Json(res)`)는 실패 가능성이 없으므로 `axum::Json` 그대로 사용.

// 3. Refresh Handler - Handler가 Cookie 직접 생성
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    // 쿠키에서 리프레시 토큰 추출
    let refresh_token = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized("Missing refresh token".into()))?;

    let (refresh_res, new_token_str, ttl_secs) =
        AuthService::refresh(&st, &refresh_token, ip, ua).await?;

    // Handler에서 쿠키 직접 설정 (Rotation)
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), new_token_str);
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    refresh_cookie.set_expires(OffsetDateTime::now_utc() + Duration::seconds(ttl_secs));

    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        refresh_cookie.set_domain(domain.clone());
    }

    Ok((jar.add(refresh_cookie), Json(refresh_res)))
}

// 4. Logout Handler - Cookie 만료 설정
pub async fn logout(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), AppError> {
    AuthService::logout(&st, auth_user.sub, &auth_user.session_id, ip, ua).await?;

    // 쿠키 만료 (과거 시간 설정)
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), "");
    refresh_cookie.set_expires(OffsetDateTime::now_utc() - Duration::days(1));
    // ... 기타 속성 설정

    Ok((jar.add(refresh_cookie), StatusCode::NO_CONTENT))
}
```

#### 2️⃣ User Domain ([user/handler.rs](src/api/user/handler.rs))

**역할**: 회원가입 (자동 로그인), 프로필/설정 CRUD

**핵심 패턴**:

```rust
// 1. Signup - 201 + Location 헤더 + Cookie
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = SignupReq,
    responses(
        (status = 201, description = "회원가입 성공 (자동 로그인)", body = SignupRes),
        (status = 409, description = "이메일 중복", body = crate::error::ErrorBody)
    )
)]
pub async fn signup(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<SignupReq>,
) -> AppResult<(CookieJar, (StatusCode, HeaderMap, Json<SignupRes>))> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let (res, refresh_token, refresh_ttl_secs) = UserService::signup(&st, req, ip, ua).await?;

    // Cookie::build() 방식 (다른 스타일)
    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.clone(),
        refresh_token,
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(...)
    .expires(...)
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())  // ⚠️
    .build();

    // Location 헤더 설정 (RESTful)
    let mut resp_headers = HeaderMap::new();
    let location = format!("/users/{}", res.user_id);
    resp_headers.insert(axum::http::header::LOCATION, HeaderValue::from_str(&location)?);

    Ok((jar.add(refresh_cookie), (StatusCode::CREATED, resp_headers, Json(res))))
}

// 2. 얇은 Handler 패턴 (대부분의 엔드포인트)
pub async fn get_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::get_me(&st, auth_user.sub).await?;
    Ok(Json(user))
}

pub async fn update_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<ProfileUpdateReq>,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::update_me(&st, auth_user.sub, req).await?;
    Ok(Json(user))
}
```

#### 3️⃣ Video Domain ([video/handler.rs](src/api/video/handler.rs))

**역할**: 비디오 목록/상세, 학습 진도 관리

**핵심 패턴**: **완전히 얇은 Handler** (Best Practice)

```rust
// 1. Query Parameter + OpenAPI 수동 문서화
#[utoipa::path(
    get,
    path = "/videos",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default 20, max 100)"),
        ("q" = Option<String>, Query, description = "Search query"),
        ("tag" = Option<String>, Query, description = "Filter by tag key"),
        ("state" = Option<String>, Query, description = "Filter by state")
    ),
    responses((status = 200, description = "List of videos", body = VideoListRes)),
    tag = "videos"
)]
pub async fn list_videos(
    State(state): State<AppState>,
    Query(req): Query<VideoListReq>,
) -> AppResult<Json<VideoListRes>> {
    let res = VideoService::list_videos(&state, req).await?;
    Ok(Json(res))
}

// 2. Path Parameter (IdParam DTO 사용)
pub async fn get_video_detail(
    State(state): State<AppState>,
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoDetailRes>> {
    let video = VideoService::get_video_detail(&state, id).await?;
    Ok(Json(video))
}

// 3. 인증 필요 엔드포인트
#[utoipa::path(
    // ...
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn get_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,  // 인증 필수
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoProgressRes>> {
    let progress = VideoService::get_video_progress(&state, auth_user.sub, id).await?;
    Ok(Json(progress))
}
```

#### 4️⃣ Study Domain ([study/handler.rs](src/api/study/handler.rs))

**역할**: 학습 목록, 문제 상세, 정답 제출, 상태/해설 조회

**핵심 패턴**: **OptionalAuthUser** (비로그인 접근 허용)

```rust
// 1. Optional Auth - 비로그인도 접근 가능
pub async fn get_study_task(
    State(state): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,  // ⭐ Optional
    Path(task_id): Path<i32>,
) -> AppResult<Json<StudyTaskDetailRes>> {
    let res = StudyService::get_study_task(&state, task_id, auth).await?;
    Ok(Json(res))
}

// 2. AuthUser 전체 전달 패턴 (다른 도메인과 다름)
pub async fn submit_answer(
    State(state): State<AppState>,
    auth_user: AuthUser,  // ⚠️ 구조 분해 없이 전체 전달
    Path(task_id): Path<i32>,
    Json(req): Json<SubmitAnswerReq>,
) -> AppResult<Json<SubmitAnswerRes>> {
    let res = StudyService::submit_answer(&state, auth_user, task_id, req).await?;
    Ok(Json(res))
}

// 3. Forbidden 응답 가능 엔드포인트
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/explain",
    responses(
        (status = 200, description = "Task Explanation", body = TaskExplainRes),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody)  // 1회 이상 풀어야 조회 가능
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_task_explain(/* ... */) { /* ... */ }
```

#### 5️⃣ Lesson Domain ([lesson/handler.rs](src/api/lesson/handler.rs))

**역할**: 레슨 목록/상세, 아이템, 진도 관리

**핵심 패턴**: ⚠️ **Service 인스턴스화** (다른 도메인과 다름)

```rust
// ⚠️ 매 요청마다 Service 인스턴스 생성
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(req): Query<LessonListReq>,
) -> AppResult<Json<LessonListRes>> {
    // 다른 도메인: VideoService::list_videos(&state, req)
    // Lesson: 인스턴스화 필요
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.list_lessons(req).await?;
    Ok(Json(res))
}

pub async fn get_lesson_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(lesson_id): Path<i64>,
) -> AppResult<Json<LessonProgressRes>> {
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.get_lesson_progress(auth_user.sub, lesson_id).await?;
    Ok(Json(res))
}
```

#### 📊 공통 패턴 요약

| 패턴 | 용도 | 사용처 | 코드 예시 |
|------|------|--------|----------|
| **State<AppState>** | 전역 상태 주입 | 모든 handler | `State(st): State<AppState>` |
| **Json<Req>** | JSON Body 추출 | POST/PUT/PATCH | `Json(req): Json<LoginReq>` |
| **Query(Req)** | Query String 추출 | GET (목록) | `Query(req): Query<VideoListReq>` |
| **Path(id)** | Path Parameter 추출 | 상세/수정/삭제 | `Path(id): Path<i64>` |
| **AuthUser** | 인증 필수 | 보호된 엔드포인트 | `AuthUser(auth): AuthUser` |
| **OptionalAuthUser** | 인증 선택 | 비로그인 허용 | `OptionalAuthUser(auth)` |
| **CookieJar** | 쿠키 관리 | auth, user | `jar: CookieJar` |
| **HeaderMap** | 헤더 추출 | IP/UA 필요 시 | `headers: HeaderMap` |

#### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 | 파일 |
|------|----------|----------|------|
| **헬퍼 중복** | auth, user 각각 정의 | 공통 모듈로 추출 | `api/common/http.rs` |
| **쿠키 생성 책임** | login: service, refresh/signup: handler | 한 곳으로 통일 | auth, user |
| **쿠키 domain 설정** | `unwrap_or_default()` | `if let Some()` 패턴 | user_handler |
| **Service 호출 방식** | lesson만 인스턴스화 | stateless 통일 | lesson_handler |
| **AuthUser 전달** | study만 전체 전달 | 구조 분해 통일 | study_handler |
| **반환 타입** | auth: `Result<_, AppError>`, 기타: `AppResult` | `AppResult` 통일 | auth_handler |

#### 📋 표준 템플릿

```rust
// handler.rs (AMK 표준 골격)
use axum::{
    extract::{State, Path, Query},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{state::AppState, error::AppResult};
use crate::api::auth::extractor::{AuthUser, OptionalAuthUser};
use super::{dto::*, service::XxxService};

// =====================================================================
// 공통 헬퍼 (권장: api/common/http.rs로 분리)
// =====================================================================

fn extract_client_ip(headers: &HeaderMap) -> String {
    headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| headers.get("x-real-ip")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim().to_string()))
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// =====================================================================
// 1. 공개 목록 (인증 불필요)
// =====================================================================
#[utoipa::path(
    get,
    path = "/xxx",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List", body = XxxListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody)
    ),
    tag = "xxx"
)]
pub async fn list_xxx(
    State(st): State<AppState>,
    Query(req): Query<XxxListReq>,
) -> AppResult<Json<XxxListRes>> {
    let res = XxxService::list(&st, req).await?;
    Ok(Json(res))
}

// =====================================================================
// 2. 상세 조회 (선택적 인증)
// =====================================================================
#[utoipa::path(
    get,
    path = "/xxx/{id}",
    params(("id" = i64, Path, description = "Resource ID")),
    responses(
        (status = 200, description = "Detail", body = XxxDetailRes),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    tag = "xxx"
)]
pub async fn get_xxx_detail(
    State(st): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,  // 비로그인 허용
    Path(id): Path<i64>,
) -> AppResult<Json<XxxDetailRes>> {
    let res = XxxService::get_detail(&st, id, auth).await?;
    Ok(Json(res))
}

// =====================================================================
// 3. 생성/수정 (인증 필수)
// =====================================================================
#[utoipa::path(
    post,
    path = "/xxx",
    request_body = XxxCreateReq,
    responses(
        (status = 201, description = "Created", body = XxxRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "xxx"
)]
pub async fn create_xxx(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,  // 구조 분해
    Json(req): Json<XxxCreateReq>,
) -> AppResult<Json<XxxRes>> {
    let res = XxxService::create(&st, auth.sub, req).await?;
    Ok(Json(res))
}

// =====================================================================
// 4. 쿠키 반환 필요 시
// =====================================================================
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> AppResult<(CookieJar, Json<LoginRes>)> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let (res, token, ttl) = AuthService::login(&st, req, ip, ua).await?;
    let cookie = build_refresh_cookie(&st, token, ttl);

    Ok((jar.add(cookie), Json(res)))
}

// 쿠키 빌드 헬퍼 (권장: 공통 모듈)
fn build_refresh_cookie(st: &AppState, token: String, ttl: i64) -> Cookie<'static> {
    let mut c = Cookie::new(st.cfg.refresh_cookie_name.clone(), token);
    c.set_path("/");
    c.set_http_only(true);
    c.set_secure(st.cfg.refresh_cookie_secure);
    c.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    c.set_expires(cookie::time::OffsetDateTime::now_utc() + cookie::time::Duration::seconds(ttl));
    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        c.set_domain(domain.clone());
    }
    c
}
```

#### 🔄 데이터 흐름

```
[HTTP Request]
      ↓
[Extractor] → State, Query, Path, Json, AuthUser 추출
      ↓
[Handler] → 얇은 레이어 (Service 호출만)
      ↓
[Service] → 비즈니스 로직, 검증, TX
      ↓
[Handler] → AppResult<Json<Res>> 또는 (CookieJar, Json<Res>)
      ↓
[HTTP Response] → 200/201/4xx/5xx + JSON Body
```

---

### 1.5 router.rs
> **Claude 코드 분석 기반** (최종 갱신 2026-04-08)

#### 📁 파일 개요

| 파일 | 라인수 | 함수명 | 조립 방식 | 주요 경로 |
|------|--------|--------|----------|----------|
| [auth/router.rs](src/api/auth/router.rs) | 16 | `auth_router()` | nest | /login, /logout, /refresh 등 |
| [user/router.rs](src/api/user/router.rs) | 15 | `user_router()` | merge ⚠️ | /users, /users/me 등 |
| [video/router.rs](src/api/video/router.rs) | 16 | `router()` | nest | /, /{id}, /{id}/progress |
| [study/router.rs](src/api/study/router.rs) | 17 | `router()` | nest | /, /tasks/{id}/answer 등 |
| [lesson/router.rs](src/api/lesson/router.rs) | 17 | `router()` | nest | /, /{id}, /{id}/progress |

#### 1️⃣ Auth Domain ([auth/router.rs](src/api/auth/router.rs))

**역할**: 인증 관련 라우트 (로그인, 로그아웃, 토큰 갱신, 계정 찾기/복구)

```rust
use axum::{routing::post, Router};
use crate::state::AppState;
use super::handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // 세션/토큰 관련
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/logout/all", post(handler::logout_all)) // 모든 기기 로그아웃
        .route("/refresh", post(handler::refresh))

        // 계정 찾기/복구
        .route("/find-id", post(handler::find_id))
        .route("/reset-pw", post(handler::reset_password))
}
```

**특징**:
- 모든 엔드포인트가 **POST** (액션 중심 API)
- 상위에서 `nest("/auth", auth_router())` 방식으로 조립
- 함수명에 도메인 접두사 포함 (`auth_router`)

#### 2️⃣ User Domain ([user/router.rs](src/api/user/router.rs))

**역할**: 사용자 관련 라우트 (회원가입, 프로필, 설정)

```rust
use super::handler::{get_me, get_settings, signup, update_me, update_settings};
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

/// 서브 라우터는 Router<AppState> 반환(프로젝트 규칙)
pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", post(signup))
        .route("/users/me", get(get_me).put(update_me).post(update_me))
        .route("/users/me/settings", get(get_settings).post(update_settings))
}
```

**특징**:
- ⚠️ **절대 경로** 사용 (`/users`, `/users/me`)
- 상위에서 `merge(user_router())` 방식으로 조립 (nest 아님)
- 한 경로에 여러 메서드: `.get(get_me).put(update_me).post(update_me)`
- 함수명에 도메인 접두사 포함 (`user_router`)

#### 3️⃣ Video Domain ([video/router.rs](src/api/video/router.rs))

**역할**: 비디오 관련 라우트 (목록, 상세, 진도)

```rust
use axum::{routing::get, Router};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_videos))
        .route("/{id}", get(handler::get_video_detail))
        .route(
            "/{id}/progress",
            get(handler::get_video_progress).post(handler::update_video_progress),
        )
}
```

**특징**:
- **상대 경로** 사용 (`/`, `/{id}`)
- 상위에서 `nest("/videos", router())` 방식으로 조립
- 진도: GET (조회) + POST (업데이트) 동일 경로
- 함수명 단순 (`router`)

#### 4️⃣ Study Domain ([study/router.rs](src/api/study/router.rs))

**역할**: 학습 관련 라우트 (목록, 문제, 제출, 상태, 해설)

```rust
use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_studies))
        .route("/tasks/{id}", get(handler::get_study_task))
        .route("/tasks/{id}/answer", post(handler::submit_answer))
        .route("/tasks/{id}/status", get(handler::get_task_status))
        .route("/tasks/{id}/explain", get(handler::get_task_explain))
}
```

**특징**:
- **중첩 리소스** 패턴: `/studies` → `/tasks/{id}` → `/answer`, `/status`, `/explain`
- 제출만 POST, 나머지는 GET
- 상위에서 `nest("/studies", router())` 방식으로 조립

#### 5️⃣ Lesson Domain ([lesson/router.rs](src/api/lesson/router.rs))

**역할**: 레슨 관련 라우트 (목록, 상세, 아이템, 진도)

```rust
use axum::{routing::get, Router};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_lessons))
        .route("/{id}", get(handler::get_lesson_detail))
        .route("/{id}/items", get(handler::get_lesson_items))
        .route(
            "/{id}/progress",
            get(handler::get_lesson_progress).post(handler::update_lesson_progress),
        )
}
```

**특징**:
- Video와 동일한 **progress 패턴** (GET + POST)
- 추가 서브 리소스: `/{id}/items`
- 상위에서 `nest("/lessons", router())` 방식으로 조립

#### 📊 공통 패턴 요약

| 패턴 | 설명 | 사용처 |
|------|------|--------|
| **Router<AppState> 반환** | 서브 라우터 표준 시그니처 | 모든 router |
| **상대 경로 + nest** | 프리픽스는 상위에서 관리 | auth, video, study, lesson |
| **절대 경로 + merge** | 라우터가 전체 경로 정의 | user ⚠️ |
| **RESTful 구조** | `/`, `/{id}`, `/{id}/sub` | video, lesson |
| **다중 메서드 체이닝** | `.get(...).post(...)` | progress, me, settings |
| **POST 전용** | 액션 중심 API | auth |
| **중첩 리소스** | `/parent/child/{id}/action` | study |

#### 📋 라우트 전체 매핑

| 도메인 | 최종 경로 | Method | Handler |
|--------|----------|--------|---------|
| **Auth** | `/auth/login` | POST | login |
| | `/auth/logout` | POST | logout |
| | `/auth/logout/all` | POST | logout_all |
| | `/auth/refresh` | POST | refresh |
| | `/auth/find-id` | POST | find_id |
| | `/auth/reset-pw` | POST | reset_password |
| **User** | `/users` | POST | signup |
| | `/users/me` | GET/PUT/POST | get_me, update_me |
| | `/users/me/settings` | GET/POST | get_settings, update_settings |
| **Video** | `/videos` | GET | list_videos |
| | `/videos/{id}` | GET | get_video_detail |
| | `/videos/{id}/progress` | GET/POST | get/update_video_progress |
| **Study** | `/studies` | GET | list_studies |
| | `/studies/tasks/{id}` | GET | get_study_task |
| | `/studies/tasks/{id}/answer` | POST | submit_answer |
| | `/studies/tasks/{id}/status` | GET | get_task_status |
| | `/studies/tasks/{id}/explain` | GET | get_task_explain |
| **Lesson** | `/lessons` | GET | list_lessons |
| | `/lessons/{id}` | GET | get_lesson_detail |
| | `/lessons/{id}/items` | GET | get_lesson_items |
| | `/lessons/{id}/progress` | GET/POST | get/update_lesson_progress |

#### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 |
|------|----------|----------|
| **함수명 불일치** | `auth_router()`, `user_router()` vs `router()` | 하나로 통일 |
| **조립 방식 불일치** | user만 merge + 절대경로 | nest + 상대경로 통일 |
| **PUT vs POST** | update_me에 PUT과 POST 둘 다 | PUT 또는 PATCH 하나만 |

#### 📋 표준 템플릿

```rust
// router.rs (AMK 표준 골격)
use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use super::handler;

/// 서브 라우터는 Router<AppState> 반환
/// 상위에서 nest("/xxx", router())로 조립
pub fn router() -> Router<AppState> {
    Router::new()
        // 목록
        .route("/", get(handler::list))

        // 상세
        .route("/{id}", get(handler::get_detail))

        // 서브 리소스 (조회 + 수정)
        .route(
            "/{id}/progress",
            get(handler::get_progress).post(handler::update_progress),
        )

        // 액션 (POST only)
        .route("/{id}/action", post(handler::do_action))
}
```

#### 🔄 상위 조립 예시

```rust
// api/mod.rs 또는 main.rs
use axum::Router;
use crate::state::AppState;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        // nest 방식 (권장) - 상대 경로 라우터
        .nest("/auth", auth::router::auth_router())
        .nest("/videos", video::router::router())
        .nest("/studies", study::router::router())
        .nest("/lessons", lesson::router::router())

        // merge 방식 - 절대 경로 라우터 (user만 해당)
        .merge(user::router::user_router())

        // 전역 상태 주입 (가장 마지막)
        .with_state(state)
}
```

---

### 1.6 기타 파일들 (Auth 유틸리티)
> **Claude 코드 분석 기반** (최종 갱신 2026-04-08)

#### 📁 파일 개요

| 파일 | 라인수 | 역할 | 주요 함수/타입 |
|------|--------|------|---------------|
| [extractor.rs](src/api/auth/extractor.rs) | 85 | 인증 Extractor | `AuthUser`, `OptionalAuthUser` |
| [jwt.rs](src/api/auth/jwt.rs) | 62 | JWT 토큰 관리 | `Claims`, `create_token`, `decode_token` |
| [password.rs](src/api/auth/password.rs) | 37 | 비밀번호 해싱 | `hash_password`, `verify_password` |
| [token_utils.rs](src/api/auth/token_utils.rs) | 44 | Refresh 토큰 유틸 | `parse_refresh_token_bytes`, `generate_refresh_cookie_value` |

#### 1️⃣ extractor.rs - 인증 Extractor

**역할**: Handler에서 인증 로직을 분리하여 `AuthUser`, `OptionalAuthUser` Extractor 제공

```rust
use axum::extract::{FromRef, FromRequestParts};
use axum::http::{header::AUTHORIZATION, request::Parts};
use crate::api::auth::jwt::{self, Claims};
use crate::error::AppError;
use crate::state::AppState;

// 인증 필수 Extractor
pub struct AuthUser(pub Claims);

// 인증 선택 Extractor (비로그인 허용)
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,  // State에서 AppState 추출 가능해야 함
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl core::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        // AppState에서 JWT secret 추출
        let app_state = AppState::from_ref(state);
        let secret = app_state.cfg.jwt_secret.clone();
        let auth_header = parts.headers.get(AUTHORIZATION).cloned();

        async move {
            // Authorization: Bearer <token> 파싱
            let token = auth_header
                .and_then(|h| h.to_str().ok().map(|s| s.to_string()))
                .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_string()))
                .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

            // JWT 검증
            let claims = jwt::decode_token(&token, &secret)
                .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

            Ok(AuthUser(claims))
        }
    }
}

// OptionalAuthUser: 헤더 없으면 Ok(None), 있으면 검증
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    fn from_request_parts(parts: &mut Parts, state: &S) -> impl core::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let app_state = AppState::from_ref(state);
        let secret = app_state.cfg.jwt_secret.clone();
        let auth_header = parts.headers.get(AUTHORIZATION).cloned();

        async move {
            let Some(header_value) = auth_header else {
                return Ok(OptionalAuthUser(None));  // 헤더 없으면 None
            };
            // 헤더 있으면 검증 진행...
            let claims = jwt::decode_token(token, &secret)?;
            Ok(OptionalAuthUser(Some(AuthUser(claims))))
        }
    }
}
```

**특징**:
- `FromRef<S>` 패턴으로 State에서 AppState 추출
- Handler에서 인증 코드 완전 제거 가능
- `OptionalAuthUser`: 공개 API + 선택적 사용자 컨텍스트에 적합

#### 2️⃣ jwt.rs - JWT 토큰 관리

**역할**: Access Token 생성/검증, Claims 구조체 정의

```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use crate::api::auth::dto::AccessTokenRes;
use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,           // User ID
    pub session_id: String, // Session ID (로그아웃 시 폐기용)
    pub exp: i64,           // Expiration time (Unix timestamp)
    pub iat: i64,           // Issued at
    pub iss: String,        // Issuer ("amk" 고정)
}

pub fn create_token(
    user_id: i64,
    session_id: &str,
    ttl_minutes: i64,
    secret: &str,
) -> AppResult<AccessTokenRes> {
    let now = OffsetDateTime::now_utc();
    let duration = Duration::minutes(ttl_minutes);
    let expires_in_dt = now + duration;

    let claims = Claims {
        sub: user_id,
        session_id: session_id.to_string(),
        exp: expires_in_dt.unix_timestamp(),
        iat: now.unix_timestamp(),
        iss: "amk".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    // ISO 8601 포맷 (프론트엔드 편의용)
    let expires_at_str = expires_in_dt.format(&Rfc3339)?;

    Ok(AccessTokenRes {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: ttl_minutes * 60,  // 초 단위
        expires_at: expires_at_str,
    })
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
```

**특징**:
- `session_id` 포함 → 세션 기반 폐기(로그아웃) 지원
- `iss: "amk"` 고정 발급자
- `AccessTokenRes`: `token_type`, `expires_in`, `expires_at` 포함 (OAuth 2.0 스타일)

#### 3️⃣ password.rs - 비밀번호 해싱

**역할**: Argon2id 기반 비밀번호 해싱/검증

```rust
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use crate::error::{AppError, AppResult};

/// 비밀번호 해싱
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id 설정 (메모리 19MB, 2 iterations, 1 parallelism)
    let params = Params::new(19_456, 2, 1, None)
        .map_err(|e| AppError::Internal(format!("Failed to create Argon2 params: {}", e)))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();

    Ok(password_hash)
}

/// 비밀번호 검증
#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?;

    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(valid)
}
```

**특징**:
- **Argon2id** 알고리즘 사용 (현재 권장 표준)
- 파라미터 중앙 관리 (메모리 19MB, 2 iterations, 1 parallelism)
- `OsRng`로 보안 난수 Salt 생성

#### 4️⃣ token_utils.rs - Refresh 토큰 유틸

**역할**: Refresh 토큰 생성/파싱

```rust
use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine as _;
use percent_encoding::percent_decode_str;
use rand::RngCore;
use uuid::Uuid;
use crate::error::AppError;

/// Refresh 토큰 파싱 (다양한 포맷 허용)
pub fn parse_refresh_token_bytes(s: &str) -> Result<Vec<u8>, AppError> {
    // 0) URL 디코딩
    let decoded = percent_decode_str(s)
        .decode_utf8()
        .map_err(|_| AppError::Unauthorized("Invalid refresh token format".into()))?;
    let ss = decoded.as_ref();

    // 1) UUID 허용
    if let Ok(u) = Uuid::parse_str(ss) {
        return Ok(u.as_bytes().to_vec());
    }
    // 2) base64url no-pad
    if let Ok(b) = URL_SAFE_NO_PAD.decode(ss) {
        return Ok(b);
    }
    // 3) base64url with pad
    if let Ok(b) = URL_SAFE.decode(ss) {
        return Ok(b);
    }
    // 4) 일반 base64
    if let Ok(b) = STANDARD.decode(ss) {
        return Ok(b);
    }

    Err(AppError::Unauthorized("Invalid refresh token format".into()))
}

/// Refresh 토큰 생성 (랜덤 32바이트)
pub fn generate_refresh_cookie_value() -> (String, [u8; 32]) {
    let mut raw = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut raw);
    let cookie_val = URL_SAFE_NO_PAD.encode(raw);
    (cookie_val, raw)  // (쿠키값, raw 바이트)
}
```

**특징**:
- **Opaque 토큰**: 랜덤 32바이트 → base64url 인코딩
- 다양한 포맷 허용 (UUID, base64url, base64) - 마이그레이션 호환성
- Raw 바이트 반환 → SHA256 해시 후 DB 저장 가능

#### 📊 공통 패턴 요약

| 패턴 | 파일 | 설명 |
|------|------|------|
| **중앙 집중화** | 모두 | 인증/보안 로직을 유틸로 분리, Service에서 직접 구현 금지 |
| **FromRef 패턴** | extractor.rs | State에서 AppState 추출하여 config 접근 |
| **Opaque Token** | token_utils.rs | Refresh는 랜덤 바이트, Access는 JWT |
| **파라미터 고정** | password.rs | Argon2 설정을 한 곳에서 관리 |
| **Claims 확장** | jwt.rs | session_id 포함으로 세션 폐기 지원 |

#### ⚠️ 현재 불일치/개선 필요 사항

| 이슈 | 현재 상태 | 권장 표준 |
|------|----------|----------|
| **JWT Validation** | `Validation::default()` | 알고리즘/issuer 명시적 검증 |
| **verify_password** | `Argon2::default()` | hash_password와 동일 파라미터 사용 |
| **Refresh 토큰 스펙** | 다양한 포맷 허용 | 단일 포맷 (opaque + hash) 통일 |

#### 📋 Auth 유틸리티 사용 흐름

```
[로그인 요청]
      ↓
[password.rs] → hash_password() 비교 또는 verify_password()
      ↓
[jwt.rs] → create_token() → AccessTokenRes
      ↓
[token_utils.rs] → generate_refresh_cookie_value() → (쿠키, raw)
      ↓
[DB/Redis] → SHA256(raw) 저장
      ↓
[HTTP Response] → Access Token (Body) + Refresh Token (Cookie)

---

[API 요청 with Token]
      ↓
[extractor.rs] → AuthUser 또는 OptionalAuthUser
      ↓
[jwt.rs] → decode_token() → Claims { sub, session_id, ... }
      ↓
[Handler] → Claims.sub (user_id) 사용
```

---

## 2. 프론트엔드 패턴 (React/TypeScript)

백엔드 레이어(`dto.rs`, `repo.rs`, `service.rs`, `handler.rs`)와 1:1로 대응되는 **Category-First** 아키텍처를 따른다.

**레이어 대응표:**

| 백엔드 (Rust/Axum) | 프론트엔드 (React/TS) | 역할 |
|-------------------|----------------------|------|
| `dto.rs` | `types.ts` | 요청/응답 타입 정의 |
| `repo.rs` | `*_api.ts` | API 호출 함수 |
| `service.rs` | `hook/*.ts` | 비즈니스 로직 (TanStack Query) |
| `handler.rs` | `page/*.tsx` | UI 조립 및 렌더링 |

**규칙:**
1. **Backend Parity:** DTO는 `snake_case`를 유지하여 변환 비용을 없앤다
2. **Auth Strategy:** Refresh Token은 HttpOnly Cookie, Access Token은 메모리(Zustand persist)에 저장
3. **Tech Stack:** `Axios` + `TanStack Query` + `Zustand` + `react-hook-form` + `Zod` + `shadcn/ui`

**디렉터리 구조:**
```
frontend/src/
├── api/
│   └── client.ts              # Axios 클라이언트 + 401 refresh interceptor
├── app/
│   └── routes.tsx             # React Router 라우팅 정의
├── category/                  # 도메인별 폴더
│   ├── auth/
│   │   ├── page/              # login_page.tsx, signup_page.tsx 등
│   │   ├── hook/              # use_login.ts, use_logout.ts 등
│   │   ├── components/        # logout_button.tsx
│   │   ├── types.ts           # Zod schema + 타입
│   │   └── auth_api.ts        # API 함수들
│   ├── video/
│   │   ├── page/
│   │   ├── hook/
│   │   ├── components/
│   │   ├── types.ts
│   │   └── video_api.ts
│   └── ...
├── components/
│   ├── ui/                    # shadcn/ui 컴포넌트
│   ├── layout/
│   └── shared/
├── hooks/
│   └── use_auth_store.ts      # Zustand + persist (전역 인증 상태)
└── routes/
    └── private_route.tsx      # PrivateRoute 가드
```

---

### 2.1 types.ts (Zod 스키마 & 타입 정의)

백엔드 `dto.rs`와 1:1 대응. **snake_case 필드명 유지**, Zod로 런타임 검증 + 타입 추론.

#### 파일 개요

| 파일 | 주요 타입 | 비고 |
|------|----------|------|
| `category/auth/types.ts` | `LoginReq`, `LoginRes`, `SignupReq`, `SignupRes` | 공용 Enum 포함 (`UserAuth`, `UserGender`) |
| `category/video/types.ts` | `VideoListReq`, `VideoListRes`, `VideoDetail`, `VideoProgressRes` | List/Detail/Progress 분리 |
| `category/user/types.ts` | `UserDetail`, `UpdateUserReq`, `SettingsRes` | Auth에서 Enum import |

#### 코드 예시: `category/auth/types.ts`

```typescript
import { z } from "zod";

// ==========================================
// 공통 Enum
// ==========================================
export const userAuthSchema = z.enum(["HYMN", "admin", "manager", "learner"]);
export type UserAuth = z.infer<typeof userAuthSchema>;

export const userGenderSchema = z.enum(["none", "male", "female", "other"]);
export type UserGender = z.infer<typeof userGenderSchema>;

// ==========================================
// 액세스 토큰 응답 (공통)
// ==========================================
export const accessTokenResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});
export type AccessTokenRes = z.infer<typeof accessTokenResSchema>;

// ==========================================
// 로그인
// ==========================================
export const loginReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(72),
  device: z.string().optional(),
  browser: z.string().optional(),
});
export type LoginReq = z.infer<typeof loginReqSchema>;

export const loginResSchema = z.object({
  user_id: z.number().int(),
  access: accessTokenResSchema,
  session_id: z.string(),
});
export type LoginRes = z.infer<typeof loginResSchema>;

// ==========================================
// 회원가입
// ==========================================
export const signupReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8).max(72),
  name: z.string().min(1).max(50),
  nickname: z.string().min(1).max(100),
  terms_service: z.boolean(),
  terms_personal: z.boolean(),
  language: z.string().min(2).max(2),
  country: z.string().min(2).max(50),
  birthday: z.string().date(),  // YYYY-MM-DD
  gender: userGenderSchema,
});
export type SignupReq = z.infer<typeof signupReqSchema>;
```

#### 코드 예시: `category/video/types.ts`

```typescript
import { z } from "zod";

// Request DTO
export const videoListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(),
  tag: z.string().optional(),
});
export type VideoListReq = z.infer<typeof videoListReqSchema>;

// Response DTO - List Meta (페이지네이션)
export const videoListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

// Response DTO - List Item
export const videoListItemSchema = z.object({
  video_id: z.number().int(),
  video_idx: z.string(),
  title: z.string().nullable(),        // Option<String> → nullable
  thumbnail_url: z.string().nullable(),
  state: z.string(),
  tags: z.array(z.string()),           // 목록에서는 문자열 배열
  created_at: z.string().datetime(),
});

// Response DTO - List 전체
export const videoListResSchema = z.object({
  meta: videoListMetaSchema,
  data: z.array(videoListItemSchema),
});
export type VideoListRes = z.infer<typeof videoListResSchema>;

// Response DTO - Progress
export const videoProgressResSchema = z.object({
  video_id: z.number().int(),
  progress_rate: z.number().int(),
  is_completed: z.boolean(),
  last_watched_at: z.string().datetime().nullable(),
});
export type VideoProgressRes = z.infer<typeof videoProgressResSchema>;
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **snake_case 유지** | 백엔드 DTO와 동일한 필드명 사용 (camelCase 변환 금지) |
| **Zod + infer** | 스키마 정의 → `z.infer<>` 로 타입 추출 |
| **nullable vs optional** | Rust `Option<T>` → `.nullable()`, 선택 필드 → `.optional()` |
| **Enum 중앙화** | 공용 Enum은 `auth/types.ts`에 정의, 다른 도메인에서 import |
| **List 표준 구조** | `{ meta: { total_count, ... }, data: T[] }` |

---

### 2.2 *_api.ts (API 함수)

백엔드 `repo.rs`와 1:1 대응. `request()` 래퍼를 사용하여 API 엔드포인트 호출.

#### 파일 개요

| 파일 | 주요 함수 | HTTP 메서드 |
|------|----------|-------------|
| `category/auth/auth_api.ts` | `login`, `signup`, `logout`, `findId`, `resetPassword` | POST |
| `category/video/video_api.ts` | `getVideoList`, `getVideoDetail`, `getVideoProgress`, `updateVideoProgress` | GET, POST |
| `category/user/user_api.ts` | `getUserMe`, `updateUserMe`, `getUserSettings`, `updateUserSettings` | GET, POST |

#### 코드 예시: `category/auth/auth_api.ts`

```typescript
import { request } from "@/api/client";
import type {
  FindIdReq,
  LoginReq,
  LoginRes,
  ResetPasswordReq,
  SignupReq,
  SignupRes,
} from "@/category/auth/types";

export const login = (data: LoginReq) => {
  return request<LoginRes>("/auth/login", {
    method: "POST",
    data,
  });
};

export const signup = (data: SignupReq) => {
  // RESTful: 사용자 생성은 /users
  return request<SignupRes>("/users", {
    method: "POST",
    data,
  });
};

export const findId = (data: FindIdReq) => {
  return request<void>("/auth/find-id", {
    method: "POST",
    data,
  });
};

export const logout = () => {
  // 토큰 헤더는 client interceptor가 자동 주입
  return request<void>("/auth/logout", {
    method: "POST",
  });
};
```

#### 코드 예시: `category/video/video_api.ts`

```typescript
import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
  VideoProgressUpdateReq,
  VideoProgressRes,
} from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}) => {
  return request<VideoListRes>("/videos", {
    params,
  });
};

export const getVideoDetail = (id: number) => {
  return request<VideoDetail>(`/videos/${id}`);
};

export const getVideoProgress = (videoId: number) => {
  return request<VideoProgressRes>(`/videos/${videoId}/progress`);
};

export const updateVideoProgress = (videoId: number, data: VideoProgressUpdateReq) => {
  return request<void>(`/videos/${videoId}/progress`, {
    method: "POST",
    data,
  });
};
```

#### 코드 예시: `category/user/user_api.ts`

```typescript
import type {
  SettingsRes,
  SettingsUpdateReq,
  UpdateUserReq,
  UserDetail,
} from "@/category/user/types";
import { request } from "@/api/client";

export const getUserMe = () => {
  return request<UserDetail>("/users/me");
};

export const updateUserMe = (data: UpdateUserReq) => {
  return request<void>("/users/me", {
    method: "POST",
    data,
  });
};

export const getUserSettings = () => {
  return request<SettingsRes>("/users/me/settings");
};

export const updateUserSettings = (data: SettingsUpdateReq) => {
  return request<void>("/users/me/settings", {
    method: "POST",
    data,
  });
};
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **request<T> 래퍼** | 제네릭으로 응답 타입 강제 |
| **GET은 params** | `{ params: { page, per_page, ... } }` |
| **POST는 data** | `{ method: "POST", data: { ... } }` |
| **void 반환** | 204 No Content 응답 시 `request<void>` |
| **경로 파라미터** | 템플릿 리터럴 사용 `` `/videos/${id}` `` |

---

### 2.3 hook/*.ts (TanStack Query 훅)

백엔드 `service.rs`와 1:1 대응. `useMutation` (변경) / `useQuery` (조회) 패턴.

#### 파일 개요

| 파일 | 훅 이름 | Query/Mutation | 특징 |
|------|--------|----------------|------|
| `auth/hook/use_login.ts` | `useLogin` | Mutation | Store 업데이트, 네비게이션 |
| `auth/hook/use_logout.ts` | `useLogout` | Mutation | Store 클리어 |
| `video/hook/use_video_list.ts` | `useVideoList` | Query | staleTime 5분 |
| `video/hook/use_video_detail.ts` | `useVideoDetail` | Query | videoId 기반 |

#### 코드 예시: `category/auth/hook/use_login.ts` (Mutation)

```typescript
import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { LoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { login } from "../auth_api";

// 에러 코드별 메시지 매핑
const statusMessageMap: Record<number, string> = {
  400: "입력 형식을 확인해주세요.",
  401: "이메일 또는 비밀번호가 일치하지 않습니다.",
  403: "접근이 차단된 계정입니다. 관리자에게 문의하세요.",
  429: "너무 많은 시도가 있었습니다. 잠시 후 다시 시도해주세요.",
  500: "서버 오류가 발생했습니다.",
};

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return statusMessageMap[error.status] ?? error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLogin = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);   // Store 업데이트
      toast.success("로그인 성공!");
      navigate("/");                         // 홈으로 이동
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
```

#### 코드 예시: `category/video/hook/use_video_list.ts` (Query)

```typescript
import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { VideoListReq } from "@/category/video/types";

import { getVideoList } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useVideoList = (params: VideoListReq) => {
  const query = useQuery({
    queryKey: ["videos", params],    // 캐시 키: ["videos", { page, per_page, ... }]
    queryFn: () => getVideoList(params),
    staleTime: 1000 * 60 * 5,        // 5분간 fresh 유지
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **Query Key 규칙** | `["도메인", params]` 형태 (ex: `["videos", { page: 1 }]`) |
| **staleTime** | 데이터 fresh 유지 시간 (기본 5분) |
| **onSuccess/onError** | Mutation 성공/실패 시 부수 효과 처리 |
| **Store 연동** | `useAuthStore.getState()` 로 Zustand 액션 호출 |
| **에러 메시지 매핑** | HTTP 상태 코드별 사용자 친화적 메시지 |
| **toast 알림** | `sonner` 라이브러리로 토스트 알림 |

---

### 2.4 page/*.tsx (페이지 컴포넌트)

백엔드 `handler.rs`와 1:1 대응. **조립(Composition)만 담당**, 로직은 훅에 위임.

#### 파일 개요

| 파일 | 컴포넌트 | 사용 훅 | 특징 |
|------|---------|--------|------|
| `auth/page/login_page.tsx` | `LoginPage` | `useLogin`, `useForm` | RHF + Zod + shadcn |
| `video/page/video_list_page.tsx` | `VideoListPage` | `useVideoList` | 목록 렌더링 |
| `user/page/my_page.tsx` | `MyPage` | `useUserMe` | 내 정보 조회 |

#### 코드 예시: `category/auth/page/login_page.tsx`

```tsx
import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link } from "react-router-dom";

import {
  Card, CardContent, CardDescription, CardHeader, CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Form, FormControl, FormField, FormItem, FormLabel, FormMessage,
} from "@/components/ui/form";
import { loginReqSchema, type LoginReq } from "@/category/auth/types";

import { useLogin } from "../hook/use_login";

export function LoginPage() {
  const loginMutation = useLogin();

  const form = useForm<LoginReq>({
    resolver: zodResolver(loginReqSchema),  // Zod 스키마로 검증
    mode: "onChange",
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = (values: LoginReq) => {
    loginMutation.mutate(values);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>로그인</CardTitle>
          <CardDescription>
            다시 돌아오신 것을 환영합니다.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 이메일 입력 */}
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>이메일</FormLabel>
                    <FormControl>
                      <Input
                        type="email"
                        placeholder="email@example.com"
                        autoComplete="email"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 비밀번호 입력 */}
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>비밀번호</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="비밀번호를 입력하세요"
                        autoComplete="current-password"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 하단 링크 */}
              <div className="flex items-center justify-between text-sm">
                <Link to="/signup" className="text-primary hover:underline">
                  계정이 없으신가요? 회원가입
                </Link>
                <Link to="/find-id" className="text-muted-foreground hover:underline">
                  아이디/비밀번호 찾기
                </Link>
              </div>

              {/* 제출 버튼 */}
              <Button
                type="submit"
                className="w-full"
                disabled={loginMutation.isPending}
              >
                {loginMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    로그인 중...
                  </>
                ) : (
                  "로그인"
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **RHF + Zod** | `zodResolver(schema)` 로 폼 검증 자동화 |
| **shadcn/ui Form** | `Form`, `FormField`, `FormItem` 컴포넌트 조합 |
| **Mutation 상태** | `isPending` 으로 로딩 상태 표시 |
| **페이지는 조립만** | 로직은 훅(`useLogin`)에 위임 |
| **반응형 레이아웃** | `max-w-md`, `min-h-screen` 등 Tailwind 유틸리티 |

---

### 2.5 공용 인프라 (Shared Infrastructure)

도메인 간 공유되는 핵심 모듈들.

#### 파일 개요

| 파일 | 역할 | 주요 기능 |
|------|------|----------|
| `api/client.ts` | HTTP 클라이언트 | Axios + 401 refresh interceptor |
| `hooks/use_auth_store.ts` | 전역 인증 상태 | Zustand + persist |
| `app/routes.tsx` | 라우팅 정의 | Public/Private 라우트 분리 |
| `routes/private_route.tsx` | 인증 가드 | 로그인 필수 라우트 보호 |

#### 코드 예시: `api/client.ts` (핵심 부분)

```typescript
import axios, { type AxiosRequestConfig } from "axios";

import type { LoginRes } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "/api";

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

export const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,  // Refresh Cookie 전송 필수
});

// 401 Silent Refresh Interceptor
api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    if (
      error.response?.status === 401 &&
      originalRequest &&
      !originalRequest._retry
    ) {
      originalRequest._retry = true;
      try {
        // Refresh 요청 (Cookie 기반)
        const refreshResponse = await api.post("/auth/refresh", {});
        const loginData = refreshResponse.data as LoginRes;
        const newToken = `Bearer ${loginData.access.access_token}`;

        // 헤더 갱신
        api.defaults.headers.common["Authorization"] = newToken;
        originalRequest.headers["Authorization"] = newToken;

        // Store 업데이트
        useAuthStore.getState().login(loginData);

        // 원 요청 재시도
        return api(originalRequest);
      } catch (refreshError) {
        // Refresh 실패 시 로그아웃
        useAuthStore.getState().logout();
        window.location.href = "/login";
        return Promise.reject(refreshError);
      }
    }
    return Promise.reject(error);
  }
);

// 제네릭 request 함수
export async function request<T>(
  path: string,
  options: Omit<AxiosRequestConfig, "url"> = {}
): Promise<T> {
  const response = await api.request<T>({ url: path, ...options });
  if (response.status === 204 || response.data === "") {
    return undefined as T;
  }
  return response.data;
}
```

#### 코드 예시: `hooks/use_auth_store.ts`

```typescript
import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { LoginRes } from "@/category/auth/types";
import type { SignupRes } from "@/category/user/types";

type StoredUser = Omit<SignupRes, "access" | "session_id"> | Pick<LoginRes, "user_id">;

type AuthState = {
  user: StoredUser | null;
  accessToken: string | null;
  isLoggedIn: boolean;
  login: (data: LoginRes | SignupRes) => void;
  logout: () => void;
};

const initialState: Pick<AuthState, "user" | "accessToken" | "isLoggedIn"> = {
  user: null,
  accessToken: null,
  isLoggedIn: false,
};

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      ...initialState,
      login: (data) => {
        set({
          user: "email" in data
            ? { user_id: data.user_id, email: data.email, name: data.name, nickname: data.nickname }
            : { user_id: data.user_id },
          accessToken: data.access.access_token,
          isLoggedIn: true,
        });
      },
      logout: () => {
        set({ ...initialState });
        useAuthStore.persist.clearStorage();
      },
    }),
    {
      name: "auth-storage",  // localStorage 키
    }
  )
);
```

#### 코드 예시: `routes/private_route.tsx`

```tsx
import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/hooks/use_auth_store";

export default function PrivateRoute() {
  const user = useAuthStore((state) => state.user);

  // 유저 정보가 없으면 로그인 페이지로 리다이렉트
  if (!user) {
    return <Navigate to="/login" replace />;
  }

  // 있으면 자식 컴포넌트 렌더링
  return <Outlet />;
}
```

#### 코드 예시: `app/routes.tsx`

```tsx
import { Route, Routes } from "react-router-dom";

import { LoginPage } from "@/category/auth/page/login_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { VideoListPage } from "@/category/video/page/video_list_page";
import { MyPage } from "@/category/user/page/my_page";
import PrivateRoute from "@/routes/private_route";

export function AppRoutes() {
  return (
    <Routes>
      {/* 누구나 접근 가능 (Public) */}
      <Route path="/login" element={<LoginPage />} />
      <Route path="/signup" element={<SignupPage />} />
      <Route path="/videos" element={<VideoListPage />} />

      {/* 로그인한 사람만 접근 가능 (Private) */}
      <Route element={<PrivateRoute />}>
        <Route path="/user/me" element={<MyPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}
```

#### 핵심 패턴

| 패턴 | 설명 |
|------|------|
| **401 Silent Refresh** | 토큰 만료 시 자동 갱신 후 원 요청 재시도 |
| **Zustand persist** | `localStorage`에 인증 상태 유지 (새로고침 대응) |
| **PrivateRoute 가드** | `<Outlet />` 패턴으로 자식 라우트 보호 |
| **withCredentials** | Refresh Token Cookie 전송을 위해 필수 |
| **ApiError 클래스** | HTTP 상태 코드 기반 에러 핸들링 |

---

### 2.6 프론트엔드 데이터 흐름 (Data Flow)

```
[사용자 액션]
      ↓
[Page Component] → useForm() + zodResolver(schema)
      ↓
[Custom Hook] → useMutation() / useQuery()
      ↓
[*_api.ts] → request<T>() 호출
      ↓
[api/client.ts] → axios + interceptor
      ↓
[Backend API]
      ↓
[Response]
      ↓
[Custom Hook] → onSuccess: Store 업데이트, toast 알림
      ↓
[Page Component] → UI 반영 (isPending, data, error)
```

---

