---
title: AMK_API_MASTER — Amazing Korean API  Master Spec
updated: 2026-01-11
owner: HYMN Co., Ltd. (Amazing Korean)
audience: server / database / backend / frontend / lead / LLM assistant
---

## ※ AMK_API_MASTER — Amazing Korean API Master Spec ※

> 이 문서는 **Amazing Korean server / database / backend / frontend / web&app 전체 스펙·규칙·로드맵의 단일 기준(Single Source of Truth)** 이다.

> 과거 문서들(`AMK_Feature_Roadmap.md`, `AMK_PROJECT_JOURNAL.md`, `AMK_ENGINEERING_GUIDE.md`, `AMK_API_OVERVIEW_FULL.md`, `README_for_assistant.md`)에 흩어져 있던 내용을 통합·정리한 버전

> **이 문서와 다른 문서가 충돌할 경우 이 문서를 정답으로 간주한다.**

---

## 0. 문서 메타 & 사용 방법

### 0.1 목적

- Amazing Korean server / database / backend / frontend / web&app 대한:
  - **기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 완료 상태)**
  - **공통 규칙 (에러 / 시간 / 인증 / 페이징 / 응답 래퍼 등)**
  - **개발 / 작업 방식 (엔지니어링 가이드)**
  - **LLM 협업 규칙 (패치 방식, 템플릿 사용법)**
  - **Open Questions & 설계 TODO**
- 을 한 파일에서 관리하기 위함.

### 0.2 사용 원칙

- **스펙 / 기능 / 엔드포인트를 변경할 때는 항상 이 파일을 먼저 수정**한다.
- 코드/마이그레이션/테스트를 변경한 뒤에는, 여기의 관련 섹션(Phase 표, 규칙, TODO)을 반드시 갱신한다.
- 과거 md 문서들은 모두 **참고용 아카이브**이며, 새로운 정보는 **여기에만 적는다**.

### 0.3 관련 파일

- DB 스키마: `amk_schema_patched.sql`
- 패치 프롬프트 템플릿: `GEMINI_PROMPT_TEMPLATE.md`
- (선택) 이 문서는 레포 내 `docs/AMK_API_MASTER.md` 경로에 위치하는 것을 기본으로 한다.

---

## 1. 프로젝트 개요 & 목표

### 1.1 서비스 개요

- 브랜드: **Amazing Korean**
- 주요 타겟:
  - EPS-TOPIK / TOPIK 준비생 ( 해외 한국어 학습자 중심)
  - EPS-TOPIK / TOPIK을 위한 한국어 초급 과정 : 900문장 학습
  - EPS-TOPIK / TOPIK 급수 달성을 위한 과정 : 초급(TOPIK 1~2급), 중급(TOPIK 3~4급), 고급(TOPIK 5~6급)
- 핵심 가치:
  - 한국어 학습이 아닌 한국어 습득에 중점, **실제 한국인이 자주 쓰는 표현 기반** 교과과정
  - 기존 한국어 학습 시간 대비 **1/3 수준의 학습 시간으로 TOPIK 3급 이상 달성**을 목표로 하는 효율성
  - 한국어를 한국어로만 교육하는 것이 아닌 학습자 구사언어와 한국어를 동시 사용으로 학습 진행
- 채널:
  - domain : https://amazingkorean.net 
  - web & app : 동영상 강의 수강, 학습 & 복습 & 시험, 결제 시스템 제공

### 1.2 비즈니스 흐름(요약)

- 관리자
  - web & app → 로그인(배정된 계정으로) → 학습자 관리 및 학습 관련 사항 수정 가능

- 학습자
  - web & app → 회원가입 → 로그인 → 결제 후 서비스 이용(동영상 강의 수강, 학습 & 복습 & 시험 및 관련 사항)

- B2C 온라인 강의 + B2B(대학·기관 대상 컨설팅/과정 운영) 병행을 고려

---

## 2. 시스템 & 개발 환경 개요

### 2.1 런타임 / 스택

- **frontend**
  - Vite + React
  - TypeScript
  - Tailwind CSS
- **backend**
  - Rust (Axum 0.8)
  - Tokio
  - SQLx (PostgreSQL)
  - utoipa v5 (OpenAPI/Swagger UI `/docs`)
  - JWT(HS256 기반 액세스 토큰)
  - Redis (세션/리프레시 토큰 관리)
- **database**
  - PostgreSQL
  - 도커 컨테이너 이름: `amk-pg`
  - 기본 포트: `5432`
  - 모든 로그/이력 테이블 시간 컬럼은 `TIMESTAMPTZ (UTC)`, `DEFAULT now()`
- **server**
  - AWS EC2 (Ubuntu/WSL에서 개발)
  - Nginx (80/443 → 앱 서버 프록시) 

### 2.2 라우팅 & OpenAPI

- `Router<AppState>` + `.with_state(state)` 패턴
- 인증 추출:
  - Axum 0.8 `FromRequestParts<S>` 기반 `Claims` 추출
  - `Claims.sub` = `user_id` (i64)
- OpenAPI 루트:
  - `src/api/docs.rs` (예: `ApiDoc`)
  - Swagger UI: `GET /docs`
  - 태그/표시 순서 **고정**: `health → auth → user → videos → study → lesson → admin` (필요 시 추가 리소스는 뒤에)

### 2.3 로컬 개발 & 실행

- DB 마이그레이션:
  - `sqlx migrate run`
- 기본 빌드/실행:
  - `cargo check`
  - `cargo fmt -- --check`
  - `cargo clippy -- -D warnings`
  - `cargo run`
- Swagger 문서 확인:
  - 브라우저에서 `http://localhost:3000/docs`

---

## 3. 공통 규칙 (전역 컨벤션)

### 3.1 시간/타임존

- DB의 시간 컬럼(특히 로그/이력)은:
  - 타입: `TIMESTAMPTZ`
  - 기본값: `DEFAULT now()` (UTC)
- 클라이언트(웹/앱)에선 KST or 로컬 타임존으로 변환하여 표시.

### 3.2 네이밍 & 스키마 규칙 (요약)

> 최상위 원칙

- **외부 인터페이스(DB 스키마, API 경로, JSON 필드 이름)** 은 **snake_case**를 기준으로 한다.
- **각 레이어의 코드 레벨 네이밍**은 해당 언어/프레임워크의 관습을 따른다.
  - 백엔드: Rust 관례
  - 프론트엔드: TypeScript/React 관례

---

#### 3.2.1 Database

> Naming Convention : snake_case  
> 논리명(문서/ERD)은 **대문자 SNAKE_CASE**, 실제 DB 스키마/컬럼은 **소문자 snake_case**를 기본으로 한다.

- **table 명**
  - 형식: `<도메인(단수형, 대문자)>_<의미 1(존재 시 대문자)>_<의미 2(존재 시 대문자)>...`
  - 예시:
    - `USERS` (PostgreSQL에 `USER` 예약어가 있어 복수형 사용)
    - `VIDEO_TAG`, `VIDEO_TAG_MAP`
    - `USERS_LOG`, `ADMIN_USERS_LOG`, `STUDY_TASK_LOG`

- **enum 명**
  - 형식: `<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>..._enum`
  - 예시:
    - `user_auth_enum`, `user_set_language_enum`
    - `study_task_kind_enum`, `lesson_item_kind_enum`

- **log 테이블/컬럼**
  - 테이블:
    - 형식: `<도메인(복수형, 대문자)>_<의미 1(대문자)>_<의미 2(대문자)>..._LOG`
    - 예시: `STUDY_TASK_LOG`, `LOGIN_LOG`, `USERS_LOG`
  - 로그용 컬럼:
    - 형식: `<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>..._log`
    - 예시: `user_nickname_log`, `video_last_user_agent_log`, `study_task_score_log`

- **admin 계열**
  - 테이블:
    - 형식: `ADMIN_<도메인(복수형, 대문자)>_<의미 1(대문자)>_<의미 2(대문자)>...`
    - 예시: `ADMIN_USERS_LOG`, `ADMIN_VIDEO_LOG`, `ADMIN_STUDY_LOG`
  - 컬럼:
    - 형식: `admin_<도메인(단수형, 소문자)>_<의미 1(소문자)>_<의미 2(소문자)>...`
    - 예시: `admin_pick_study_id`, `admin_user_action`, `admin_study_log_id`

---

#### 3.2.2 API 경로 & JSON 필드

- **API 경로**
  - 경로 표기: **소문자 + 케밥케이스**  
    - 예시: `/users`, `/auth/login`, `/admin/videos`
  - 리소스 이름:
    - 기본: **명사(복수형)** 사용 (`/users`, `/videos`, `/studies` 등)
    - 예외: `/auth` 계열은 기능 중심 (`/auth/login`, `/auth/refresh` 등)
  - 액션 표현:
    - **HTTP 메서드**로 표현  
      - 예시: `GET /users`, `POST /users`, `POST /users/me`, `PATCH /admin/users/{id}`

- **리소스 / ID 경로 패턴**
  - 단일 리소스:
    - `/users/{user_id}`, `/videos/{video_id}`
  - 하위 리소스:
    - `/videos/{video_id}/captions`
    - `/videos/{video_id}/progress`
    - `/studies/tasks/{task_id}/explain`

- **사용 예시**
  - 조회:
    - `GET /users/me/settings`, `GET /videos`
  - 생성/업데이트:
    - `POST /videos/{video_id}/progress`
    - `POST /studies/tasks/{task_id}/answer`
    - `POST /users/me`, `POST /users/me/settings`

- **JSON 필드**
  - API 요청/응답의 필드 이름은 **DB 컬럼과 동일한 snake_case**를 사용한다.
    - 예시: `user_id`, `video_title`, `created_at`, `user_state`

---

#### 3.2.3 백엔드(Rust) 네이밍 & 역할

> 기본 원칙  
> - DB 스키마·API·JSON 필드 = **snake_case**  
> - 코드 레벨 네이밍은 **Rust 관례**를 따른다.

- **모듈/파일명**
  - 도메인별 디렉터리 구조(예: `src/api/user/`):
    - `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs` 고정
    - 예: `src/api/user/dto.rs`, `src/api/user/service.rs`, `src/api/user/repo.rs`
  - 그 외 보조 파일은 필요 시 **소문자 + snake_case**로 추가
    - 예: `token_utils.rs`, `validator.rs` 등

- **함수/변수명 (Naming Convention)**
  - **기본 규칙**: `snake_case` (소문자 + 언더스코어)
  - **계층 간 통일 (Feature Parity)**:
    - 하나의 기능(Feature)에 대해 Handler, Service, Repo 계층의 **메인 함수명은 반드시 통일**한다.
    - 코드 추적성(Traceability) 향상을 위함.
    - **패턴**: `[도메인]_[행위]_[대상]` (필요 시 도메인 생략 가능)
    - **예시 (관리자 유저 생성)**:
      - Handler: `admin_create_user(...)`
      - Service: `admin_create_user(...)`
      - Repo: `admin_create_user(...)`
  - **Repo 보조 함수 (Helpers)**:
    - 메인 로직 외의 단순 조회, 검증, 로그 기록 등은 기능에 맞는 이름 사용 가능.
    - 예: `exists_email`, `create_audit_log`, `find_by_id`
  - **타입(Struct/Enum/DTO)**: **PascalCase** (대문자 카멜 표기)
    - 예: `SignupReq`, `AdminUserRes`, `VideoProgressLog`, `UserAuth`

- **DTO/필드명**
  - DB/JSON과 매핑되는 필드 이름은 **snake_case**로 작성
    - 예:
      ```rust
      pub struct UserMeRes {
          pub user_id: i64,
          pub user_email: String,
          pub user_state: String,
      }
      ```
  - 필요 시 `#[serde(rename = "...")]`, `#[sqlx(rename = "...")]` 로 DB/JSON 필드와의 정렬성을 명시적으로 유지

- **도메인별 repo 역할**
  - 각 도메인(`user`, `video`, `study`, `lesson`, `admin` 등)의 `repo.rs`는  
    **그 도메인의 단일 DB 진입점(single entry point)** 역할을 한다.
  - 다른 도메인에서 해당 도메인의 데이터를 다뤄야 할 때,
    - 가능한 한 **그 도메인의 service 레이어**를 경유해서 접근한다.
    - 예: admin이 유저를 생성할 때 → `user::service::create_by_admin(...)` 호출

- **유즈케이스 단위 함수 이름 규칙**
  - 하나의 유즈케이스(예: `/users` 회원가입, `/auth/login`, `/users/me/settings` 수정 등)에 대해서는  
    도메인별 `handler.rs` / `service.rs` / `repo.rs`에서 **가능하면 동일한 함수명**을 사용한다.
    - 예:
      - `handler::signup`
      - `service::signup`
      - `repo::signup`
  - Rust 모듈 네임스페이스를 활용해,
    - `handler::signup` → `service::signup` → `repo::signup` 흐름이 한눈에 보이도록 맞춘다.
  - 예시:
    ```rust
    // handler.rs
    pub async fn signup(...) -> AppResult<Json<SignupRes>> {
        let res = service::signup(...).await?;
        Ok(Json(res))
    }

    // service.rs
    pub async fn signup(...) -> AppResult<SignupRes> {
        let user = repo::signup(...).await?;
        Ok(SignupRes::from(user))
    }

    // repo.rs
    pub async fn signup(...) -> AppResult<UserRow> {
        // INSERT INTO users ...
    }
    ```

- **공통 repo 함수 (여러 유즈케이스에서 공유할 때)**
  - 여러 유즈케이스에서 동일한 DB 동작을 사용하는 경우,
    - repo 내부에서 **좀 더 일반적인 이름**으로 공통 함수를 분리한다.
    - 예:
      - `insert_user`, `get_user_by_email`, `update_user_state` 등
  - service 계층에서는 유즈케이스 이름을 유지한다.
    - 예:
      ```rust
      // repo.rs
      pub async fn insert_user(...) -> AppResult<UserRow> { ... }
      pub async fn get_user_by_email(...) -> AppResult<Option<UserRow>> { ... }

      // service.rs
      pub async fn signup(...) -> AppResult<SignupRes> {
          if repo::get_user_by_email(&req.email).await?.is_some() {
              return Err(AppError::Conflict(...));
          }
          let user = repo::insert_user(...).await?;
          Ok(SignupRes::from(user))
      }

      pub async fn admin_create_user(...) -> AppResult<AdminUserRes> {
          let user = repo::insert_user(...).await?;
          Ok(AdminUserRes::from(user))
      }
      ```
  - 이때 **쿼리 자체를 별도 “쿼리 전용 모듈”로 빼지 않고**,  
    각 도메인 repo(`user::repo`, `video::repo` 등)가 그 도메인의 쿼리 단일 소스 역할을 맡는다.
  - 정말 cross-domain으로 공유해야 하는 복잡한 패턴(예: 통합 통계 뷰 등)은  
    PostgreSQL의 **VIEW/FUNCTION**으로 추상화하는 것을 우선 검토한다.

> 정리:  
> - **유즈케이스 이름은 handler/service/repo에서 최대한 동일하게**,  
> - **쿼리 중복 제거와 스키마 변경 대응은 도메인별 repo에서 책임**,  
> - DB 레벨 공통화가 필요하면 VIEW/FUNCTION으로 해결하는 것을 기본 전략으로 한다.

---

#### 3.2.4 프론트엔드(TypeScript + React) 네이밍

- **React 컴포넌트**
  - 파일명: PascalCase
    - 예시: `LoginPage.tsx`, `VideoListPage.tsx`, `UserSettingsForm.tsx`
  - 컴포넌트 이름 & JSX:
    - 예시: `function LoginPage() { ... }`, `<LoginPage />`

- **기타 TS 파일 (hook / api / lib / util 등)**
  - 파일명: 소문자 + snake_case
    - 예시: `video_api.ts`, `auth_api.ts`, `use_auth.ts`, `date_format.ts`
  - 함수/변수명: camelCase
    - 예시: `fetchVideos`, `loginUser`, `formatDate`

- **API DTO 인터페이스**
  - 인터페이스 이름: PascalCase
    - 예시: `interface VideoRes { ... }`
  - 필드 이름: **snake_case** (백엔드/DB와 동일)
    - 예시:
      ```ts
      export interface VideoRes {
        video_id: number;
        video_title: string;
        created_at: string;
      }
      ```

---

> 자세한 컬럼 구조와 실제 타입 정의는 `amk_schema_patched.sql` 및 각 도메인별 Rust/TS DTO를 기준으로 하며, 이 문서에는 **책임과 역할, 규칙 위주로 요약**한다.

### 3.3 공통 헤더 & 인증

- HTTP 헤더:
  - `Authorization: Bearer <ACCESS_TOKEN>`
    - 인증 필요한 모든 엔드포인트에 필수
  - `Content-Type: application/json`
    - 요청 본문이 JSON일 때
  - `Accept: application/json`
- 인증 플로우(기본):
  - `POST /auth/login` → 액세스 토큰(헤더), 리프레시 토큰(쿠키) 발급
  - 만료 시 `POST /auth/refresh`로 재발급 (리프레시 회전/검증/로그 기록)
- 리프레시 쿠키:
  - SameSite/Domain/Secure 설정은 서버 환경설정에 따르되,
    배포 환경에서 **HTTPS + Secure**를 기본으로 가정.

### 3.4 에러 응답 표준

- 공통 에러 바디 예시:

```json
{
  "error": {
    "code": "invalid_argument",
    "http_status": 400,
    "message": "video_state must be one of: ready,open,close",
    "details": null,
    "trace_id": "..."
  }
}
```

- 필드 의미:
  - `code`: 내부/클라이언트 공통으로 식별 가능한 에러 코드 문자열
  - `http_status`: 실제 HTTP status 코드 (예: 400, 401, 403, 404, 409, 500…)
  - `message`: 사용자가 이해할 수 있는 메시지(영문/다국어는 이후 확장)
  - `details`: 필드별 검증 에러 등 구조화된 정보 (없으면 `null`)
  - `trace_id`: 로깅/트레이싱용 ID

- 대표 매핑 예:
  - 400: 검증 실패, 잘못된 요청 파라미터
  - 401: 인증 실패(토큰 없음/만료/위조)
  - 403: 권한 부족 (`user_state != on`, RBAC 불일치 등)
  - 404: 리소스 없음
  - 409: 무결성 위반 (예: 이메일 중복, UNIQUE 제약)
  - 500/503: 서버 내부 오류, 일시적인 외부 의존성 장애

### 3.5 페이징 & 검색 표준

- 기본 규칙:
  - **페이지 기반(page/size) 페이징**을 기본으로 사용
  - 쿼리 파라미터:
    - `page`: 1 기반 페이지 번호
    - `size`: 페이지 당 개수(기본값/상한은 엔드포인트별 정의)
    - `sort`: 정렬 컬럼 (예: `created_at`, `video_title`)
    - `order`: 정렬 방향 (`asc` / `desc`)
- 페이징 응답 래퍼 예시:

```json
{
  "items": [ /* 결과 배열 */ ],
  "page": 1,
  "size": 20,
  "total": 57
}
```

- 기존에 커서 기반 등의 다른 방식이 있다면:
  - 새로 추가되는 목록형 API는 위 표준을 우선 적용
  - 단건 조회/소규모 목록은 굳이 래퍼 없이 배열/객체 반환 허용

### 3.6 응답 래퍼 정책

- 성공 응답:
  - 별도 상위 래퍼 없이 **직접 JSON 객체/배열** 반환을 기본으로 한다.
  - 페이징이 필요한 경우에만 `items/page/size/total` 래퍼 사용.
- 실패 응답:
  - 위의 **공통 에러 바디**를 사용한다.
- PUT/DELETE:
  - 일반적으로 `200` 또는 `204 No Content` 사용
  - 필요한 경우 `200 + 수정 결과 객체` 허용

---

## 4. 데이터 모델 개요 (요약)

> 전체 DDL/컬럼은 `amk_schema_patched.sql` 기준.
> 여기서는 **주요 도메인과 테이블 역할**만 요약한다.

### 4.1 사용자 도메인 (USERS)

- `USERS`
  - 회원 정보 (이메일, 비밀번호 해시, 이름, 국가, 언어, 생년월일, 성별 등)
  - `user_auth_enum` (HYMN, admin, manager, learner) 사용자 권한
  - `user_state` : boolean 타입 (true = on, false = off) 사용자 계정 활성 여부
  - `user_language_enum` (ko, en) 사용자 구사 언어(추가 확장 예정)
  - `user_gender_enum` (none, male, female, other) 사용자 성별
- `USERS_LOG`
  - 회원 정보 활동 기록
  - `user_action_log` (signup, find_id, reset_pw, update) 사용자 활동 이력
  - `user_auth_enum` (HYMN, admin, manager, learner) 사용자 권한 이력
  - `user_language_enum` (ko, en) 사용자 구사 언어 이력(추가 확장 예정)
  - `user_gender_enum` (none, male, female, other) 사용자 성별 이력
- `USERS_SETTING`
  - 사용자 관련 UI 언어, 학습 언어 우선순위 등 개인 설정
  - `user_set_language_enum` (ko, en) 사용자 사용 언어(추가 확장 예정)
- `ADMIN_USERS_LOG`
  - 사용자 관련 관리자 활동 기록
  - `admin_action_enum` (create, update, banned, reorder, publish, unpublish) 사용자 관련 관리자 활동 이력
- `USER_EXPORT_DATA`
  - 개인정보 내보내기/백업 요청 상태 및 결과 관리(비동기 처리용)

### 4.2 인증/로그인 도메인 (AUTH/LOGIN)

- `LOGIN`
  - 로그인 정보(지역, 방식, 시간, 상태)
  - `login_device_enum` (mobile, tablet, desktop, other) 로그인 기기
  - `login_method_enum` (email, google, apple) 로그인 방법
  - `login_state_enum` (active, revoked, expired, logged_out) 로그인 상태
- `LOGIN_LOG`
  - 로그인 정보 활동 이력(로그인 이벤트, 세부 지역, 세부 방식)
  - `login_event_enum` (login, logout, refresh, rotate, fail) 로그인 활동 이력
  - `login_device_enum` (mobile, tablet, desktop, other) 로그인 기기 이력
  - `login_method_enum` (email, google, apple) 로그인 방법 이력
  - `login_state_enum` (active, revoked, expired, logged_out) 로그인 상태 이력
- `REDIS_SESSION`
  - Key: ak:session:< sid >
  - TTL은 expire_at 기준. 세션 본문은 직렬화(JSON 등)하되, 운영 상 조회 필드는 컬럼으로 문서화.
  - `login_state_enum` (active, revoked, expired, logged_out) 로그인 상태
- `REDIS_REFRESH`
  - Key: ak:refresh:< hash > -> < sid >
  - 로테이션(rotate-on-use) 시 refresh_hash 교체. 재사용 탐지 시 세션 일괄 폐기 정책과 연동.
- `REDIS_USER_SESSIONS`
  - Key: ak:user_sessions:< uid > (set/list 모델을 행 단위로 전개)
  - 실제 Redis에서는 set/list로 보관. dbdiagram 문서화를 위해 행 형태로 표현.

### 4.3 비디오 도메인 (VIDEOS)

- `VIDEO`
  - 동영상 강의 정보(vimeo 링크, 상태, 접근)
  - `video_state_enum` (ready, open, close) 강의 상태
  - `video_access_enum` (public, paid, private, promote) 강의 접근
- `VIDEO_LOG`
  - 동영상 강의 시청 정보(진행, 완료, 횟수, 접속정보)
- `VIDEO_TAG`
  - 동영상 강의 메타 정보(제목, 부제목)
- `VIDEO_TAG_MAP`
  - 동영상 강의 맵핑 : `VIDEO_TAG` - `VIDEO`
- `VIDEO_STAT_DAILY`
  - 동영상 일별 통계 : UTC 기준
- `ADMIN_VIDEO_LOG`
  - 동영상 강의 관련 관리자 활동 기록
  - `admin_action_enum` (create, update, banned, reorder, publish, unpublish) 동영상 강의 관련 관리자 활동 이력

### 4.4 학습 도메인 (STUDY)

- `STUDY`
  - 학습 문제 정보(상태, 프로그램, 문제 정보)
  - `study_state_enum` (ready, open, close) 학습 문제 상태
  - `study_program_enum` (basic_pronunciation, basic_word, basic_900, topik_read, topik_listen, topik_write, tbc) 학습 문제 프로그램
- `STUDY_TASK`
  - 학습 문제 세부 정보(종류, 순서)
  - `study_task_kind_enum` (choice, typing, voice) 학습 문제 유형
- `STUDY_TASK_CHOICE`
  - 학습 문제 : 4지 선다
  - **정답 검증 방안(study_task_choice_correct 비교 방식 : DB Column 비교 방안? ) 추후 구현**
- `STUDY_TASK_TYPING`
  - 학습 문제 : 쓰기 / 타이핑
- `STUDY_TASK_VOICE`
  - 학습 문제 : 발음 → *발음 입력 및 검증 로직 구성 후 세부 컬럼 추가*
- `STUDY_EXPLAIN`
  - 학습 문제 해설(해설 언어, 해설 내용)
  - `user_set_language_enum` (ko, en) 해설 제공 언어(추가 확장 예정)
- `STUDY_TASK_STATUS`
  - 학습 상태(시도 횟수, 최고점, 완료여부)
- `STUDY_TASK_LOG`
  - 학습 문제 풀이 기록(시도 횟수, 최고점, 완료여부, 풀이내용, 접속정보)
  - `study_task_log_action_enum` (view, start, answer, finish, explain) 학습 문제 풀이 이력
- `ADMIN_STUDY_LOG`
  - 학습 문제 관련 관리자 활동 기록
  - `admin_action_enum` (create, update, banned, reorder, publish, unpublish) 학습 문제 관련 관리자 활동 이력

### 4.5 수업 구성 도메인 (LESSON)

- `LESSON`
  - 수업 구성 : 동영상 강의 + 학습 문제(내용 설명)
- `LESSON_ITEM`
  - 수업 구성 : 순서 지정(순서, 종류)
  - `lesson_item_kind_enum` (video, task) 수업 구성 내용
- `LESSON_PROGRESS`
  - 수업 구성 : 학습 진도 사항(진도율, 순서)
- `ADMIN_LESSON_LOG`
  - 수업 구성 관련 관리자 세부 정보
  - `admin_action_enum` (create, update, banned, reorder, publish, unpublish) 수업 구성 관련 관리자 활동 이력

> 상세 스키마 변경이 필요하면, 항상 이 문서와 `amk_schema_patched.sql`을 함께 업데이트한다.

### 4.6 향후 업데이트 도메인 

- `PAY`
  - 결제 : 사용자 결제 관련 테이블, 결제 후 콘텐츠 이용 가능
- `COURSE`
  - 결제 맵핑 : 결제 후 `COURSE` 와 `LESSON`를 맵핑해 콘텐츠 이용 진행
- `LIVE`
  - 실시간 강의 : ZOOM API 연동을 통한 실시간 강의 서비스 관련 테이블

---

## 5. 기능 & API 로드맵 (Phase / 화면 / 엔드포인트 / 상태 / DoD)

> 이 섹션은 **기존 `AMK_Feature_Roadmap.md`의 내용을 기준으로 한다.**
> 아래 표들은 _Phase / 엔드포인트 / 화면 경로 / 기능 명칭 / 점검사항 / UX 규칙 / 기능 완료_ 를 나타내며,
> 마지막 열의 체크박스(`[ ]`/`[x]`)는 구현 완료 여부를 의미한다.
> **추후 업데이트 사항** : URL 구성 순서 수정 필요 -> 각각의 함수명도 수정필요

| 축 | 상태 값 | 상태 설명 |
|---|---|---|
| **Auth** | `pass` | 인증 완료(접근 허용) |
|  | `stop` | 인증 불가(로그인 필요/토큰 없음·만료) |
|  | `forbid` | 접근 불가(권한 부족·차단 상태) |
| **Session** | `active` | 세션 발급 |
|  | `expiring` | 세션 재발급 필요 임박 |
|  | `rotating` | 세션 리프레시 중 |
|  | `expired` | 세션 만료 |
| **Page** | `init` | 페이지 로딩 중(초기 진입·프리페치) |
|  | `ready` | 페이지 로딩 완료(입력/상호작용 가능) |
| **Data** | `empty` | 데이터 없음(오류 아님, 0개 결과) |
|  | `present` | 데이터 존재(정상 바인딩) |
|  | `error` | 데이터 조회 실패(404/500 등) |
| **Form** | `pristine` | 화면 구성 직후, 입력 전 상태 |
|  | `dirty` | 사용자 입력 발생(변경됨) |
|  | `validating` | 클라이언트 검증 중(형식·범위 확인) |
|  | `submitting` | 서버로 제출 중(중복 제출 차단) |
|  | `success` | 서버 저장 성공(후속 이동/토스트) |
|  | `error.client` | 클라이언트 검증 실패(형식/범위 오류) |
|  | `error.conflict` | 서버 충돌(예: 409 이메일 중복) |
| **Request** | `pending` | 네트워크 요청 진행 중 |
|  | `success` | 네트워크 요청 성공 |
|  | `error` | 네트워크 요청 실패 |
|  | `retryable` | 일시 장애로 재시도 가능(예: 503/네트워크) |
| **Course** | `buy` | 구매 완료, 구매 권한 존재 |
|  | `taster` | 체험판, 체험 권한 존재 |
|  | `buy-not` | 비구매, 구매 권한 없음 |
|  | `checking` | 구매 여부 및 구매 권한 점검 |


---

### 5.0 Phase 0 — health ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 0-1 | `GET /healthz` | `/health` | 라이브 헬스 | ***서버 작동 여부 확인***<br>**성공:** Auth pass / Page : healthz init→ready / Request : healthz pending→success / Data : healthz present → **200**<br>**실패:** Auth pass / Page : healthz init→ready / Request : healthz pending→error / Data : healthz error → **500** | [✅] |
| 0-2 | `GET /docs` | `/docs` | API 문서 | ***Swagger 태그 순서 고정(health → auth → user → videos → study → lesson → admin)***<br>**성공:** Auth pass / Page : docs init→ready / Request : docs pending→success / Data : docs present → **200**<br>**실패(스키마 집계 실패):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **500**<br>**실패(정적 경로 누락):** Auth pass / Page : docs init→ready / Request : docs pending→error / Data : docs error → **404** | [✅] |

---

<details>
  <summary>Phase 0 — health 시나리오</summary>
  
#### 5.0-1 : `GET /healthz` 시나리오
- **성공**
  - When: 클라이언트가 `GET /healthz` 호출, Swagger에서만 실행
  - Then: `200 OK`, JSON 바디 `{"status":"live","uptime_ms":..., "version":"v0.1.0"}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패**
  - When: 헬스 핸들러 내부 예외
  - Then: `500 Internal Server Error`, 에러 바디 `{"error":{"http_status":500,"code":"HEALTH_INTERNAL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error

---

#### 5.0-2 : `GET /docs` 시나리오
- **성공**
  - When: 클라이언트가 `GET /docs` 호출, Swagger에서만 실행
  - Then: `200 OK`, Swagger UI 렌더링, **태그 순서가 user→auth→videos→lesson→admin→health**로 보임
  - 상태축: Auth=pass / Page=init→ready / Request=pending→success / Data=present
- **실패(스키마 집계 실패)**
  - When: OpenAPI 집계/리소스 로드 오류
  - Then: `500 Internal Server Error`, `{"error":{"http_status":500,"code":"DOCS_BUILD_FAIL"}}`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
- **실패(정적 경로 누락)**
  - When: 리버스 프록시/정적 경로 미설정
  - Then: `404 Not Found`
  - 상태축: Auth=pass / Page=init→ready / Request=pending→error / Data=error
</details>

---

### 5.1 Phase 1 — user ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 1-1 | `POST /users` | `/signup` | 회원가입 | ***USERS, USERS_LOG 저장 + 세션/토큰 발급***<br>성공: Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→submitting→success / Request signup pending→success / Data signup present → **201**<br>실패(형식/누락): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup empty → **400**<br>실패(도메인 제약): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.client / Request signup pending→error / Data signup error → **422**<br>실패(중복/충돌): Auth pass / Page signup init→ready / Form signup pristine→dirty→validating→error.conflict / Request signup pending→error / Data signup error → **409**<br>실패(레이트리밋): Auth pass / Page signup ready / Form signup error.client / Request signup pending→error / Data signup error → **429** | [✅] |
| 1-2 | `GET /users/me` | `/me` | 내 정보 조회 | ***USERS 안전 필드(비밀번호 제외)***<br>성공: Auth pass / Page me init→ready / Request me pending→success / Data me present → **200**<br>실패(미인증): Auth stop / Page me init→ready / Request me pending→error / Data me error → **401**<br>실패(미존재/비활성): Auth pass / Page me init→ready / Request me pending→error / Data me error → **404** | [✅] |
| 1-3 | `POST /users/me` | `/me/edit` | 내 정보 수정 | ***USERS 일부 업데이트 → USERS_LOG 저장***<br>성공: Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→submitting→success / Request me_edit pending→success / Data me_edit present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit empty → **400**<br>실패(도메인 제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.client / Request me_edit pending→error / Data me_edit error → **422**<br>실패(미인증): Auth stop / Page me_edit init→ready / Request me_edit pending→error / Data me_edit error → **401**<br>실패(충돌·고유제약): Auth pass / Page me_edit init→ready / Form me_edit pristine→dirty→validating→error.conflict / Request me_edit pending→error / Data me_edit error → **409** | [✅] |
| 1-4 | `GET /users/me/settings` | `/settings` | 내 설정 조회 | ***USERS_SETTING 조회***<br>성공: Auth pass / Page settings init→ready / Request settings pending→success / Data settings present → **200**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅] |
| 1-5 | `POST /users/me/settings` | `/settings` | 내 설정 수정 | ***USERS_SETTING 수정 → USERS_LOG 저장***<br>성공: Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→submitting→success / Request settings pending→success / Data settings present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings empty → **400**<br>실패(도메인 제약): Auth pass / Page settings init→ready / Form settings pristine→dirty→validating→error.client / Request settings pending→error / Data settings error → **422**<br>실패(미인증): Auth stop / Page settings init→ready / Request settings pending→error / Data settings error → **401** | [✅] |

---

<details>
  <summary>5.1 Phase 1 — user 시나리오</summary>

#### 5.1-1 : `POST /users` (회원가입)
- **성공 → 201 Created**
  - When: `/signup` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **201**, `Location: /users/{id}`(권장)
    - **Body**: `SignupRes` (안전한 유저 정보 + **Access Token**, `session_id`)
    - **Cookie**: `ak_refresh` (**Refresh Token**, HttpOnly, Secure)
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` pristine→dirty→validating→submitting→success** / Request=`signup` pending→success / Data=`signup` present
  - 로그: USERS insert 후 **USERS_LOG(성공 스냅샷)** 기록(민감정보 제외)
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 형식 불일치, 필수 항목 누락, JSON 파싱 오류
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` empty**
  - 에러 바디: `{ "error": { "http_status": 400, "code": "BAD_REQUEST", "message": "...", "trace_id": "..." } }`
  - 로그: **USERS_LOG(실패 이벤트)** 기록(에러코드/사유, 민감값 마스킹)
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 금지값, 정책 규칙 위반
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:422, code:"UNPROCESSABLE_ENTITY"`
  - 로그: 실패 이벤트 기록
- **실패(중복/충돌) → 409 Conflict**
  - 예: 이메일 UNIQUE 충돌
  - 상태축: Auth=pass / Page=`signup` init→ready / **Form=`signup` … → error.conflict** / Request=`signup` pending→error / **Data=`signup` error**
  - 에러 바디: `http_status:409, code:"CONFLICT"`
  - 로그: 실패 이벤트 기록
- **실패(레이트리밋) → 429 Too Many Requests**
  - 조건: 과도한 가입 시도
  - 상태축: Auth=pass / Page=`signup` ready / **Form=`signup` error.client** / Request=`signup` pending→error / **Data=`signup` error**
  - 헤더: `Retry-After: <seconds>`
  - 로그: 실패 이벤트 + 차단 지표
- **중복 제출 방지(정책)**
  - 프론트: **Form=submitting** 동안 버튼/Enter 비활성
  - 백엔드: 시간/조건 기반 중복 생성 방지(최근 N분 동일 이메일 재시도 시 409 또는 200 재응답 정책 중 택1)

---

#### 5.1-2 : `GET /users/me` (내 정보 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/me` 화면에서 자기 정보를 조회한다
  - Then: **200**, 안전 필드만 반환(비밀번호·해시·토큰 제외)
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→success / **Data=`me` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료/서명 오류
  - Then: **401**, `WWW-Authenticate: Bearer ...`
  - 상태축: **Auth=stop** / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**
- **실패(미존재/비활성) → 404 Not Found**
  - When: 토큰은 유효하나 사용자 계정이 비활성/삭제 처리되어 조회 불가
  - Then: **404**
  - 상태축: Auth=pass / Page=`me` init→ready / Request=`me` pending→error / **Data=`me` error**

---

#### 5.1-3 : `POST /users/me` (내 정보 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/me/edit` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(변경 후 스냅샷 바디) **또는 204**, USERS 업데이트 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` pristine→dirty→validating→submitting→success** / Request=`me_edit` pending→success / **Data=`me_edit` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 이메일 포맷 오류, 필수 필드 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: birthday 범위 위반, 허용되지 않은 locale 값 등
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.client** / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - 상태축: **Auth=stop** / Page=`me_edit` init→ready / Request=`me_edit` pending→error / **Data=`me_edit` error**
- **실패(충돌/고유제약) → 409 Conflict**
  - 예: 닉네임/이메일 고유 제약 충돌 정책을 허용하는 경우
  - 상태축: Auth=pass / Page=`me_edit` init→ready / **Form=`me_edit` … → error.conflict** / Request=`me_edit` pending→error / **Data=`me_edit` error**

---

#### 5.1-4 : `GET /users/me/settings` (내 설정 조회)
- **성공 → 200 OK**
  - When: 인증된 사용자가 `/settings`에서 설정을 조회한다
  - Then: **200**, USERS_SETTING 반환
  - 상태축: Auth=pass / Page=`settings` init→ready / Request=`settings` pending→success / **Data=`settings` present**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**

---

#### 5.1-5 : `POST /users/me/settings` (내 설정 수정)
- **성공 → 200 OK(또는 204)**
  - When: `/settings` 폼 입력 후 제출이 서버 검증을 통과한다
  - Then: **200**(최신 설정 반환) **또는 204**, USERS_SETTING 수정 후 **USERS_LOG(성공 스냅샷)** 기록
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` pristine→dirty→validating→submitting→success** / Request=`settings` pending→success / **Data=`settings` present**
- **실패(형식/누락) → 400 Bad Request**
  - 예: 필수 설정 누락, JSON 파싱 실패
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**
  - 예: 언어 코드 범위/우선순위 규칙 위반(선호 언어 배열 유효성)
  - 상태축: Auth=pass / Page=`settings` init→ready / **Form=`settings` … → error.client** / Request=`settings` pending→error / **Data=`settings` error**
- **실패(미인증) → 401 Unauthorized**
  - When: 토큰 없음/만료
  - Then: **401**
  - 상태축: **Auth=stop** / Page=`settings` init→ready / Request=`settings` pending→error / **Data=`settings` error**

---

#### 공통 정책(1-1 ~ 1-5)
- **응답 에러 스키마(고정)**  
  `{ "error": { "http_status": 400|401|404|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그 정책**: **성공/실패 모두 USERS_LOG 기록**(민감정보 제외, 실패 시 에러코드/사유 포함)
- **검증 기준**: **400**=형식/누락/파싱, **422**=도메인 제약 위반
- **중복 제출 방지**: Form=`submitting` 동안 UI 차단 + 서버 시간/조건 기반 방지
- **레이트리밋(우선 대상: 1-1)**: 과도 시 **429** + `Retry-After`
- **성공 후 페이지 전환**: 성공 시 다음 화면으로 이동하여 **Form 수명주기 종료**
</details>

---

### 5.2 Phase 2 — auth ✅ **Rieds 세션 도입 추후 진행**
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 | 
|---|---|---|---|---|---|
| 2-1 | `POST /auth/login` | `/login` | 로그인 | ***LOGIN/LOGIN_LOG 저장 + Redis 세션/리프레시 발급***<br>성공: Auth pass / Page login init→ready / Form login pristine→dirty→validating→submitting→success / Request login pending→success / Data login present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login empty → **400**<br>실패(도메인 제약): Auth pass / Page login init→ready / Form login pristine→dirty→validating→error.client / Request login pending→error / Data login error → **422**<br>실패(자격증명 불일치): Auth stop / Page login ready / Form login error.client / Request login pending→error / Data login error → **401**<br>실패(계정 상태/차단): Auth forbid / Page login ready / Form login error.client / Request login pending→error / Data login error → **403**(또는 **423**)<br>실패(레이트리밋): Auth pass / Page login ready / Form login error.client / Request login pending→error / Data login error → **429** | [✅] | 
| 2-2 | `POST /auth/logout` | `/logout` | 로그아웃 | ***세션/리프레시 키 제거, LOGIN_LOG 저장***<br>성공: Auth pass / Page logout ready / Request logout pending→success / Data logout present → **204**(또는 **200**)<br>실패(미인증/세션 없음): Auth stop / Page logout ready / Request logout pending→error / Data logout error → **401** | [✅] |
| 2-3 | `POST /auth/refresh` | (전역처리) | 토큰 재발급 | ***리프레시 로테이션/검증/재사용탐지 + 로그(rotate)***<br>성공: Auth pass / Page app ready / Request refresh pending→success / Data refresh present → **200**<br>실패(형식/누락): Auth pass / Page app ready / Request refresh pending→error / Data refresh empty → **400**<br>실패(도메인 제약): Auth pass / Page app ready / Request refresh pending→error / Data refresh error → **422**<br>실패(리프레시 무효/만료): Auth stop / Page app ready / Request refresh pending→error / Data refresh error → **401**<br>실패(재사용탐지/위조): Auth forbid / Page app ready / Request refresh pending→error / Data refresh error → **409**(또는 **403**) | [✅] |
| 2-4 | `POST /auth/find-id` | `/find-id` | 회원 아이디 찾기 | ***개인정보 보호: 결과 폭로 금지(Enumeration Safe), USERS_LOG 저장***<br>성공(요청 수락/존재 여부와 무관):<br> Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→submitting→success / Request find_id pending→success / Data find_id present → **200**(항상 동일 메시지)<br>실패(형식/누락): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id empty → **400**<br>실패(도메인 제약): Auth pass / Page find_id init→ready / Form find_id pristine→dirty→validating→error.client / Request find_id pending→error / Data find_id error → **422**<br>실패(레이트리밋): Auth pass / Page find_id ready / Form find_id error.client / Request find_id pending→error / Data find_id error → **429** | [✅] |
| 2-5 | `POST /auth/reset-pw` | `/reset-password` | 회원 비밀번호 재설정 | ***요청→검증→재설정의 단일 엔드포인트(토큰/코드 포함), USERS_LOG 저장***<br>성공(재설정 완료):<br> Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→submitting→success / Request reset_pw pending→success / Data reset_pw present → **200**(또는 **204**)<br>실패(형식/누락): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw empty → **400**<br>실패(도메인 제약): Auth pass / Page reset_pw init→ready / Form reset_pw pristine→dirty→validating→error.client / Request reset_pw pending→error / Data reset_pw error → **422**<br>실패(토큰/코드 무효·만료): Auth stop / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **401**<br>실패(레이트리밋): Auth pass / Page reset_pw ready / Form reset_pw error.client / Request reset_pw pending→error / Data reset_pw error → **429** | [✅] |

---

<details>
  <summary>5.2 Phase 2 — auth 시나리오 상세 (5.2-1 ~ 5.2-5)</summary>

#### 공통 정책(5.2-1 ~ 5.2-5)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|409|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **로그**: 성공/실패 모두 이벤트 기록  
  - `LOGIN`(성공 상태), `LOGIN_LOG`(성공/실패, 원인, IP/UA 등), 사용자 관련 변경은 `USERS_LOG`  
- **검증 기준**: **400**=형식·누락·파싱, **422**=도메인 제약(길이·패턴·정책 위반)  
- **레이트리밋**: 로그인/비번재설정/아이디찾기엔 **429 + Retry-After**  
- **보안**: Enumeration Safe(아이디 찾기/재설정은 결과 노출 없이 동일 응답 문구)

---

#### 5.2-1 : `POST /auth/login` (로그인)
- **성공 → 200 OK(또는 204)**  
  - When: `/login`에서 이메일/비밀번호 제출(검증 통과)  
  - Then: **200**(또는 **204**), 액세스 토큰·리프레시 토큰 발급(쿠키/헤더), Redis 세션 및 리프레시 키 저장, `LOGIN`/`LOGIN_LOG` 기록  
  - 상태축: Auth=pass / Page=`login` init→ready / **Form=`login` pristine→dirty→validating→submitting→success** / Request=`login` pending→success / Data=`login` present / Session=active
- **실패(형식/누락) → 400**  
  - 예: 이메일 포맷 불일치, 필수 필드 누락, JSON 파싱 실패  
  - 상태축: Form=`login` … → error.client / Request … → error / Data=empty
- **실패(도메인 제약) → 422**  
  - 예: 허용되지 않은 로그인 방식, 비밀번호 정책 위반(클라이언트 강화 검증)  
- **실패(자격증명 불일치) → 401**  
  - 예: 이메일 존재하지만 비밀번호 불일치, 계정 없음  
  - 상태축: Auth=stop / Form error.client / Data error  
- **실패(계정 상태/차단) → 403(또는 423)**  
  - 예: user_state≠'on', 임시 잠금(여러 실패 시도 후)  
- **실패(레이트리밋) → 429**  
  - 헤더: `Retry-After: <seconds>`

---

#### 5.2-2 : `POST /auth/logout` (로그아웃)
- **성공 → 204 No Content(또는 200)**  
  - When: 사용자가 로그아웃 트리거  
  - Then: **204**, Redis의 세션/리프레시 키 제거, `LOGIN_LOG`(logout 이벤트) 기록  
  - 상태축: Auth=pass / Page=`logout` ready / Request=`logout` pending→success / Data=`logout` present / Session=expired
- **실패(미인증/세션 없음) → 401**  
  - 예: 유효한 세션/토큰 없이 호출

---

#### 5.2-3 : `POST /auth/refresh` (토큰 재발급)
- **성공 → 200 OK**  
  - When: 백그라운드 토큰 만료 임박/만료 후 리프레시 제출  
  - Then: **200**, 새 액세스/리프레시 발급(로테이션), Redis: `ak:refresh:<hash> -> <new_session_id>` 갱신, rotate 로그 기록  
  - 상태축: Auth=pass / Page=app ready / Request=`refresh` pending→success / Data=`refresh` present / Session=active
- **실패(형식/누락) → 400**  
  - 예: 리프레시 토큰 헤더/쿠키 누락  
- **실패(도메인 제약) → 422**  
  - 예: 허용되지 않은 클라이언트/디바이스 조합  
- **실패(무효/만료) → 401**  
  - 예: 만료·폐기된 리프레시, 서명 검증 실패  
- **실패(재사용탐지/위조) → 409(또는 403)**  
  - 정책: 재사용 탐지 시 기존 세션 무효화 + 알림/로그인 강제

---

#### 5.2-4 : `POST /auth/find_id` (회원 아이디 찾기)
- 성공 → **200**
  - When: `/find-id`에서 식별 정보(이름 + 이메일)를 입력하고 제출한다
  - Then: **200**, “일치 시 등록된 이메일로 안내가 발송되었습니다” **같은 문구**로 항상 응답(Enumeration Safe), `USERS_LOG` 기록
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` pristine→dirty→validating→submitting→success / Request=`find_id` pending→success / Data=`find_id` present
- 실패(형식/누락) → **400**
  - 예: 필수 입력 누락, 형식 불일치(글자/숫자/이메일 패턴 등), JSON 파싱 오류
  - 상태축: Auth=pass / Page=`find_id` init→ready / Form=`find_id` … → error.client / Request=`find_id` pending→error / Data=`find_id` empty
- 실패(레이트리밋) → **429**
  - 조건: 과도한 시도 감지 시
  - 헤더: `Retry-After: <seconds>`
  - 상태축: Auth=pass / Page=`find_id` ready / Form=`find_id` error.client / Request=`find_id` pending→error / Data=`find_id` error

---

#### 5.2-5 : `POST /auth/reset_pw` (회원 비밀번호 재설정)
- **성공(재설정 완료) → 200 OK(또는 204)**  
  - When: `/reset-password`에서 토큰/코드 + 새 비밀번호 제출  
  - Then: **200**(또는 **204**), 비밀번호 해시 갱신, 관련 세션 전부 무효화(보안), `USERS_LOG` 기록  
  - 상태축: Auth=pass / Page=`reset_pw` init→ready / **Form=`reset_pw` pristine→dirty→validating→submitting→success** / Request=`reset_pw` pending→success / Data=`reset_pw` present / Session=rotating→active
- **실패(형식/누락) → 400**, **실패(도메인 제약) → 422**  
  - 예: 비밀번호 규칙 위반(길이/복잡성), 필수 누락  
- **실패(토큰/코드 무효·만료) → 401**  
  - 예: 만료 코드, 위조 토큰  
- **실패(레이트리밋) → 429**
</details>

---

### 5.3 Phase 3 — video ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 3-1 | `GET /videos` | `/videos` | 비디오 목록 | ***`video_url_vimeo` 불러오기, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page videos init→ready / Request videos pending→success / Data videos empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page videos init→ready / Request videos pending→error / Data videos error → **422** | [✅] |
| 3-2 | `GET /videos/{id}` | `/videos/{videos_id}` | 비디오 상세 | ***VIDEO_TAG 조회, 시청 로그 트리거(클라이언트 재생 시)***<br>성공: Auth pass 또는 stop / Page video init→ready / Request video pending→success / Data video present → **200**<br>실패(없는 영상): Auth pass 또는 stop / Page video init→ready / Request video pending→error / Data video error → **404** | [✅] |
| 3-3 | `GET /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 조회 | ***VIDEO_LOG: `progress_percent`, `last_watched_at` 조회***<br>성공: Auth pass / Page video init→ready / Request progress pending→success / Data progress present(또는 empty=기록없음, 0%) → **200**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅] |
| 3-4 | `POST /videos/{id}/progress` | `/videos/{videos_id}` | 진행도 갱신 | ***0~100 고정(멱등연산) → VIDEO_LOG 저장(`progress_percent`, `last_watched_at`)***<br>성공:<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→submitting→success /<br> Request progress pending→success / Data progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page video init→ready / Form progress pristine→dirty→validating→error.client / Request progress pending→error / Data progress error → **422**<br>실패(미인증): Auth stop / Page video init→ready / Request progress pending→error / Data progress error → **401**<br>실패(없는 영상): Auth pass / Page video init→ready / Request progress pending→error / Data progress error → **404** | [✅] |

---

<details>
  <summary>5.3 Phase 3 — video 시나리오 상세 (5.3-1 ~ 5.3-4)</summary>

#### 공통 정책(5.3-1 ~ 5.3-4)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**  
  - **400** = 형식 오류/필수 누락/파싱 실패(예: page, per_page 숫자 아님)  
  - **422** = 도메인 제약 위반(예: progress 0~100 범위 위반, 증가/감소 규칙 위반을 둘 경우)
- **진행도 규칙**  
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공  
  - `last_watched_at`는 서버 시각으로 갱신  
  - 기록 없음(progress 미생성)은 **200 + empty(0%)**로 응답(오류 아님)

---

#### 5.3-1 : `GET /videos` (비디오 목록)
- **성공(데이터 있음) → 200**
  - When: `/videos` 진입, `page/per_page/sort`가 유효
  - Then: **200**, 목록 + 페이지 메타, 각 항목에 `video_url_vimeo` 포함
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→success / Data=`videos` present

- **성공(데이터 없음) → 200**
  - Then: **200**, 빈 배열 + 페이지 메타
  - 상태축: Data=`videos` empty

- **실패(형식/누락) → 400**
  - 예: `page=abc`(숫자 아님), `per_page=foo`(숫자 아님), `sort=` 값 파싱 불가(쉼표/형식 오류)
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error

- **실패(도메인 제약) → 422**
  - 예: `page<1`, `per_page<1` 또는 허용 상한 초과(예: `per_page>100`), `sort` 값이 허용 목록 외, `lang` 필터가 허용되지 않은 언어코드
  - 상태축: Auth=pass 또는 stop / Page=`videos` init→ready / Request=`videos` pending→error / Data=`videos` error


---

#### 5.3-2 : `GET /videos/{id}` (비디오 상세)
- **성공 → 200 OK**  
  - When: 상세 진입, 존재하는 영상 id  
  - Then: **200**, 본문에 메타(제목, 설명, 길이, `video_url_vimeo`, **VIDEO_TAG 배열**)  
  - 상태축: Auth=pass 또는 stop / Page=`video` init→ready / Request=`video` pending→success / **Data=`video` present**
- **실패(없는 영상) → 404 Not Found**  
  - When: 잘못된 id  
  - 상태축: Request … → error / **Data=`video` error**

> 메모: 실제 시청(재생 시작/완료 등)은 클라이언트에서 비메오 플레이어 이벤트로 잡고, 별도 **progress API**(3-4)를 호출해 **VIDEO_LOG**를 적재.

---

#### 5.3-3 : `GET /videos/{id}/progress` (진행도 조회)
- **성공(기록 있음) → 200 OK**  
  - When: 인증된 사용자가 자신의 진행도 조회  
  - Then: **200**, `{ progress_percent, last_watched_at }`  
  - 상태축: Auth=pass / Page=`video` init→ready / Request=`progress` pending→success / **Data=`progress` present**
- **성공(기록 없음) → 200 OK**  
  - Then: **200**, `{ progress_percent: 0, last_watched_at: null }`  
  - 상태축: Data=`progress` **empty**
- **실패(미인증) → 401 Unauthorized**  
  - When: 토큰 없음/만료  
  - 상태축: Auth=stop / Request … → error / Data=`progress` error
- **실패(없는 영상) → 404 Not Found**

---

#### 5.3-4 : `POST /videos/{id}/progress` (진행도 갱신)
- **성공 → 200 OK(또는 204 No Content)**  
  - When: 클라이언트가 재생 이벤트 동안 진행도(0~100)를 전송  
  - Then: **200**(업데이트 후 스냅샷 반환) **혹은 204**, 서버는 `progress_percent`(클램프 0~100)와 `last_watched_at` 갱신, **VIDEO_LOG upsert**  
  - 상태축: Auth=pass / Page=`video` init→ready / **Form=`progress` pristine→dirty→validating→submitting→success** / Request=`progress` pending→success / **Data=`progress` present**
- **실패(형식/누락) → 400 Bad Request**  
  - 예: `progress_percent`가 숫자 아님, 바디 누락  
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` empty**
- **실패(도메인 제약) → 422 Unprocessable Entity**  
  - 예: 범위(0~100) 위반, (정책 선택 시) 지나친 감소 등 규칙 위반  
  - 상태축: **Form=`progress` … → error.client** / Request … → error / **Data=`progress` error**
- **실패(미인증) → 401 Unauthorized**  
  - When: 토큰 없음/만료  
- **실패(없는 영상) → 404 Not Found**
</details>

---

### 5.4 Phase 4 — study ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 4-1 | `GET /studies` | `/studies` | 학습 문제 목록 | ***`study_program_enum` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page studies init→ready / Request studies pending→success / Data studies empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page studies init→ready / Request studies pending→error / Data studies error → **422** | [✅] |
| 4-2 | `GET /studies/tasks/{id}` | `/studies/tasks/{task_id}` | 학습 문제 상세 | ***STUDY_TASK 조회, 보기(풀이 전)***<br>성공: Auth pass 또는 stop / Page task init→ready / Request task pending→success / Data task present → **200**<br>실패(없는 문항): Auth pass 또는 stop / Page task init→ready / Request task pending→error / Data task error → **404** | [✅] |
| 4-3 | `POST /studies/tasks/{id}/answer` | `/studies/tasks/{task_id}` | 정답 제출/채점 | ***STUDY_TASK_STATUS 업데이트 → STUDY_TASK_LOG 저장(채점 포함)***<br>성공:<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→submitting→success /<br> Request answer pending→success / Data answer present → **200**<br>실패(형식/누락):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer empty → **400**<br>실패(도메인 제약: 선택지 범위/중복 허용 규칙 등):<br> Auth pass / Page task init→ready / Form answer pristine→dirty→validating→error.client / Request answer pending→error / Data answer error → **422**<br>실패(미인증): Auth stop / Page task init→ready / Request answer pending→error / Data answer error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request answer pending→error / Data answer error → **404** | [✅] |
| 4-4 | `GET /studies/tasks/{id}/status` | `/studies/tasks/{task_id}` | 내 시도/기록 | ***내 최신 STATUS(progress/score/attempts) 조회***<br>성공: Auth pass / Page task init→ready / Request status pending→success / Data status present(또는 empty=기록없음) → **200**<br>실패(미인증): Auth stop / Page task init→ready / Request status pending→error / Data status error → **401**<br>실패(없는 문항): Auth pass / Page task init→ready / Request status pending→error / Data status error → **404** | [✅] |
| 4-5 | `GET /studies/tasks/{id}/explain` | `/studies/tasks/{task_id}/explain` | 해설 보기 | ***STUDY_EXPLAIN 문항별 해설/미디어***<br>성공: Auth pass 또는 stop / Page explain init→ready / Request explain pending→success / Data explain present → **200**<br>실패(없는 문항/해설 없음): Auth pass 또는 stop / Page explain init→ready / Request explain pending→error / Data explain error → **404**<br>실패(도메인 정책: 시도 전 열람 금지 설정 시): Auth pass 또는 stop / Page explain ready / Request explain pending→error / Data explain error → **403** | [✅] |

---

<details>
  <summary>5.4 Phase 4 — study 시나리오 상세 (5.4-1 ~ 5.4-5)</summary>

#### 공통 정책(5.4-1 ~ 5.4-5)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**  
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `program=` 빈값)
  - **422** = 도메인 제약 위반(예: `study_program_enum`에 없는 값, `per_page` 상한 초과, 보기 규칙 위반)
- **로그**  
  - 정답 제출(4-3): **STUDY_TASK_LOG**에 제출/채점 결과, 소요시간, 선택지 기록(민감 마스킹 정책 준수)
  - 상태 조회(4-4): 조회 로그는 선택(필요 시 집계용 샘플링)
- **레이트리밋(선택)**  
  - 과도한 채점/새로고침 방지 → **429 + Retry-After**(우선순위 낮음, 추후)
- **권한/공개 정책**  
  - 목록/상세/해설은 서비스 정책에 따라 공개/비공개를 조절 가능(기본: 공개 열람 가능, 정답 제출·내 기록 조회는 인증 필요)

---

#### 5.4-1 : `GET /studies` (학습 문제 목록)
- 성공(데이터 있음) → **200**  
  - When: `/studies` 진입, `program/page/per_page/sort` 유효
  - Then: **200**, 목록 + 페이지 메타, `study_program_enum` 필터 반영
  - 상태축: Auth=pass 또는 stop / Page=`studies` init→ready / Request=`studies` pending→success / Data=`studies` present
- 성공(데이터 없음) → **200**  
  - 빈 배열 + 페이지 메타 / Data=`studies` empty
- 실패(형식/누락) → **400**  
  - 예: `page`/`per_page` 숫자 아님, `program` 파라미터 형식 오류
- 실패(도메인 제약) → **422**  
  - 예: `program`이 enum에 없음, `per_page` 상한 초과, 허용되지 않은 `sort` 필드

---

#### 5.4-2 : `GET /studies/tasks/{id}` (학습 문제 상세)
- 성공 → **200**  
  - Then: **200**, 문제 본문/보기/메타(난이도/분류)
  - 상태축: Auth=pass 또는 stop / Page=`task` init→ready / Request=`task` pending→success / Data=`task` present
- 실패(없는 문항) → **404**  
  - 잘못된 `{id}`

---

#### 5.4-3 : `POST /studies/tasks/{id}/answer` (정답 제출/채점)
- 성공 → **200**  
  - When: 인증 사용자, study_task_typing, study_task_choice, study_task_voice 답안을 제출
  - Then: **200**, 채점 결과(`is_correct`, `score`, `correct_choice`, `explain_available` 등), **STUDY_TASK_STATUS** 업데이트, **STUDY_TASK_LOG** 적재
  - 상태축: Auth=pass / Page=`task` init→ready / Form=`answer` pristine→dirty→validating→submitting→success / Request=`answer` pending→success / Data=`answer` present
- 실패(형식/누락) → **400**  
  - 예: 바디 없음, 선택지 배열 스키마 불일치, 서술형 빈 문자열 금지 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` empty
- 실패(도메인 제약) → **422**  
  - 예: 단일선택 문항에 다중 선택 제출, 범위를 벗어난 보기 인덱스, 이미 종료된 시도에 재제출 금지 정책 등
  - 상태축: Form=`answer` … → error.client / Request=`answer` pending→error / Data=`answer` error
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**  
  - 잘못된 `{id}`
- 실패(레이트리밋, 선택) → **429**
  - 과도한 제출/채점 요청

---

#### 5.4-4 : `GET /studies/tasks/{id}/status` (내 시도/기록)
- 성공 → **200**  
  - Then: **200**, `{ last_attempt_at, attempts, best_score, last_score, progress }`
  - 상태축: Auth=pass / Page=`task` init→ready / Request=`status` pending→success / Data=`status` present(또는 empty)
- 실패(미인증) → **401**
  - 토큰 없음/만료
- 실패(없는 문항) → **404**

---

#### 5.4-5 : `GET /studies/tasks/{id}/explain` (해설 보기)
- 성공 → **200**  
  - Then: **200**, 해설 텍스트/이미지/영상 링크(있다면)
  - 상태축: Auth=pass 또는 stop / Page=`explain` init→ready / Request=`explain` pending→success / Data=`explain` present
- 실패(해설 없음/없는 문항) → **404**
  - 자료 미제공 또는 잘못된 `{id}`
- 실패(정책상 제한) → **403**
  - 예: “최소 1회 시도 후 열람” 정책을 켠 경우, 시도 전 접근 차단

</details>

---

### 5.5 Phase 5 — lesson ✅
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 5-1 | `GET /lessons` | `/lessons` | 수업 전체 목록 | ***`lesson_idx` 기준 조회, 페이지네이션***<br>성공(데이터 있음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons present → **200**<br>성공(데이터 없음): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→success / Data lessons empty → **200**<br>실패(형식/누락): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **400**<br>실패(도메인 제약): Auth pass 또는 stop / Page lessons init→ready / Request lessons pending→error / Data lessons error → **422** | [✅] |
| 5-2 | `GET /lessons/{id}` | `/lessons/{lesson_id}` | 수업 상세 | ***`video_tag_id` + `study_task_id` 기반 목록 조회, 페이지네이션***<br>성공: Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→success / Data lesson present → **200**<br>실패(없는 수업): Auth pass 또는 stop / Page lesson init→ready / Request lesson pending→error / Data lesson error → **404** | [✅] |
| 5-3 | `GET /lessons/{id}/items` | `/lessons/{lesson_id}/items` | 수업 학습 | ***`lesson_item_seq` 기준 조회, 학습 화면 로드(풀이/진행은 별도 API)***<br>성공: Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→success / Data lesson_items present → **200**<br>실패(없는 수업/항목): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson_items ready / Request lesson_items pending→error / Data lesson_items error → **403**<br>실패(형식/누락·도메인): Auth pass 또는 stop / Page lesson_items init→ready / Request lesson_items pending→error / Data lesson_items error → **400**/**422** | [✅] |
| 5-4 | `GET /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 조회 | ***LESSON_PROGRESS 최신 값 조회(없으면 0%)***<br>성공: Auth pass / Page lesson init→ready / Request lesson_progress pending→success / Data lesson_progress present(또는 empty=0%) → **200**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404** | [✅] |
| 5-5 | `POST /lessons/{id}/progress` | `/lessons/{lesson_id}` | 수업 진행 갱신 | ***LESSON_PROGRESS 컬럼 업데이트(0~100 고정, 멱등)***<br>성공:<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→submitting→success /<br> Request lesson_progress pending→success / Data lesson_progress present → **200**(또는 **204**)<br>실패(형식/누락):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress empty → **400**<br>실패(도메인 제약: 범위/증감 규칙):<br> Auth pass / Page lesson init→ready / Form lesson_progress pristine→dirty→validating→error.client /<br> Request lesson_progress pending→error / Data lesson_progress error → **422**<br>실패(미인증): Auth stop / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **401**<br>실패(없는 수업): Auth pass / Page lesson init→ready / Request lesson_progress pending→error / Data lesson_progress error → **404**<br>실패(정책상 제한: 수강권 필요): Auth forbid / Page lesson ready / Request lesson_progress pending→error / Data lesson_progress error → **403** | [✅] |

---

<details>
  <summary>5.5 Phase 5 — lesson 시나리오 상세 (5.5-1 ~ 5.5-5)</summary>

#### 공통 정책(5.5-1 ~ 5.5-5)
- **에러 바디(고정)**  
  `{ "error": { "http_status": 400|401|403|404|422|429|500, "code": "...", "message": "...", "details": { }, "trace_id": "..." } }`
- **검증 기준**  
  - **400** = 형식/누락/파싱 실패(예: `page=abc`, `per_page=foo`)  
  - **422** = 도메인 제약 위반(예: `per_page` 상한, 허용되지 않은 `sort`, 진행도 0~100 범위 위반 등)
- **권한/수강권**  
  - 수업 목록/상세/아이템은 서비스 정책에 따라 공개 가능하되, **수강권 필수 정책을 켜면 403** 적용  
  - 진행도 조회/갱신은 **인증 필수**
- **진행도 규칙**  
  - 멱등: 동일 값 재전송은 상태 변화 없이 성공  
  - 기록 없음은 **200 + empty(0%)**로 응답(오류 아님)
- **로그**  
  - 진행도 갱신(5-5): LESSON_PROGRESS 업데이트 시 서버시각으로 갱신, 필요 시 LESSON_PROGRESS_LOG(선택)

---

#### 5.5-1 : `GET /lessons` (수업 전체 목록)
- 성공(데이터 있음) → **200**  
  - When: `/lessons` 진입, `page/per_page/sort` 유효  
  - Then: **200**, 목록 + 페이지 메타(`lesson_idx` 기준 정렬)
  - 상태축: Auth=pass 또는 stop / Page=`lessons` init→ready / Request=`lessons` pending→success / Data=`lessons` present
- 성공(데이터 없음) → **200**  
  - 빈 배열 + 페이지 메타 / Data=`lessons` empty
- 실패(형식/누락) → **400**  
  - 예: 숫자 아님, 음수/0 페이지
- 실패(도메인 제약) → **422**  
  - 예: `per_page` 상한 초과, 허용 외 정렬 키

---

#### 5.5-2 : `GET /lessons/{id}` (수업 상세)
- 성공 → **200**  
  - Then: **200**, 수업 메타 + 연계 목록(영상 태그/학습 과제 id 집합) 페이지네이션
  - 상태축: Auth=pass 또는 stop / Page=`lesson` init→ready / Request=`lesson` pending→success / Data=`lesson` present
- 실패(없는 수업) → **404**
- **추후 Lessons 관련 state enum 및 column 추가 필요!!**

---

#### 5.5-3 : `GET /lessons/{id}/items` (수업 학습)
- 성공 → **200**  
  - Then: **200**, `lesson_item_seq` 기준 아이템 목록(문항/비디오/자료 등), 학습 화면 로드
  - 상태축: Auth=pass 또는 stop / Page=`lesson_items` init→ready / Request=`lesson_items` pending→success / Data=`lesson_items` present
- 실패(없는 수업/항목) → **404**
- 실패(정책상 제한: 수강권 필요) → **403** →**추후 수강권 관련 사항 업데이트 후 적용 필요.**
- 실패(형식/누락 → 400 / 도메인 제약 → 422)**

---

#### 5.5-4 : `GET /lessons/{id}/progress` (수업 진행 조회)
- 성공 → **200**  
  - Then: **200**, `{ progress_percent, last_updated_at }` (없으면 `{0, null}`)
  - 상태축: Auth=pass / Page=`lesson` init→ready / Request=`lesson_progress` pending→success / Data=`lesson_progress` present(또는 empty)
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**

---

#### 5.5-5 : `POST /lessons/{id}/progress` (수업 진행 갱신)
- 성공 → **200**(또는 **204**)  
  - When: 학습 중간/완료 시 진행도를 제출(0~100), 멱등 업데이트
  - Then: **200**(업데이트 후 스냅샷) 또는 **204**, 서버는 LESSON_PROGRESS 갱신
  - 상태축: Auth=pass / Page=`lesson` init→ready / Form=`lesson_progress` pristine→dirty→validating→submitting→success / Request=`lesson_progress` pending→success / Data=`lesson_progress` present
- 실패(형식/누락) → **400**
  - 예: 바디 누락, 숫자 아님
- 실패(도메인 제약) → **422**
  - 예: 0~100 범위 위반, (정책 선택 시) 역진행 금지
- 실패(미인증) → **401**
- 실패(없는 수업) → **404**
- 실패(정책상 제한: 수강권 필요) → **403**

</details>

---

### 5.5.6 Phase 6 — admin ✅ **mvp 진행 후 보안 부분 업데이트 필요**
| 번호 | 엔드포인트 | 화면 경로 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|---|
| 6-1 | `GET /admin/users` | `/admin/users?page=&size=&q=&sort=&order=` | 사용자 조회 | ***검색/정렬/페이지네이션, RBAC(admin)***<br>성공(데이터 있음/없음):<br> Auth pass / Page admin_users init→ready / Request admin_users pending→success /<br> Data admin_users present empty → **200**<br>실패(미인증): Auth stop → **401**<br>실패(RBAC): Auth forbid → **403**<br>실패(형식/누락): … → **400**<br>실패(도메인 제약): … → **422** | [✅] |
| 6-2 | `POST /admin/users` | `/admin/users/new` | 사용자 단건 생성 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공:<br> Auth pass / Page admin_users_new init→ready / Form admin_users_new pristine→dirty→validating→submitting→success /<br> Request admin_users_new pending→success / Data admin_users_new present → **201**<br>실패(미인증): **401** / RBAC: **403** / 형식: **400** / 도메인: **422** / 중복: **409** | [✅] |
| 6-3 | `POST /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 생성 | ***ADMIN_USERS_LOG 저장, 부분 성공 처리, RBAC***<br>성공(전량): … → **201**<br>성공(부분): … → **207**(멀티), 실패 항목 포함<br>실패(인증/권한/형식/도메인/중복): **401/403/400/422/409** | [✅] |
| 6-4 | `PATCH /admin/users/{id}` | `/admin/users/{user_id}/edit` | 사용자 단건 수정 | ***ADMIN_USERS_LOG 저장, RBAC***<br>성공: … → **200**(또는 **204**)<br>실패(미인증/권한): **401/403**<br>실패(대상없음): **404**<br>실패(형식/도메인/충돌): **400/422/409** | [✅] |
| 6-5 | `PATCH /admin/users/bulk` | `/admin/users/bulk` | 사용자 다중 수정 | ***ADMIN_USERS_LOG 저장, 부분 성공, RBAC***<br>성공(전량): **200**(또는 **204**)<br>성공(부분): **207**<br>실패(인증/권한/형식/도메인/충돌): **401/403/400/422/409** | [✅] |
| 6-6 | `GET /admin/videos` | `/admin/videos?page=&size=&q=&sort=&order=` | 비디오 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공(있음/없음): … → **200** / 실패(401/403/400/422) | [✅] |
| 6-7 | `POST /admin/videos` | `/admin/videos/new` | 비디오 단건 생성 | ***ADMIN_VIDEO_LOG 저장, RBAC***<br>성공: … → **201**<br>실패(401/403/400/422/409) | [✅] |
| 6-8 | `POST /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 생성 | ***ADMIN_VIDEO_LOG 저장, 부분 성공, RBAC***<br>성공(전량): **201** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-9 | `PATCH /admin/videos/{id}` | `/admin/videos/{video_id}/edit` | 비디오 단건 수정 | ***ADMIN_VIDEO_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-10 | `PATCH /admin/videos/bulk` | `/admin/videos/bulk` | 비디오 다중 수정 | ***ADMIN_VIDEO_LOG 저장, 부분 성공, RBAC***<br>성공: **200**(또는 **204**) / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-11 | `PATCH /admin/videos/{id}/tags` | `/admin/videos/{video_id}/tags` | 비디오 태그 단건 수정 | ***태그 검증·중복 방지, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-12 | `PATCH /admin/videos/bulk/tags` | `/admin/videos/bulk/tags` | 비디오 태그 다중 수정 | ***부분 성공, ADMIN_VIDEO_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-13 | `GET /admin/videos/{id}/stats` | `/admin/videos/{video_id}/stats?from=&to=&granularity=daily` | 비디오 일별 통계 조회 **추후진행** | ***VIDEO_STAT_DAILY 조회, 기간/그라뉼러리티 검증, RBAC***<br>성공: **200**(없음도 **200**) / 실패: **401/403/404/400/422** | [❗❗❗❗❗] |
| 6-14 | `GET /admin/studies` | `/admin/studies?page=&size=&q=&sort=&order=` | 학습 문제 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200** / 실패: **401/403/400/422** | [✅] |
| 6-15 | `POST /admin/studies` | `/admin/studies/new` | 학습 문제 단건 생성 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **201** / 실패: **401/403/400/422/409** | [✅] |
| 6-16 | `POST /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-17 | `PATCH /admin/studies/{id}` | `/admin/studies/{study_id}/edit` | 학습 문제 단건 수정 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-18 | `PATCH /admin/studies/bulk` | `/admin/studies/bulk` | 학습 문제 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-19 | `GET /admin/studies/tasks` | `/admin/studies/tasks?study_id={study_id}&page=&size=` | 학습 문제 세부 정보 조회 | ***study_id 필수 검증, 페이지네이션, RBAC***<br>성공: **200** / 실패: **401/403/400/422/404** | [✅] |
| 6-20 | `POST /admin/studies/tasks` | `/admin/studies/tasks/new` | 학습 문제 세부 정보 단건 생성 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **201** / 실패: **401/403/400/422/404/409** | [✅] |
| 6-21 | `POST /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 문제 세부 정보 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207** / 실패: **401/403/400/422/404/409** | [✅] |
| 6-22 | `PATCH /admin/studies/tasks/{id}` | `/admin/studies/tasks/{task_id}/edit` | 학습 문제 세부 정보 단건 수정 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-23 | `PATCH /admin/studies/tasks/bulk` | `/admin/studies/tasks/bulk` | 학습 문제 세부 정보 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-24 | `GET /admin/studies/tasks/explain` | `/admin/studies/tasks/explain?task_id={task_id}&page=&size=` | 학습 문제 해설 조회 | ***task_id/페이지 검증, RBAC***<br>성공: **200** / 실패: **401/403/400/422/404** | [✅] |
| 6-25 | `POST /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/new` | 학습 문제 해설 단건 생성 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **201** / 실패: **401/403/400/422/404/409** | [✅] |
| 6-26 | `POST /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 문제 해설 다중 생성 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **201** / 부분: **207** / 실패: **401/403/400/422/404/409** | [✅] |
| 6-25 | `PATCH /admin/studies/tasks/{id}/explain` | `/admin/studies/tasks/{task_id}/explain/edit` | 학습 문제 해설 단건 수정 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-27 | `PATCH /admin/studies/tasks/bulk/explain` | `/admin/studies/tasks/bulk/explain` | 학습 문제 해설 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409/404** | [✅] |
| 6-28 | `GET /admin/studies/tasks/status` | `/admin/studies/tasks/status?task_id={task_id}&page=&size=` | 학습 문제 상태 조회 | ***task_id/페이지 검증, RBAC***<br>성공: **200** / 실패: **401/403/400/422/404** | [✅] |
| 6-29 | `PATCH /admin/studies/tasks/{id}/status` | `/admin/studies/tasks/{task_id}/status/edit` | 학습 문제 상태 단건 수정 | ***ADMIN_STUDY_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-30 | `PATCH /admin/studies/tasks/bulk/status` | `/admin/studies/tasks/bulk/status` | 학습 문제 상태 다중 수정 | ***부분 성공, ADMIN_STUDY_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409/404** | [ ✅] |
| 6-31 | `GET /admin/lessons` | `/admin/lessons?page=&size=&q=&sort=&order=` | 수업 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200** / 실패: **401/403/400/422** | [✅] |
| 6-32 | `POST /admin/lessons` | `/admin/lessons/new` | 수업 단건 생성 | ***ADMIN_LESSON_LOG 저장, RBAC***<br>성공: **201** / 실패: **401/403/400/422/409** | [✅] |
| 6-33 | `POST /admin/lessons/bulk` | `/admin/lessons/bulk` | 수업 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-34 | `PATCH /admin/lessons/{id}` | `/admin/lessons/{lesson_id}/edit` | 수업 단건 수정 | ***ADMIN_LESSON_LOG 저장, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-35 | `PATCH /admin/lessons/bulk` | `/admin/lessons/bulk` | 수업 다중 수정 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-36 | `GET /admin/lessons/items` | `/admin/lessons/items?page=&size=&q=&sort=&order=` | 수업 순서 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200** / 실패: **401/403/400/422** | [✅] |
| 6-37 | `POST /admin/lessons/items/{id}` | `/admin/lessons/new` | 수업 단건 생성 | ***ADMIN_LESSON_LOG 저장, RBAC***<br>성공: **201** / 실패: **401/403/400/422/409** | [✅] |
| 6-38 | `POST /admin/lessons/bulk/items` | `/admin/lessons/bulk` | 수업 다중 생성 | ***부분 성공, ADMIN_LESSON_LOG, RBAC***<br>성공: **201** / 부분: **207** / 실패: **401/403/400/422/409** | [✅] |
| 6-39 | `PATCH /admin/lessons/{id}/items` | `/admin/lessons/{lesson_id}/items` | 수업 순서 단건 수정 | ***순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-40 | `PATCH /admin/lessons/bulk/items` | `/admin/lessons/bulk/items` | 수업 순서 다중 수정 | ***부분 성공, 순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409/404** | [✅] |
| 6-41 | `GET /admin/lessons/progress` | `/admin/lessons/progress?page=&size=&q=&sort=&order=` | 수업 진행 조회 | ***검색/정렬/페이지네이션, RBAC***<br>성공: **200** / 실패: **401/403/400/422** | [✅] |
| 6-42 | `PATCH /admin/lessons/{id}/progress` | `/admin/lessons/{lesson_id}/progress` | 수업 진행 단건 수정 | ***순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200**(또는 **204**) / 실패: **401/403/404/400/422/409** | [✅] |
| 6-43 | `PATCH /admin/lessons/bulk/progress` | `/admin/lessons/bulk/progress` | 수업 진행 다중 수정 | ***부분 성공, 순서 규칙 검증, ADMIN_LESSON_LOG, RBAC***<br>성공: **200** / 부분: **207** / 실패: **401/403/400/422/409/404** | [✅] |

---

<details>
  <summary>5.6 Phase 6 — admin 공통 정책 & 시나리오 템플릿</summary>

#### 공통 보안/권한
- 미인증: Auth=stop → **401**
- 권한 부족(RBAC): Auth=forbid → **403**
- 리소스 은닉 전략(선택): 민감 리소스는 **404**로 은닉 가능

#### 에러 스키마(고정)
`{ "error": { "http_status": 400|401|403|404|409|422|429|500, "code": "...", "message": "...", "details": {}, "trace_id": "..." } }`

#### 검증 기준
- **400**: 형식/누락/파싱 실패(예: page=abc, size<1, 잘못된 정렬문법)
- **422**: 도메인 제약 위반(허용되지 않은 sort 필드, size 상한 초과, 비즈 규칙 위반)
- **409**: 고유제약/상태충돌(중복 이메일/태그, 삭제된 리소스 수정 금지 등)
- **429**: 대량/연속 작업 차단(선택, Retry-After 포함)

#### 로깅(필수)
- 모든 Admin 엔드포인트: 성공/실패 모두 `admin_*_log` 기록(요청 요약, actor user_id, 대상/개수, 결과코드, trace_id). 민감값은 마스킹.

---

#### 목록/조회 공통 시나리오(예: GET /admin/videos)
- 성공(데이터 있음/없음) → **200**  
  Auth pass / Page init→ready / Request pending→success / Data present|empty
- 실패(미인증/권한) → **401**/**403**
- 실패(형식/도메인) → **400**/**422**

---

#### 단건 생성 템플릿(예: POST /admin/videos)
- 성공 → **201**  
  Page init→ready / Form pristine→dirty→validating→submitting→success / Request pending→success / Data present  
  헤더: `Location: /admin/videos/{id}`
- 실패(형식/도메인/중복/권한) → **400**/**422**/**409**/**401**/**403**

---

#### 다중 생성(벌크) 템플릿
- 성공(전량) → **201**
- 성공(부분) → **207**
- 실패 항목은 배열로 에러 사유 제공(예: 400/422/409)

---

#### 단건 수정 템플릿(예: PATCH /admin/lessons/{id})
- 성공 → **200** 또는 **204**
- 실패 → **401**/**403**/**404**/**400**/**422**/**409**

---

#### 벌크 수정 템플릿
- 성공(전량) → **200** 또는 **204**
- 성공(부분) → **207**
- 실패 항목별 에러 사유 포함

---

#### 통계 조회(예: GET /admin/videos/{id}/stats)
- 성공 → **200** (빈 구간도 **200**)
- 실패 → **401**/**403**/**404**/**400**/**422**  
  (기간(from≤to)·그라뉼러리티 검증 포함)

</details>

---

### 5.7 Phase 7 — scripts *(엔드포인트 없음)*
| 번호 | 작업 | 기능 명칭 | 점검사항 | 기능 완료 |
|---|---|---|---|---|
| 7-1 | Docker/ENV | 로컬/배포 스크립트 | 일관된 `up/run` 스크립트화 | [ ] |
| 7-2 | Migration | DB 초기화/업데이트 | `sqlx migrate run` 표준화 | [ ] |
| 7-3 | Smoke | cURL/K6 스모크 | 성공·실패 1케이스 자동화 | [ ] |

---

### 비고
- 코스(Course)는 후순위. ERD 정리 후 별도 Phase로 추가 예정.
- 모든 Phase는 “**백엔드 엔드포인트 구현 → 프론트 1화면 연동 → 스모크(성공+대표 에러)**” 순으로 완료 표시.

---

## 6. 프론트엔드 구조 & 규칙

> 목적: AMK 백엔드(API)와 일관되게 동작하는 **Vite + React + Tailwind** 기반 프론트엔드의 공통 규칙을 정의한다.  
> 이 섹션은 **웹(반응형, 앱까지 고려)** 을 기준으로 한다.

---

### 6.1 프론트엔드 스택 & 기본 원칙

> 목적: AMK 백엔드(API)와 일관되게 동작하며, **한국어 학습자 환경(저사양/데이터 절약)**에 최적화된 **"Lightweight React"** 아키텍처를 정의한다.

- **기술 스택 (Strict)**
  - **Core**: Vite + React + TypeScript
    - *Create React App(CRA) 및 Next.js 사용 금지 (SPA 모드 유지)*
  - **Styling**: Tailwind CSS
  - **UI Library**: **shadcn/ui** (Radix UI 기반 Headless)
    - *MUI, AntD 등 번들 사이즈가 큰 UI 프레임워크 반입 금지*
  - **State Management**:
    - **Server State**: **TanStack Query (React Query)** (API 캐싱 및 로딩 상태 관리)
    - **Global Client State**: **Zustand** (로그인 세션 등 최소한의 전역 상태)
    - **Form**: **React Hook Form** + **Zod** (렌더링 최적화 및 스키마 검증)
  - **Routing**: React Router (v6)
  - **HTTP**: `fetch` API 래퍼 (Axios 사용 지양, `src/api/client.ts`로 통일)

- **설계 기본 원칙**
  1. **단일 소스 오브 트루스 (SSOT)**
     - 백엔드 스펙/엔드포인트/상태코드/에러 정책은 **항상 AMK_API_MASTER.md** 를 기준으로 한다.
  
  2. **성능 및 데이터 최적화 (Data Saver First)**
     - **목표**: 인터넷 환경이 좋지 않은 국가의 학습자를 위해 초기 로딩 속도와 데이터 소모를 최소화한다.
     - **Code Splitting**: 모든 페이지 라우트는 `React.lazy`와 `Suspense`를 통해 동적으로 로딩한다.
     - **Asset Lazy Loading**: 이미지와 비디오(Vimeo Player SDK 포함)는 뷰포트에 들어오거나 사용자가 상호작용(클릭)하기 전까지 절대 미리 로드하지 않는다.
     - **No Heavy Libs**: Gzip 기준 **10kb**를 초과하는 외부 라이브러리 추가 시, 반드시 대체재(직접 구현 또는 경량 라이브러리)를 검토한다.

  3. **모바일 퍼스트 & 앱 확장성 (Mobile First Architecture)**
     - **반응형**: 모든 UI는 모바일(`sm`) 기준으로 먼저 설계하고, 태블릿(`md`) 및 데스크톱(`lg`)으로 확장한다.
     - **로직 분리 (Hook Separation)**:
       - 향후 **React Native 모바일 앱** 확장을 고려하여, 비즈니스 로직은 컴포넌트(UI) 내부에 작성하지 않는다.
       - 반드시 **Custom Hook** (`useAuth`, `useVideoPlayer` 등)으로 추출하여 UI와 로직을 100% 분리한다.

  4. **도메인(Category) 주도 구조**
     - 백엔드와 동일하게 `auth / user / video / study / lesson / admin` 도메인 기준으로 폴더와 로직을 격리한다.
     - 페이지 안에서 "즉석 컴포넌트"를 만들지 않고, `common/ui`의 디자인 시스템을 조립하여 사용한다.

---

### 6.2 프론트 디렉터리 구조 & 컴포넌트 계층

> 목적: **도메인 주도(Domain-Driven)** 구조를 기반으로, shadcn/ui 표준과 React Hook 패턴을 결합하여 유지보수성과 확장성을 극대화한다.

#### 6.2.1 디렉터리 구조 (Strict)

- 기준 경로: `frontend/src/`

```text
src/
  app/
    router.tsx           # 라우트 정의 (React Router v6)
    layout_root.tsx      # 최상위 레이아웃 (AppShell)
    providers.tsx        # 전역 Provider 모음 (QueryClient, AuthProvider 등)
  
  api/
    client.ts            # fetch 래퍼 (Axios 지양), Interceptor (토큰/에러)
    # 도메인별 API 호출 함수 (fetcher)
    auth.ts
    user.ts
    video.ts
    study.ts
    lesson.ts
    admin.ts

  category/              # ★ 핵심: 도메인별 기능 격리 (Vertical Slicing)
    auth/
      page/              # 페이지 컴포넌트 (Route와 1:1 매핑)
      component/         # 해당 도메인 전용 UI 조각
      hook/              # 비즈니스 로직 & Custom Hook (UI 분리 원칙)
      types.ts           # 해당 도메인 전용 Request/Response DTO 타입
    user/
      page/
      component/
      hook/
      types.ts
    video/
      # ... (동일 구조)
    study/
      # ... (동일 구조)
    lesson/
      # ... (동일 구조)
    admin/
      # ... (동일 구조)

  components/            # 공용 컴포넌트 (Horizontal Slicing)
    ui/                  # ★ shadcn/ui 설치 경로 (Button, Dialog 등)
    layout/              # Header, Footer, Sidebar 등 레이아웃 조각
    shared/              # 도메인에 종속되지 않는 재사용 컴포넌트 (LoadingSpinner 등)

  hooks/                 # 전역 Custom Hook
    use_auth.ts          # 인증 상태 관리 (Zustand + Logic)
    use_toast.ts         # 알림 UI 제어
    use_mobile.ts        # 모바일 감지 및 반응형 처리

  lib/
    utils.ts             # cn() 등 shadcn/ui 필수 유틸
    constants.ts         # 전역 상수
    format.ts            # 날짜/시간/통화 포맷터
```

> **네이밍 규칙 (Strict)**
> - **Files**:
>   - React 컴포넌트 (`.tsx`): **PascalCase** (예: `LoginPage.tsx`, `VideoCard.tsx`)
>   - 그 외 TS 파일 (`.ts`): **snake_case** (예: `video_api.ts`, `use_auth.ts`, `utils.ts`)
> - **Code**:
>   - 컴포넌트/인터페이스/타입명: **PascalCase**
>   - 변수/함수명: **camelCase**
>   - **API DTO 필드명**: 백엔드 DB 컬럼명과 100% 일치하는 **snake_case** (예: `video_id`, `is_completed`)
>     - *프론트엔드에서 camelCase로 변환하지 않고 그대로 사용한다.*

#### 6.2.2 컴포넌트 3단계 계층

1. **Page 컴포넌트 (`category/*/page/`)**
   - **역할**: 라우팅의 종착점. 데이터 페칭(`useQuery`)과 레이아웃 조립만 담당.
   - **규칙**:
     - `useEffect` 등 복잡한 로직을 직접 포함하지 않는다. (Hook으로 위임)
     - 스타일링(Tailwind)을 최소화하고, `component`들을 배치하는 데 집중한다.
     - 파일명 예시: `VideoListPage.tsx`, `SignupPage.tsx`

2. **도메인 컴포넌트 (`category/*/component/`)**
   - **역할**: 특정 도메인 기능(비디오 플레이어, 문제 풀이 폼)을 수행하는 UI 블록.
   - **규칙**:
     - 해당 도메인(`category`) 내에서만 사용된다.
     - 비즈니스 로직이 필요한 경우, 상위 Page에서 Props로 받거나 전용 Hook을 사용한다.
     - 파일명 예시: `VideoPlayer.tsx`, `AnswerForm.tsx`

3. **공용 UI 컴포넌트 (`components/ui/`)**
   - **역할**: 디자인 시스템의 원자(Atom). (`shadcn/ui` 컴포넌트들)
   - **규칙**:
     - **도메인 로직(비즈니스)을 절대 포함하지 않는다.**
     - `className` prop을 통해 외부에서 스타일 확장이 가능해야 한다.
     - 파일명 예시: `Button.tsx`, `Dialog.tsx`

#### 6.2.3 훅(Hook) & API 레이어 설계

- **API Layer (`src/api/*.ts`)**
  - 순수 함수(Pure Function)로 구성된 `fetch` 호출부.
  - React 의존성(State, Hook)이 전혀 없어야 한다.
  - `client.ts`를 import하여 사용한다.

- **Query Hook (`category/*/hook/`)**
  - **TanStack Query**를 래핑하여 데이터 상태(`isLoading`, `data`, `error`)를 제공하는 훅.
  - 예: `useVideoListQuery`, `useVideoProgressMutation`
  - 이 계층에서 **API 응답 타입(DTO)**과 **프론트엔드 뷰 모델** 간의 변환이 필요하다면 수행한다. (단, 기본적으로는 DTO 구조를 그대로 사용하는 것을 권장)

- **Logic Hook (`category/*/hook/`)**
  - UI 상태(Form, Modal open/close)와 사용자 인터랙션 핸들러를 캡슐화.
  - Page 컴포넌트가 "Controller" 역할을 하지 않도록 로직을 분리해내는 핵심 계층.
  - 예: `useSignupForm`, `useVideoPlayerController`

---

### 6.3 라우팅 & 접근 제어

> 목적: 5. 기능 & API 로드맵의 “화면 경로”를 기준으로, **Code Splitting이 적용된 React Router 트리**와 **엄격한 접근 제어(Auth/Admin Guard)**를 정의한다.

#### 6.3.1 라우트 매핑 원칙 (Lazy Loading 필수)

- **라우트 정의 위치**
  - `src/app/router.tsx` 에서 **전체 라우트 트리**를 정의한다.
  - **성능 원칙**: 모든 페이지 컴포넌트는 `React.lazy`로 import하여, 초기 번들 사이즈를 최소화해야 한다.

- **파일명 패턴 (예시)**
  - `/login` → `category/auth/page/LoginPage.tsx`
  - `/videos/:video_id` → `category/video/page/VideoDetailPage.tsx`
  - `/admin/users` → `category/admin/page/AdminUserListPage.tsx`
  - *파일명은 PascalCase를 따른다.*

- **라우트 구성 예시 (Strict Code Splitting)**

```tsx
// app/router.tsx
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Suspense, lazy } from "react";
import { AppShell } from "@/components/layout/AppShell"; // layout 경로 수정됨
import { RequireAuth } from "./route_guard_auth";
import { RequireAdmin } from "./route_guard_admin";
import { LoadingSpinner } from "@/components/shared/LoadingSpinner";

// ★ 핵심: 모든 페이지는 Lazy Load 처리
const LoginPage = lazy(() => import("@/category/auth/page/LoginPage"));
const SignupPage = lazy(() => import("@/category/auth/page/SignupPage"));
const VideoListPage = lazy(() => import("@/category/video/page/VideoListPage"));
const VideoDetailPage = lazy(() => import("@/category/video/page/VideoDetailPage"));
const MePage = lazy(() => import("@/category/user/page/MePage"));
const AdminUserListPage = lazy(() => import("@/category/admin/page/AdminUserListPage"));

export function AppRouter() {
  return (
    <BrowserRouter>
      {/* Suspense: Lazy Loading 중 보여줄 Fallback UI */}
      <Suspense fallback={<LoadingSpinner fullScreen />}>
        <AppShell>
          <Routes>
            {/* Public Routes */}
            <Route path="/login" element={<LoginPage />} />
            <Route path="/signup" element={<SignupPage />} />

            {/* Protected Routes (Member) */}
            <Route element={<RequireAuth />}>
              <Route path="/me" element={<MePage />} />
              <Route path="/videos" element={<VideoListPage />} />
              <Route path="/videos/:video_id" element={<VideoDetailPage />} />
            </Route>

            {/* Admin Routes (RBAC) */}
            <Route element={<RequireAdmin />}>
              <Route path="/admin/users" element={<AdminUserListPage />} />
              {/* ... other admin routes */}
            </Route>
            
            {/* 404 Handling */}
            <Route path="*" element={<div>Page Not Found</div>} />
          </Routes>
        </AppShell>
      </Suspense>
    </BrowserRouter>
  );
}
```

> 실제 구현 시 파일명/컴포넌트명은 이 문서의 **네이밍 규칙(3.2.4 프론트엔드 네이밍)** 을 따른다.

#### 6.3.2 접근 제어 패턴 (Auth / Admin 가드)

- **공통 개념**
  - 백엔드의 상태축을 프론트에서 `useAuth()` 훅을 통해 `pass / stop / forbid` 상태로 해석한다.
  - **권한 확인 로직은 `hooks/use_auth.ts`에 중앙화한다.**

- **`RequireAuth` (사용자 로그인 필수)**
  - **로직**:
    - `authStatus === "pass"` (토큰 유효) AND `user_state === "on"` (계정 활성)
  - **실패 시 처리**:
    - `authStatus === "stop"` (미로그인/토큰만료) → 로그인 페이지로 이동 (`state: { from: location }` 전달)
    - `user_state !== "on"` (정지/탈퇴) → "계정 비활성화" 안내 페이지로 이동.

- **`RequireAdmin` (관리자 RBAC)**
  - **로직**:
    - `RequireAuth` 통과 AND `user_auth_enum` IN `['HYMN', 'admin', 'manager']`
  - **실패 시 처리**:
    - 인증은 되었으나 권한 부족 → **403 Forbidden** 페이지 (커스텀 에러 페이지) 렌더링.
    - *절대 로그인 페이지로 튕겨내지 않는다 (무한 루프 방지).*

- **Redirect 정책 (Guest Guard)**
  - 로그인 상태(`pass`)인 사용자가 `/login` 또는 `/signup` 접근 시:
    - 일반 사용자 → `/videos` (메인)으로 리다이렉트
    - 관리자 → `/admin/dashboard` 등으로 리다이렉트 (선택 사항)

---

### 6.4 상태 관리 & API 연동 패턴

> 목적: **TanStack Query(Server State)**와 **Zustand(Client State)**를 중심으로, 백엔드 API와 프론트엔드 UI를 **선언적(Declarative)**으로 연결한다.

#### 6.4.1 인증 상태 관리 (Zustand + AuthProvider)

- **토큰/세션 보관 전략 (Strict)**
  - **Access Token**: 메모리(Zustand Store) 또는 React Query 캐시에만 보관. (LocalStorage 저장 금지 - XSS 취약)
  - **Refresh Token**: `httpOnly` 쿠키로 백엔드가 설정. (JS 접근 불가)

- **Auth Store 구조 (`hooks/use_auth.ts`)**
  - `Zustand`를 사용하여 전역 인증 상태를 관리한다.
  - **State**:
    - `user`: User DTO | null
    - `authStatus`: `"pass"`(인증됨) | `"stop"`(미인증/만료) | `"forbid"`(권한부족)
    - `isAdmin`: boolean (Helper Getter)
  - **Actions**:
    - `login(token, user)`: 상태 업데이트 및 토큰 메모리 저장
    - `logout()`: 상태 초기화 및 `/auth/logout` API 호출
    - `refresh()`: 앱 초기 진입 시 `/auth/refresh` 호출하여 세션 복구

#### 6.4.2 공통 API 클라이언트 (`src/api/client.ts`)

- **역할**
  - `fetch` API 기반의 Singleton 인스턴스.
  - **Interceptor**: 요청 시 헤더에 `Authorization: Bearer {token}` 자동 주입.
  - **Error Handling**: HTTP 에러를 `AppError` 객체로 변환하여 throw.

- **네이밍 규칙 (Strict)**
  - **Request/Response DTO는 백엔드와 동일하게 `snake_case`를 사용한다.**
  - 프론트엔드에서 `camelCase`로 변환하지 않는다. (불필요한 연산 및 매핑 오버헤드 제거)

- **에러 매핑 규칙 (Global Error Boundary)**
  - `401 Unauthorized` → `authStatus`를 `"stop"`으로 변경하고 로그인 모달/페이지 유도.
  - `403 Forbidden` → `authStatus`를 `"forbid"`로 변경.
  - `5xx Server Error` → Toast 메시지로 "잠시 후 다시 시도해주세요" 출력.

#### 6.4.3 도메인별 훅 패턴 (React Query & Custom Hooks)

> **원칙**: UI 컴포넌트는 `useEffect`를 사용하지 않고, 아래 훅을 통해 데이터를 구독한다.

- **Query Hook (Data Fetching)**
  - **TanStack Query**를 사용하여 서버 상태를 관리한다.
  - 파일 위치: `category/*/hook/use[Domain]Query.ts`
  - 예시:
    ```typescript
    // useVideoListQuery.ts
    export const useVideoListQuery = (params) => {
      return useQuery({
        queryKey: ["videos", params],
        queryFn: () => fetchVideos(params), // api/video.ts 호출
        staleTime: 1000 * 60 * 5, // 5분간 캐시 유지 (데이터 절약)
      });
    };
    ```

- **Mutation Hook (Data Update)**
  - 데이터 변경(POST/PUT/DELETE)을 담당한다.
  - 예시:
    ```typescript
    // useVideoProgressMutation.ts
    export const useVideoProgressMutation = () => {
      const queryClient = useQueryClient();
      return useMutation({
        mutationFn: updateVideoProgress,
        onSuccess: () => {
          queryClient.invalidateQueries(["videos"]); // 목록 갱신
        }
      });
    };
    ```

- **Controller Hook (UI Logic)**
  - 폼 핸들링, 모달 제어 등 순수 클라이언트 로직.
  - `useForm`(React Hook Form)과 `zod` 스키마를 결합하여 사용한다.
  - 예: `useSignupForm`, `useVideoPlayerController`

#### 6.4.4 상태축과 UI 상태 매핑

> **5. 기능 & API 로드맵**의 상태축을 프론트엔드 변수로 변환하는 규칙이다.

- **Request 상태 (React Query 상태 매핑)**
  - `pending` → `isLoading` (스피너 표시)
  - `error` → `isError` (에러 메시지/재시도 버튼 표시)
  - `success` → `data` (콘텐츠 렌더링)
  - `retryable` → React Query의 `retry` 옵션으로 자동 처리

- **Course 상태 (접근 권한 계산)**
  - `/videos/{id}` 등 유료 콘텐츠 접근 시 `Course` 축(`buy/taster/buy-not`)을 계산하는 로직은 **Selector** 또는 **Helper Hook**으로 분리한다.
  - 예: `useCourseAccess(videoId)`
    - Return: `{ canPlay: boolean, showPaywall: boolean }`
    - 로직: 내 수강권 목록과 해당 비디오의 `is_free` 여부를 대조.

- **Form 상태**
  - React Hook Form의 `formState`를 그대로 활용한다.
  - `isSubmitting` (전송 중), `isValid` (유효성 검증 통과), `errors` (필드별 에러)

### 6.5 UI/UX & Tailwind 규칙 (shadcn/ui System)

> 목적: **shadcn/ui** 디자인 시스템을 기반으로, 모바일 퍼스트 및 의미론적(Semantic) 스타일링 규칙을 정의하여 일관성과 생산성을 확보한다.

#### 6.5.1 디자인 시스템 철학 (Shadcn First)

- **Mobile First**: 모든 레이아웃은 모바일(`sm`)에서 시작하여 태블릿(`md`), 데스크톱(`lg`)으로 확장한다.
- **Semantic Styling**: 색상 코드를 직접 사용하지 않고, 역할에 따른 변수를 사용한다.
  - ❌ Bad: `bg-blue-600`, `text-gray-500`
  - ⭕ Good: `bg-primary`, `text-muted-foreground`
- **Atomic Components**:
  - 버튼, 인풋 등을 처음부터 만들지 않는다.
  - `components/ui/`에 설치된 **shadcn 컴포넌트**(`<Button>`, `<Input>`, `<Card>`)를 조립하여 화면을 구성한다.

#### 6.5.2 레이아웃 & 그리드

- **AppShell (`components/layout/AppShell.tsx`)**
  - 앱의 최상위 껍데기.
  - 구성:
    - **Header**: 로고 + 햄버거 메뉴(모바일) / 네비게이션(데스크톱) + 유저 프로필
    - **Main**: `max-w-screen-xl mx-auto px-4` (콘텐츠 중앙 정렬 및 가로 여백 확보)
    - **Footer**: (선택적) 법적 고지 등

- **반응형 전략**
  - **Grid**: `grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6` 패턴을 기본으로 한다.
  - **Spacing**: 모바일에서는 `gap-4`, 데스크톱에서는 `gap-6` 이상을 사용하여 시원한 느낌을 준다.

#### 6.5.3 Tailwind & Color System (Theme)

- **색상 토큰 (globals.css 기반)**
  - `primary`: 브랜드 메인 컬러 (Amazing Korean Blue) → 주요 액션 버튼
  - `secondary`: 보조 컬러 → 취소/서브 버튼
  - `destructive`: 위험/삭제 → `bg-red-600` 계열
  - `muted`: 비활성/배경 → `bg-gray-100` 계열
  - `accent`: 강조 포인트 → 학습 완료 체크 등

- **타이포그래피**
  - `h1` (Page Title): `text-2xl font-bold tracking-tight md:text-3xl`
  - `h2` (Section): `text-xl font-semibold tracking-tight`
  - `p` (Body): `leading-7 [&:not(:first-child)]:mt-6`
  - `small` (Caption): `text-sm font-medium leading-none`
  - `muted` (Subtext): `text-sm text-muted-foreground`

- **유틸리티 함수 (`cn`)**
  - Tailwind 클래스 병합을 위해 `lib/utils.ts`의 `cn()` 함수를 적극 활용한다.
  - 예: `<div className={cn("flex items-center", isMobile && "flex-col")}>`

#### 6.5.4 주요 UI 패턴 가이드

- **Card Pattern (목록 아이템)**
  - `Card`, `CardHeader`, `CardContent`, `CardFooter` 컴포넌트 조합 사용.
  - 썸네일(이미지/비디오)은 **`aspect-video` (16:9 비율)** 클래스를 사용하여 레이아웃 이동(CLS)을 방지한다.

- **Form Pattern (로그인/입력)**
  - **React Hook Form** + **zod** + **shadcn Form** 조합 필수.
  - `<Form>` 감싸기 → `<FormField>` → `<FormItem>` → `<FormControl>` 구조 준수.
  - 에러 메시지는 `<FormMessage />` 컴포넌트로 자동 노출.

- **Feedback (Toast)**
  - 사용자 액션 결과는 `alert()` 대신 **Toast** (`hooks/use-toast.ts`)를 사용한다.
  - 성공: `toast({ title: "저장되었습니다.", variant: "default" })`
  - 에러: `toast({ title: "오류 발생", variant: "destructive" })`

#### 6.5.5 미디어 & 데이터 최적화 (UX)

- **이미지 (Image)**
  - 포맷: `WebP` 사용 권장.
  - 로딩: `loading="lazy"` 속성 필수.
  - 플레이스홀더: 이미지가 로드되기 전 `bg-muted` 영역을 미리 잡아준다.

- **비디오 (Video)**
  - 목록 화면에서는 무거운 `Vimeo Player` 대신 **가벼운 썸네일 이미지**만 보여준다.
  - 사용자가 "재생" 버튼을 클릭했을 때만 플레이어 SDK를 로드한다 (Lazy Interaction).

---

### 6.6 프론트 테스트 & 빌드/배포 (요약)

> 목적: Vite + React 환경에서 **Type Safety**를 보장하며, 빌드된 정적 자원(`dist/`)을 운영 환경에 일관되게 배포하는 파이프라인을 정의한다.

#### 6.6.1 로컬 개발 플로우

- **패키지 관리**
  - `npm`을 표준 패키지 매니저로 사용한다. (`package-lock.json` 공유)
  - 설치: `npm install`
  - shadcn 컴포넌트 추가: `npx shadcn@latest add [component-name]`

- **환경 변수 (.env)**
  - `.env.local` (로컬 전용, gitignore 대상)
  - `.env.production` (운영 전용)
  - 필수 변수:
    - `VITE_API_BASE_URL`: 백엔드 API 주소 (예: `http://localhost:8080` 또는 `https://api.amazingkorean.net`)
    - *Client 코드에서는 `import.meta.env.VITE_API_BASE_URL`로 접근.*

- **개발 서버 실행**
  - `npm run dev` (기본 포트: 5173)

#### 6.6.2 빌드 & 배포 전략

- **빌드 커맨드 (Strict)**
  - `npm run build` 실행 시:
    1.  `tsc -b` (TypeScript 컴파일 검사)가 먼저 실행되어야 한다. **타입 에러 발생 시 빌드는 실패해야 한다.**
    2.  Vite가 프로덕션용 최적화(Minify, Tree Shaking)를 수행하고 `dist/` 폴더를 생성한다.

- **SPA 서빙 전략 (SPA Fallback)**
  - 프론트엔드는 **Single Page Application**이므로, **모든 404 요청을 `index.html`로 리다이렉트**해야 한다.
  - **Nginx 배포 시**: `try_files $uri $uri/ /index.html;` 설정 필수.
  - **Rust(Axum) 통합 배포 시**: 정적 파일 서빙 핸들러에서 Fallback 경로 설정 필요.

#### 6.6.3 품질 보증 (QA) & 스모크 체크

- **정적 분석 (CI Gate)**
  - `npm run lint`: ESLint (코드 스타일 및 잠재적 버그 검사)
  - `npm run typecheck`: TypeScript 타입 정합성 검사 (필수)

- **수동 스모크 테스트 (Release Checklist)**
  - 배포 전 아래 시나리오를 **반드시 1회 수동 확인**한다.
    1.  **진입**: 랜딩 페이지 로딩 및 폰트/이미지 깨짐 확인.
    2.  **인증**: 로그인(토큰 발급) → 새로고침 시 로그인 유지 확인.
    3.  **영상**: 비디오 목록 로딩 → 상세 페이지 진입 → 플레이어 재생 확인.
    4.  **라우팅**: 잘못된 URL 입력 시 404 페이지(또는 리다이렉트) 동작 확인.

#### 6.6.4 향후 확장 계획 (Roadmap)

- **자동화 테스트 도입 (Phase 3 이후)**
  - **Unit Test**: `Vitest` 도입. (유틸 함수 및 복잡한 Hook 로직 검증)
  - **E2E Test**: `Playwright` 도입. (핵심 비즈니스 플로우 자동화)

- **CI/CD 파이프라인**
  - GitHub Actions 연동:
    - Push 시: `Lint` + `Typecheck` 자동 실행.
    - Tag/Merge 시: `Build` 수행 후 Docker Image 생성 또는 S3 업로드.

## 7. 작업 방식 / 엔지니어링 가이드 (요약)

> 기존 `AMK_ENGINEERING_GUIDE.md` + `README_for_assistant.md` + `AMK_PROJECT_JOURNAL.md`의 “결정/규칙” 부분을 통합한 섹션.

### 7.1 작업 원칙

1. **문서 우선**
   - 스펙/기능/규칙은 항상 이 문서를 기준으로 한다.
2. **파일 전체 교체본**
   - LLM/Gemini에게 코드 패치를 요청할 때는 **항상 “파일 전체 교체본”**으로 요청/응답한다.
3. **정적 가드 필수**
   - `cargo fmt -- --check`
   - `cargo clippy -- -D warnings`
   - `cargo check`
   - 위 3개를 모두 통과해야 PR/머지 가능.
4. **마이그레이션 규칙**
   - 이미 적용된 마이그레이션 파일은 **수정/이름 변경 금지**.
   - 변경이 필요하면 항상 **새 마이그레이션 추가**.
   - SQLx 마커는 `--! up` / `--! down`만 사용 (ASCII 하이픈).
   - 적용 순서 : 1) USERS → 2) LOGIN → 3) VIDEO → 4) STUDY → 5) LESSON
5. **로그/감사**
   - 도메인별 변경 사항 기록 : `USERS_LOG`, `LOGIN_LOG`, `VIDEO_LOG`, `STUDY_TASK_LOG`
   - 관리자 활동 사항 기록 : `ADMIN_USERS_LOG`, `ADMIN_VIDEO_LOG`, `ADMIN_STUDY_LOG`, `ADMIN_LESSON_LOG`
6. **보안 (기본)**  
   - 계정 상태
     - `user_state == 'on'` 인 사용자만 로그인/액세스 허용.
     - 로그인 시점 + 모든 인증 보호 엔드포인트에서 `user_state`를 다시 검증한다.
   - 인증 토큰
     - 모든 보호 엔드포인트는 `Authorization: Bearer <ACCESS_TOKEN>`를 요구한다.
     - 토큰 안의 `sub`(user_id)는 **유일한 신뢰 가능한 사용자 식별자**로 사용하고,  
       요청 바디/쿼리로 들어오는 user_id는 신뢰하지 않는다.
   - 비밀번호 및 민감정보
     - 비밀번호는 Argon2 등 안전한 해시로만 저장하고, **원문은 절대 저장/로그에 남기지 않는다.**
     - USERS_LOG, ADMIN_*_LOG 등 어떤 로그에도 비밀번호/토큰/쿠키 값은 남기지 않는다.
   - 세션/리프레시 토큰
     - 세션/리프레시는 Redis 키(`ak:session:*`, `ak:refresh:*`)를 사용한다.
     - 리프레시는 **사용 시 로테이션(rotate-on-use)** 하고,  
       재사용이 감지되면 해당 세션/사용자의 관련 세션을 일괄 폐기하는 정책과 연동한다.
   - 관리자 RBAC
     - 관리자 롤은 `HYMN / admin / manager` 를 기준으로 한다.
     - `/admin/**` 경로는 기본적으로 **“허용된 롤만 접근 가능”**(default deny) 원칙을 따른다.
     - 롤별 세부 권한 매트릭스는 **Open Questions 섹션**에서 정의/업데이트 한다.
   - 통신
     - 운영 환경에서는 반드시 HTTPS를 사용하고, 토큰/세션 ID를 URL(query string)에 노출하지 않는다.

### 7.2 개발 플로우

1. 개발 사항에 대한 이 문서(**AMK_API_MASTER.md**) 확인 및 참조
2. 1) 기존 개발 사항 : **AMK_API_MASTER.md** 확인 및 참조 후 해당 개발 사항 작업 진행
   2) 신규 개발 사항 : 신규 API 명시 → **AMK_API_MASTER.md** 확인 및 참조 → **AMK_API_MASTER.md** 형식으로 문서 업데이트 → 해당 개발 사항 작업 진행
3. GEMINI 템플릿에 따라 patch prompt 작성
4. 코드/마이그레이션 생성
5. 정적 가드(cargo check/fmt/clippy) + 최소 스모크 테스트(cURL or 스크립트)
6. 관련 로드맵 행의 “기능 완료” 체크박스 업데이트

### 7.3 DTO/검증 규칙 (요약)

- 공통 원칙
  - HTTP 경계에서는 항상 **DTO(struct)** 를 사용하고,  
    내부 도메인 타입과 분리한다.
  - 필수/옵션 필드, 기본값, 검증 규칙은 **DTO에 명시**한다.

- 문자열 필드
  - `trim` 후 검증을 기준으로 한다.
  - 길이 제한을 명시한다. (예: 이메일/닉네임 등은 최소/최대 길이 지정)
  - 공백만 있는 문자열은 “빈 값”으로 처리하고, 필요한 경우 400으로 반환한다.

- 이메일
  - RFC 이메일 형식 검증 (예: `validator` 크레이트).
  - 대소문자는 구분하지 않는 것을 기본 가정으로 한다.

- 비밀번호
  - 최소 길이/복잡도는 프로젝트 정책으로 정의 (예: 최소 8자 이상).
  - DTO에서 문자열 길이만 검증하고, **해시는 service 계층에서 수행**한다.
  - 비밀번호 원문은 절대 로그/이력에 남기지 않는다.

- 날짜
  - DTO에서는 `chrono::NaiveDate` 사용.
  - DB에는 `DATE` 또는 `TIMESTAMPTZ`로 캐스팅한다.
  - 잘못된 날짜 형식은 400 + `invalid_argument` 로 응답한다.

- Enum 필드
  - DTO에서는 enum 타입을 사용하거나, 문자열 입력을 enum으로 매핑한다.
  - 정의되지 않은 값이 들어오면 400 + `invalid_argument`.
  - enum 값은 **이 문서 4. 데이터 모델 개요의 enum 정의**를 기준으로 한다.

- ID / 페이징
  - ID는 음수가 아닌 정수로 검증한다. (0 또는 음수는 400)
  - 페이징 파라미터
    - `page >= 1`, `1 <= size <= 최대값(예: 100)`
    - 위반 시 400 + `invalid_argument`.

### 7.4 서비스 계층 및 파일 구조

> 기준 경로: `src/` (예: `\\wsl.localhost\Ubuntu\home\kkryo\dev\amazing-korean-api\src`)

#### 7.4.1 디렉터리 구조(요약)

- `src/api`
  - `admin/{lesson,study,user,video}/`
    - `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs`
  - `auth/`
    - `dto.rs`, `extractor.rs`, `handler.rs`, `jwt.rs`, `repo.rs`, `router.rs`, `service.rs`, `token_utils.rs`, `mod.rs`
  - `health/`
    - `handler.rs`, `mod.rs`
  - `lesson/`, `study/`, `user/`, `video/`
    - 각 도메인별 `dto.rs`, `handler.rs`, `repo.rs`, `router.rs`, `service.rs`, `mod.rs`
  - `scripts/`
    - `db_fastcheck.sh`
  - `mod.rs` (api 루트 모듈)

- 루트 파일
  - `config.rs`  : 환경 변수/설정 로딩
  - `docs.rs`    : OpenAPI/Swagger 정의
  - `error.rs`   : 공통 에러 타입(AppError 등)
  - `main.rs`    : 엔트리 포인트(서버 부트스트랩)
  - `state.rs`   : `AppState` 정의(DB 풀, Redis, 설정 등)
  - `types.rs`   : 공용 타입/별칭

#### 7.4.2 계층별 역할

- `dto.rs`
  - 요청/응답 DTO 정의
  - `serde`/`validator`/`utoipa::ToSchema` 등을 사용
  - **핵심:** HTTP 경계에서만 쓰이는 타입(내부 도메인 모델과 분리)

- `handler.rs`
  - Axum 핸들러 함수(라우트별 엔드포인트 구현)
  - 역할:
    - Path/Query/Json 등 요청 파라미터 파싱
    - DTO 검증 결과 처리
    - `Claims`/`AppState` 추출
    - **비즈니스 로직은 직접 수행하지 않고 `service`를 호출**

- `service.rs`
  - 도메인 비즈니스 로직의 중심 계층
  - 역할:
    - 유즈케이스 단위 메서드 (예: `signup_user`, `update_video`, `submit_answer`)
    - 여러 `repo` 호출을 묶어 **트랜잭션 경계**를 형성
    - 검증/권한 체크/상태 전이 규칙을 여기서 처리
  - 원칙:
    - HTTP/프레임워크 의존성 없음 (가능한 한 순수 로직 유지)
    - handler는 얇게, service는 두껍게

- `repo.rs`
  - DB 접근 전담 계층(sqlx 쿼리)
  - 역할:
    - SELECT/INSERT/UPDATE/DELETE 및 저장 프로시저/함수 호출
    - 입력/출력을 struct로 매핑
  - 원칙:
    - 비즈니스 규칙은 넣지 않는다(검증/권한/상태 전이는 service 담당)
    - 필요 시 트랜잭션 핸들러(`&mut Transaction<'_, Postgres>`)를 인자로 받아 사용

- `router.rs`
  - 도메인별 서브 라우터 정의
  - 역할:
    - 각 HTTP 메서드 + 경로에 `handler`를 매핑
    - 도메인 공통 미들웨어(예: 관리자 인증, 로깅) 부착
  - 반환 타입:
    - `Router<AppState>` (상위 `api::mod.rs`에서 `.nest("/videos", video::router())` 형태로 사용)

- `mod.rs`
  - 각 도메인 모듈의 루트
  - 역할:
    - `pub mod dto; pub mod handler; ...` 선언
    - `pub fn router() -> Router<AppState>` 같은 진입 함수 노출
    - 상위 모듈에서 사용할 공개 타입/함수 re-export

#### 7.4.3 특수 모듈(auth, health, scripts)

- `api/auth/`
  - `extractor.rs` : `Claims` 등 인증 관련 Axum extractor
  - `jwt.rs`       : JWT 인코딩/디코딩, 키 관리
  - `token_utils.rs`: 액세스/리프레시 토큰 생성·검증 유틸
  - 나머지(`dto/handler/repo/service/router`)는 일반 도메인과 동일 패턴

- `api/health/`
  - `handler.rs`: `/health/live`, `/health/ready` 등 헬스체크 엔드포인트
  - `mod.rs`: 헬스 라우터 노출

- `api/scripts/db_fastcheck.sh`
  - 로컬/CI용 DB 빠른 연결 확인 스크립트
  - `sqlx` 마이그레이션 실행 전 DB 준비 상태 점검 등에 사용

### 7.5 트랜잭션 패턴

> 목표: **여러 DB 작업을 한 덩어리(원자 단위)로 처리**해서  
> 중간에 에러가 나면 전부 되돌리고, 성공하면 전부 반영되도록 한다.

#### 7.5.1 언제 트랜잭션을 쓰는가

- 대표 사용 사례
  - `USERS` + `USERS_LOG` 같이 **본 테이블 + 로그 테이블**을 함께 갱신할 때
  - `VIDEO` / `STUDY` / `LESSON` 데이터를 수정하면서 **관련 `ADMIN_*_LOG`까지 함께 기록**할 때
  - 한 HTTP 요청 안에서 **여러 테이블을 연속으로 변경**해야 할 때
  - 결제/수강권 등 **비즈니스 일관성이 특히 중요한 작업** (PAY + COURSE/COURSE_PROGRESS 등)
- 원칙
  - “이 중 하나만 반영되고 나머지는 실패하면 안 되는 작업”이면 **트랜잭션을 쓴다.**
  - “로그/통계가 약간 늦게 쌓여도 되느냐?”가 아니라  
    **“이 변경과 로그/통계가 항상 같이 있어야 하느냐”** 기준으로 판단한다.
  - 현재 AMK의 기본 방향:
    - **핵심 비즈니스 상태 + 그에 대한 로그**는 한 트랜잭션 안에서 함께 처리한다.

#### 7.5.2 어디에서 트랜잭션을 여는가

- handler 계층
  - 트랜잭션을 **직접 열지 않는다.**
  - 역할:
    - HTTP 요청 파싱 (path/query/body)
    - 인증/인가 정보 추출 (Claims 등)
    - 기본 수준의 유효성 검사
    - → 이후 **service** 함수 호출
- service 계층
  - **트랜잭션 시작/커밋/롤백의 책임을 가진다.**
  - 하나의 유즈케이스(예: `/users` signup, `/users/me` update 등)에 필요하다면  
    service 함수에서 트랜잭션을 열고, 도메인 repo들을 호출한다.
  - 패턴 예시:

    ```rust
    pub async fn update_user_and_log(
        state: &AppState,
        input: UpdateUserInput,
        actor_id: i64,
    ) -> AppResult<User> {
        // 1) 트랜잭션 시작
        let mut tx = state.db.begin().await?;

        // 2) 메인 상태 변경
        let user = user_repo::update_user(&mut tx, &input).await?;

        // 3) 로그 기록
        user_repo::insert_users_log(&mut tx, &user, actor_id, "update_profile").await?;

        // 4) (필요 시) 통계/기타 파생 데이터 갱신

        // 5) 전부 성공했으면 커밋
        tx.commit().await?;

        Ok(user)
    }
    ```
    
  - 중간에 에러가 나면 `commit()`에 도달하지 못하고,
    트랜잭션 객체가 drop되면서 전체 작업이 **롤백**된다고 보는 것을 기본 전제로 한다.
- repo 계층
  - “어떤 실행 컨텍스트(DB 연결 or 트랜잭션)를 받을지”만 신경 쓴다.
  - 트랜잭션을 시작/커밋하지 않고, **넘겨받은 executor 안에서만 쿼리 실행**한다.
    ```rust
    pub async fn update_user<'e, E>(
        executor: E,
        input: &UpdateUserInput,
    ) -> AppResult<UserRow>
    where
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        // UPDATE users SET ... WHERE user_id = ...
        // ...
    }
    ```
  - 같은 repo 함수가 트랜잭션 안/밖 양쪽에서 재사용될 수 있도록  
    **`&PgPool` / `&mut Transaction<'_, Postgres>` 모두를 받을 수 있는 제네릭 executor 패턴**을 사용한다.

#### 7.5.3 API upsert 패턴 (예: 비디오 진도 저장)

- 비디오 진도 API는 DB 함수 형태로 upsert를 처리하는 것을 기본 패턴으로 한다.
  - 예: `api_upsert_video_progress(user_id, video_id, progress, ...)`
- 이 함수 안에서:
  - 새 기록이면 `INSERT`
  - 기존 기록이면 `UPDATE`
  - 필요한 경우 `VIDEO_LOG` / `VIDEO_STAT_DAILY` 등 연관 정보까지 함께 갱신
  - → 를 **한 번에 처리**하도록 설계한다. (DB 함수 내부가 하나의 트랜잭션 역할)
- 서비스 계층에서는:
  - 1) 입력 검증 (0~100 범위, 사용자 권한, 소유권 등)
  - 2) `api_upsert_video_progress(...)`를 **한 번 호출하는 것**을  
       “이 작업의 트랜잭션 단위”로 본다.
- 한 HTTP 요청에서
  - “진도 upsert + 다른 테이블 변경”이 함께 필요하다면,
  - 7.5.2 패턴대로 **서비스에서 트랜잭션을 열고**, 그 안에서
    - `api_upsert_video_progress(...)`
    - + 기타 repo 함수
    - 를 함께 호출한다.

#### 7.5.4 트랜잭션 내부 순서 패턴

> 기본 순서: **검증 → 메인 변경 → 로그 → (통계/파생) → 커밋**

- 1) 검증 / 현재 상태 조회
  - 예:
    - 대상 레코드 존재 여부 확인
    - 소유권/권한 체크
    - 중복 여부(이메일 중복 등)
  - 주로 `SELECT ... FOR UPDATE` 또는 단순 `SELECT` 로 처리
- 2) 메인 상태 변경
  - 비즈니스에 직접적인 영향을 주는 테이블 변경
    - 예: `USERS`, `USERS_SETTING`, `STUDY_TASK_STATUS`, `LESSON_PROGRESS` 등
  - `INSERT` / `UPDATE` / `DELETE` 중심
- 3) 로그/감사 기록
  - `USERS_LOG`, `LOGIN_LOG`, `VIDEO_LOG`, `STUDY_TASK_LOG`, `ADMIN_*_LOG` 등
  - 가능하면 **before/after 스냅샷, actor, action**을 함께 저장
- 4) 통계/파생 데이터(선택)
  - 집계/통계용 테이블 (`VIDEO_STAT_DAILY` 등)
  - 필요 시에만 갱신, 너무 복잡해지면 추후 비동기/배치로 분리 검토
- 5) 커밋
  - 위 단계들(1~4)이 모두 성공한 경우에만 `commit()`
  - 중간에 하나라도 실패하면 → **전체 롤백**  
    → 실제 데이터와 로그/통계 간 **일관성 유지**

- 예시 (USERS + USERS_LOG):

  ```rust
  pub async fn update_profile(
      state: &AppState,
      req: UpdateProfileReq,
      actor_id: i64,
  ) -> AppResult<UserMeRes> {
      let mut tx = state.db.begin().await?;

      // 1) 현재 상태 조회 (검증)
      let before = user_repo::find_user_for_update(&mut tx, req.user_id).await?;

      // 2) 메인 상태 변경
      let after = user_repo::update_user_profile(&mut tx, &req).await?;

      // 3) 로그 기록
      user_repo::insert_users_log(&mut tx, &before, &after, actor_id, "update_profile").await?;

      // 4) (필요 시) 통계/파생 데이터 갱신

      // 5) 커밋
      tx.commit().await?;

      Ok(UserMeRes::from(after))
  }

> **요약**  
> 하나의 유즈케이스 안에서 여러 DB 작업이 필요하면 **service 레이어에서 트랜잭션을 열고**,  
> **검증 → 메인 변경 → 로그 → (통계) → 커밋** 순서로 실행한다.  
> 이렇게 하면 사용자 입장에서는 “요청 한 번”이,  
> 관리/운영 입장에서는 **일관성 있는 상태 + 신뢰할 수 있는 로그**로 남는다.

### 7.6 테스트 & 자동화

> 목표: **“사람이 실수로 빼먹지 않게” 최소한의 테스트를 자동으로 돌리는 것**  
> (처음에는 가볍게 시작하고, 점진적으로 확장한다.)

#### 7.6.1 최소 정적 가드 (로컬 + CI 공통)

- 항상 돌려야 하는 기본 가드:
  - `cargo fmt -- --check`  : 포맷 일관성
  - `cargo clippy -- -D warnings` : 잠재 버그/나쁜 패턴 차단
  - `cargo check` : 타입/빌드 오류 사전 검출
- 사용 방식:
  - **로컬**: 기능 개발 후 커밋 전에 수동 실행
  - **CI**: PR 생성/업데이트 시 자동 실행 (향후 GitHub Actions 등으로 구성 예정)

#### 7.6.2 스모크 테스트 (기능 단위 확인)

- 목적:
  - “서버가 뜨고, 대표적인 API 몇 개는 정상 응답을 준다”를 빠르게 확인하기 위함.
- 대상:
  - 대표 엔드포인트
    - health: `/health`
    - auth: `/auth/login`, `/auth/refresh`
    - user: `/users`, `/users/me`
    - videos: `/videos`, `/videos/{id}`, `/videos/{id}/progress`
    - admin: `/admin/videos`, `/admin/studies` 등
- 형태:
  - `scripts/` 폴더에 cURL 기반 스모크 스크립트를 둔다.
    - 예: `scripts/smoke_health.sh`, `scripts/smoke_auth.sh`, `scripts/smoke_videos.sh`
  - 각 스크립트는 **성공 케이스 + 대표 에러 케이스 1개 정도**를 포함한다.
    - 예: 토큰 없이 `/users/me` 호출 → 401 확인
- 실행 타이밍:
  - **로컬**: 큰 변경(예: 도메인 추가, 마이그레이션 변경) 후 수동 실행
  - **CI (향후)**: main 브랜치에 머지되기 전 1회 실행을 목표로 한다.

#### 7.6.3 자동화 레벨 (초기 방침)

- 1단계: 로컬 스크립트
  - 개발자는 다음을 수동으로 실행한다.
    - `./scripts/db_fastcheck.sh` (DB 준비 상태 점검)
    - `cargo fmt -- --check`
    - `cargo clippy -- -D warnings`
    - `cargo check`
    - 필요 시 `./scripts/smoke_*.sh`
- 2단계: CI 연계 (향후)
  - PR 생성/업데이트 시:
    - `fmt` / `clippy` / `check` 자동 실행
  - main 브랜치 머지 전:
    - 최소 한 개 이상의 스모크 스크립트 실행 (예: `smoke_health.sh`, `smoke_auth.sh`)
- 3단계: 부하/성능 테스트 (K6, 향후)
  - K6 스크립트를 `scripts/k6/` 아래에 두고,
  - 주요 시나리오(로그인 + 비디오 조회 + 진도 저장 등)를 기준으로 부하 테스트를 구성한다.
  - CI/CD 파이프라인에서 주기적으로 또는 수동 트리거로 실행하는 것을 목표로 한다.

---

## 8. LLM 협업 규칙 (나와 일하는 법)

> 기존 `README_for_assistant.md` + GEMINI 템플릿 관련 내용 정리.

### 8.1 질문/요청 방식

1) 요청 : 기존 작업 진행, 신규 작업 추가 및 진행, 작업 관련 질문
2) 대상 : 작업 환경(aws, docker, linux 등등), 작업 코드(database, backend, frontend, web&app 등등)
3) 방식 : **AMK_API_MASTER.md의 섹션 / 계층 / 파일** 바탕으로 작업 관련 사항 작성 ex) “Phase 3-5 `/videos/{id}/progress`에 대한 Rust 핸들러/서비스/테스트를 구현해줘”
4) 결과 : 해당 작업의 기댓값 작성 ex) “Phase 3-5 `/videos/{id}/progress`에 대한 Rust 핸들러/서비스/테스트를 구현해서 오류 없이 잘 작동 할 수 있도록 만들어줘”

### 8.2 LLM 응답 기대 형식

1) 질문 단계  
   - LLM은 요청 내용을 한 줄로 요약해서 “지금 어떤 작업을 하려는지”를 먼저 정리한다.  
   - 문서(AMK_API_MASTER.md)와 코드/요청이 명백히 충돌하거나, 선택지가 크게 갈리는 지점이 있을 때만 짧게 질의한다.  
   - 그 외에는 필요한 가정을 명시하고 바로 작업을 진행한다.

2) 진행 기준  
   - 요청한 작업에 대해, 내가 제시한 **AMK_API_MASTER.md의 섹션 / 계층 / 파일**을 최우선으로 참조하여 작업을 진행한다.  
   - 문서와 현재 코드가 다를 경우, 문서를 “정답”으로 보고 코드를 문서에 맞춘다(예외가 필요하면 명시).

3) 답변 구조  
   - 3-1) 요약: 이번 답변에서 무엇을 했는지 한 줄 또는 짧은 단락으로 정리  
   - 3-2) 세부내용: 파일/함수/쿼리 단위로 구체적인 변경 내용 제시(필요 시 코드블록)  
   - 3-3) 결과 및 효과: 변경 후 어떤 시나리오가 가능해졌는지, 어떤 문제가 해결되었는지 설명  
   - 3-4) 우려점 및 개선 사항(있다면): 성능/보안/확장성/추가 리팩터링 포인트 등  
   - 3-5) 다음 작업 추천: 자연스럽게 이어질 Phase/태스크 한두 개 제안  
   - *간단한 Q&A/개념 설명만 필요한 경우에는 3-1~3-2 중심으로 답변하고, 나머지는 필요 시에만 포함한다.*

4) 마무리 / 문서 반영  
   - 작업이 완료되면, LLM은 **AMK_API_MASTER.md에서 수정이 필요한 위치와 변경 내용(체크박스, 메모 등)**을 제안한다.  
   - 실제 파일 수정은 사용자가 수행하되, LLM은 복붙 가능한 형태로 변경안을 제공한다.

### 8.3 LLM_PATCH_TEMPLATE 연동

- 실제 코드 패치는 `LLM_PATCH_TEMPLATE.md` 형식을 따른다.
- 기본 구조:
  - ROLE / OBJECTIVE / CONTEXT / CONTRACT / PATCH RULES / ACCEPTANCE / FILE PATCHES / cURL SMOKE
- 요청 시:
  - AMK_API_MASTER.md의 **해당 섹션/Phase/엔드포인트**를 CONTRACT·CONTEXT에 명시한다.
  - 예) Phase 3-5 `/videos/{id}/progress` 스펙을 기준으로 패치 요청.
- 응답/패치 시:
  - FILE PATCHES에 나오는 각 `// FILE: ...` 블록은 **파일 전체 교체본**이다(부분 패치 금지).
  - 네이밍/enum/스키마는 AMK_API_MASTER.md의 3.2(네이밍 규칙), 4.x(데이터 모델)를 우선적으로 따른다.

---

## 9. Open Questions & 설계 TODO

> 기존 `AMK_PROJECT_JOURNAL.md`의 Open Questions + Engineering Guide의 “다음 단계 로드맵”에서 정책 수준만 정리.

### 9.1 RBAC / 관리자 권한

- 임시 가드(모든 요청 허용)를 실제 RBAC로 교체해야 함.
- 롤 후보:
  - HYMN / admin / manager
- TODO:
  - 각 롤별 허용 엔드포인트/액션 정의
  - RBAC 미스매치 시 403 정책 정리

### 9.2 Admin action log actor 연결

- `ADMIN_USERS_LOG` 및 비디오/스터디/레슨 admin 로그에:
  - **actor user id**를 전 경로에서 일관되게 채워야 함.
- TODO:
  - 인증 추출기 → handler/service/repo까지 actor id 전달 체계 확립

### 9.3 페이징 고도화 (Keyset vs Page)

- 현재 표준은 page/size 기반.
- 비디오/학습 문제와 같이 데이터가 커질 도메인에서는 **Keyset pagination** 검토 필요.
- TODO:
  - 어떤 리스트에 keyset을 우선 적용할지 정의
  - 기존 API와의 호환성 (기존 page/size와 병행할지 여부)

### 9.4 테스트 전략

- E2E/K6 부하 테스트:
  - 목표 RPS, 허용 응답시간, peak 시나리오 정의 필요
- TODO:
  - 대표 시나리오 정리 (회원가입+로그인+비디오 시청+진도 저장 등)
  - k6 스크립트 기본 골격 설계

### 9.5 보안/운영 (후순위 계획)

- 관리자 MFA 도입(특히 HYMN/admin 계정)
- 세션/리프레시 토큰 정책 강화(관리자 TTL/동시 세션 수 제한/재사용 탐지)
- 접근 제어: 관리자 IP allowlist, step-up MFA 등

---

## 10. 변경 이력 (요약)

- **2025-11-18**
  - `AMK_Feature_Roadmap.md`, `AMK_PROJECT_JOURNAL.md`, `AMK_ENGINEERING_GUIDE.md`, `AMK_API_OVERVIEW_FULL.md`, `README_for_assistant.md`의 핵심 내용을 통합.
  - 이 문서(`AMK_API_MASTER.md`)를 프로젝트의 단일 기준 문서로 지정.
- 이후 변경 사항은 커밋 메시지 `docs: update AMK_API_MASTER <요약>` 형식으로 관리하고, 필요 시 이 섹션에 중요한 방향 전환만 추가한다.
