---
title: AMK_BACKEND_STATUS - 현재까지 백엔드 작업 진행 상황
updated: 2026-01-15
owner: HYMN Co., Ltd. (Amazing Korean)
audience: server / database / backend LLM assistant
---

## 1. 개요 & 계약 (Reference)
- **SSoT 문서**: `AMK_API_MASTER.md` (전체 아키텍처 및 스택 정의)
- **현재 단계**: **Phase 3 (Video) 리팩토링 완료** → **Phase 4 (Admin) 진입 직전**
- **Core Stack**: Rust (Axum 0.8), SQLx, PostgreSQL, Redis, Docker, JWT
- **Frontend Sync**: React (Vite) + Zod (`types.ts` 최신화 완료)

## 2. 환경 및 인프라 (Environment)
- **Database**: PostgreSQL (`amk-pg`), Redis (`amk-redis`)
- **Dev Tools**: Windows + WSL2 (Ubuntu), VS Code, Codex CLI
- **Schema Note**: 
  - 모든 시간 컬럼은 `TIMESTAMPTZ` (UTC)
  - `video` 테이블과 `video_tag` 테이블은 **N:M 관계** (중요: 제목/부제목은 `video_tag`에 존재)

## 3. 구현 현황 (Implementation Checklist)
> `AMK_API_MASTER.md` : **## 5. 기능 & API 로드맵** 기준 구현 완료. 현재 리팩토링 진행 중

> **Refactoring Standard (작업 원칙)**:
> 1. **Stateless Architecture**: `Service`와 `Repo`는 필드가 없는 구조체(Empty Struct)로 정의하며, 메서드 호출 시 `&AppState`(DB Pool)를 인자로 받아 처리함.
> 2. **Database Pattern**: 
>    - 복잡한 검색/필터는 `sqlx::QueryBuilder`를 사용하여 동적 쿼리 구성.
>    - Rust(`i64`)와 PG(`INT4`) 타입 불일치 시 쿼리 내 `::bigint` 캐스팅 필수 적용.
>    - `video` 테이블의 `title` 부재 등 스키마 불일치는 `JOIN` 및 `COALESCE/MAX` 집계 함수로 해결.
> 3. **Code Structure**:
>    - 도메인별 패키지 분리: `src/api/{domain}/{handler, service, repo, dto}.rs`
>    - DTO 네이밍: JSON 응답은 `snake_case` 준수, `Option<T>`를 통한 `nullable` 명시적 처리.

### Phase 1: User Domain (Refactored) [x]
- [x] `POST /users`: 회원가입 (Argon2 해싱, 트랜잭션)
- [x] `GET /users/me`: 내 정보 조회 (Auth Guard)
- [x] `PATCH /users/me`: 정보 수정
- [x] `GET /users/me/settings`: 내 설정 조회 (학습 언어, 알림 설정 등)
- [x] `POST /users/me/settings`: 내 설정 수정

### Phase 2: Auth Domain (Refactored) [x]
- [x] `POST /auth/login`: 로그인 (Access Token 발급, Refresh Token Redis 저장)
- [x] `POST /auth/logout`: 로그아웃 (Redis 토큰 삭제 + 쿠키 만료)
- [x] `POST /auth/refresh`: 토큰 갱신 (Rotation 적용)
- [x] **Middleware**: `AuthUser` Extractor 구현 (JWT 검증 + Redis 블랙리스트 체크)
- [x] `POST /auth/find-id`: 회원 아이디 찾기 (이메일 기반)
- [x] `POST /auth/reset-pw`: 회원 비밀번호 재설정 (인증 코드 검증 후 처리)

### Phase 3: Video Domain (Refactored) [x]
- [x] `GET /videos`: 목록 조회
  - **특이사항**: 동적 쿼리(`QueryBuilder`) 적용. 검색/필터/페이징/정렬 완벽 구현.
  - **로직**: `video` 테이블에 없는 제목 정보는 `video_tag` 테이블을 조인하여 `MAX()`로 가져옴.
- [x] `GET /videos/{id}`: 상세 조회
  - **로직**: 해당 비디오에 연결된 모든 태그 정보를 `JSONB` 배열로 반환.
- [x] `GET /videos/{id}/progress`: 내 진도율 조회
- [x] `POST /videos/{id}/progress`: 진도율 저장 (Upsert)
  - **로직**: `100%` 달성 시 `is_completed` 자동 처리.

### Phase 4: Study Domain (Refactored) []
> **Goal**: 학습 문제(Task) 풀이, 채점, 해설 및 로그 관리 (CBT 핵심 기능)
  - [x] `GET /studies`: 학습 문제 목록 조회
    - **Auth**: 비로그인 접근 가능 (Public).
    - **Logic**: `study_program_enum` 기준 필터링 + 페이지네이션.
    - **Refactor Note**: `QueryBuilder`를 사용해 프로그램 필터/정렬/페이징을 하나의 동적 쿼리로 통합 구현.
  - [x] `GET /studies/tasks/{id}`: 학습 문제 상세 조회 (풀이 화면)
    - **Auth**: **필수** (`AuthUser`).
    - **Logic**: `STUDY_TASK` 테이블 조회. 문제 지문 및 선택지 반환.
    - **Refactor Note**: DB의 평면적 컬럼들을 `TaskPayload` Enum(Choice/Typing/Voice)으로 매핑하여 다형성 있는 JSON 응답 구조 완성.
    - **Logging** : `study_task_log` 테이블 업데이트 (트랜잭션 필수)
  - [x] `POST /studies/tasks/{id}/answer`: 정답 제출 및 채점
    - **Auth**: **필수** (`AuthUser`).
    - **Logic**: 채점 로직 수행(Choice/Typing/Voice 테이블의 답안 비교)
    - **Validation**: 선택지 범위 오류 시 422, 형식 오류 시 400.
    - **Refactor Note**
      1) Service 계층에서 채점(Grading) 로직 수행
      2) Repo 계층에서 순차적으로 로그 업데이트 
        2-1. `upsert_log` : `study_task_log` 테이블 업데이트 함수
        2-2. `upsert_status` : `study_task_status` 테이블 업데이트 함수
    - **Logging** : `study_task_status`, `study_task_log` 테이블 업데이트 (트랜잭션 필수)
  - [x] `GET /studies/tasks/{id}/status`: 내 학습 현황 조회
    - **Auth**: **필수** (`AuthUser`).
    - **Logic**: 해당 문제에 대한 내 최신 기록(진도, 점수, 시도 횟수) 조회. 기록 없으면 빈 값(200) 반환.
    - **Refactor Note**: `study_task_log` 테이블에서 사용자의 풀이 상태(`study_task_status_try_count`, `study_task_status_is_solved`) 조회 로직 구현.
    - **Logging** : `study_task_log` 테이블 업데이트 (트랜잭션 필수)
  - [x] `GET /studies/tasks/{id}/explain`: 문제 해설 조회
    - **Logic**: `STUDY_EXPLAIN` 테이블 조회 (해설 텍스트/미디어).
    - **Access Control**: 정책에 따라 '문제 풀이 전 열람 시' **403 Forbidden** 처리 로직 고려.
    - **Refactor Note**: 해설 및 정답 텍스트 조회 쿼리 구현 완료 (`403` 정책은 추후 기획 확정 시 Service 계층에 추가 예정).
    - **Logging** : `study_task_log` 테이블 업데이트 (트랜잭션 필수)

### Phase 5: Lesson Domain (To-Do) [ ]
> **Goal**: 수업(Lesson) 목록, 상세 정보, 학습 시퀀스(Items) 및 진도율 관리
- [ ] `GET /lessons`: 수업 전체 목록 조회
  - **Logic**: `lesson_idx` 기준 정렬 및 페이지네이션 적용.
  - **Auth**: 비로그인 접근 가능.
- [ ] `GET /lessons/{id}`: 수업 상세 정보 조회
  - **Logic**: 해당 수업에 포함된 `video_tag_id` + `study_task_id` 목록 기반 구성.
- [ ] `GET /lessons/{id}/items`: 수업 학습 항목(Sequence) 조회
  - **Logic**: `lesson_item_seq` 순서대로 정렬하여 반환. (실제 풀이/재생은 Video/Study 도메인 API 사용)
  - **Access Control**: 수강권(Ticket)이 필요한 경우 권한 없으면 **403 Forbidden** 처리.
- [ ] `GET /lessons/{id}/progress`: 수업 진행률 조회
  - **Auth**: **필수** (`AuthUser`).
  - **Logic**: `LESSON_PROGRESS` 최신 값 조회. 기록이 없으면 0% 반환.
- [ ] `POST /lessons/{id}/progress`: 수업 진행률 갱신
  - **Auth**: **필수** (`AuthUser`).
  - **Logic**: 0~100 사이 값으로 업데이트 (멱등성 보장). 수강권 검증 로직 포함.

### Phase 6: Admin Domain (To-Do) [ ]
> **Goal**: 백오피스 (CMS & LMS 관리) - RBAC, 감사 로그(Audit Log), 대량 처리(Bulk)
- [ ] **Middleware**: `AdminGuard` 구현 (User Role: 'ADMIN' 검증)
- [ ] **User Management**:
  - `GET`, `POST`, `PATCH` (Single/Bulk): 사용자 조회 및 정보 수정.
  - **Logic**: 대량 처리 시 `207 Multi-Status` 및 부분 성공 지원. `ADMIN_USERS_LOG` 기록 필수.
- [ ] **Video Management**:
  - `GET`, `POST`, `PATCH` (Single/Bulk): 비디오 메타데이터 등록/수정.
  - `PATCH .../tags`: 비디오 태그 매핑(N:M) 수정.
  - **Logic**: `ADMIN_VIDEO_LOG` 기록. `stats`(통계) 기능은 추후 구현.
- [ ] **Study Management**:
  - **Basic**: 학습 세트(`Study`) 관리 (CRUD + Bulk).
  - **Tasks**: 문제(`Task`) 세부 정보 및 정답 데이터 관리 (CRUD + Bulk).
  - **Explain**: 문제 해설 및 미디어 자료 관리 (CRUD + Bulk).
  - **Status**: 학습자 문제 풀이 상태(`Status`) 강제 보정/수정.
  - **Logic**: 계층 구조(Study → Task → Explain) 무결성 검증, `ADMIN_STUDY_LOG` 기록.
- [ ] **Lesson Management**:
  - **Basic**: 수업(`Lesson`) 정보 관리 (CRUD + Bulk).
  - **Items**: 수업 구성 요소(Video/Study 연결) 및 순서(`Sequence`) 관리.
  - **Progress**: 학습자 수업 진도율(`Progress`) 강제 보정/수정.
  - **Logic**: `lesson_item_seq` 중복/순서 규칙 검증, `ADMIN_LESSON_LOG` 기록.

## 4. 핵심 아키텍처 & 패턴 (Architecture Note)
1.  **Stateless Architecture (Service Layer)**:
    - **원칙**: `Repo`, `Service` 구조체는 필드를 가지지 않는 Empty Struct로 정의.
    - **패턴**: 모든 비즈니스 로직 메서드는 `st: &AppState` (DB Pool, Redis 포함)를 첫 번째 인자로 받아 처리.
    - **장점**: Rust의 소유권/수명(Lifetime) 문제를 최소화하고, 핸들러 간 상태 공유를 단순화.

2.  **Dynamic Query & Schema Adaptation (SQLx)**:
    - **동적 쿼리**: `sqlx::QueryBuilder`를 사용하여 검색(`q`), 필터(`state`, `tag`) 등 조건부 쿼리를 안전하게 조립.
    - **N:M 관계 해소**: `Video` 테이블(물리)과 `VideoTag` 테이블(메타/컨텍스트)이 분리된 구조. 목록 조회 시 `LEFT JOIN` + `GROUP BY` + `MAX()` 집계 함수를 사용하여 대표 제목을 추출.

3.  **Data Mapping & Type Safety**:
    - **ID Casting**: PostgreSQL의 `Serial(INT4)`과 Rust의 `i64` 불일치로 인한 런타임 패닉 방지를 위해, 쿼리 내에서 `v.video_id::bigint` 캐스팅 필수 적용.
    - **Nullable Handling**: DB 스키마에 없는 필드(예: `duration`)를 DTO에 맞춰야 할 경우, 쿼리 레벨에서 `NULL::type` 또는 기본값(Default)을 반환하여 Rust 타입 안정성 보장.

4.  **Transaction Strategy**:
    - **규칙**: 트랜잭션의 시작(`begin`)과 종료(`commit/rollback`)는 **Service 계층**에서 담당.
    - **흐름**: Service가 `tx`를 생성 → Repo 메서드에 `&mut tx` 전달 → 모든 Repo 작업 성공 시 Service가 `commit`.
    - **적용**: 회원가입(User+Log), 학습 기록(Log+Status), Admin 생성(Main+Log) 등 복합 로직에 필수.

5.  **Auth & Security Pattern**:
    - **Extractor**: 핸들러 내부에서 토큰을 파싱하지 않고, `AuthUser`라는 Axum Extractor를 통해 진입 전 검증(JWT 서명 + Redis 블랙리스트) 완료.
    - **Context**: 검증된 사용자 정보(`user_id`, `role`)는 핸들러의 인자로 자동 주입됨.

## 5. Known Issues & Tech Debt
### 🚀 Feature Gaps (기능 보완 필요)
- [ ] **Auth & Security**:
  - **Email Verification**: 회원가입, ID 찾기, 비밀번호 재설정 시 **SMTP 연동** 및 인증 코드 검증 로직 구현.
  - **Admin Promotion**: 일반 회원을 관리자로 승격시키는 로직(Super Admin) 또는 CLI 툴 부재.
  - **Login Metadata**: `login_log` 테이블의 `country`, `asn`, `org` 등 채워지지 못한 컬럼 채우기 (GeoIP 서비스 또는 라이브러리 연동 필요).
- [ ] **Business Logic**:
  - **Payment & Course**: 결제 모듈(PG) 연동 및 결제 완료 시 수강권(Ticket) 자동 발급/Course 해금 로직.
  - **Log Enum Expansion**: `user_action_log_enum`에 `PROFILE_VIEW`, `PROFILE_UPDATE`, `SETTINGS_VIEW`, `SETTINGS_UPDATE` 타입 추가 필요.
- [ ] **Global Service (i18n)**:
  - **UI Language**: 사용자 설정(`user.language`)에 따라 에러 메시지 및 시스템 텍스트 다국어 반환 아키텍처 수립.

### 🛠 Tech Debt (기술적 부채 & 인프라)
- [ ] **Observability (Logging)**:
  - 현재 `println!` 매크로 사용 중 → **`tracing` 크레이트** 도입으로 구조화된 로그 수집 필요.
  - **Admin Log Viewer**: 수집된 서버 로그를 DB 또는 파일로 저장하여 Admin 페이지에서 실시간/과거 로그를 조회하는 기능 구현.
- [ ] **DB Schema Alignment**:
  - `video` 테이블에 물리적 메타데이터(duration, thumbnail) 컬럼 부재로 코드단에서 `NULL` 처리 중 (추후 업데이트 방향 제시 예정).
- [ ] **Quality Assurance**:
  - 핵심 로직(Auth, Payment)에 대한 유닛 테스트 및 통합 테스트 코드 부재.

## 6. Backend Dev Log (최근 교훈 및 의사결정)

### 📝 Output Formatting Rules (답변 형식 준수)
- **Code Block Escape (Quadruple Backticks)**:
    - 답변에 마크다운 파일 내용이나 ` ``` `(3중 백틱)이 포함된 코드를 작성할 때는, **반드시 4중 백틱(```` ` ````)으로 감싸서** 렌더링이 깨지지 않도록 해야 함.

### 🚨 Critical Rules (재발 방지 필수)
- **Axum 0.8 Path Syntax**:
    - **이슈**: Axum 0.8 버전부터 라우터 경로 파라미터 문법이 변경됨. (기존 `:id` 사용 시 패닉 발생 가능성)
    - **해결**: OpenAPI 표준과 동일하게 **`{id}`** 형식을 사용해야 함. (예: `.route("/tasks/{id}", ...)`).
- **Utoipa Macro & Extractor Conflict**:
    - **이슈**: 핸들러 함수 인자에 `AuthUser(_)`와 같이 패턴 매칭을 직접 사용하면, `#[utoipa::path]` 매크로가 이를 파싱하지 못해 컴파일 에러(`unexpected syn::Pat`) 발생.
    - **해결**: 핸들러 인자에서는 **단순 변수명(`_auth`)**을 사용하고, 필요 시 함수 내부에서 로직을 처리해야 함.
- **Unused Import Strictness**:
    - **이슈**: DTO나 Enum을 습관적으로 `use` 구문에 모두 넣으면 Rust 컴파일러가 `unused import` 경고를 발생시킴.
    - **해결**: 코드를 작성 후 실제 사용되는 타입만 `use`에 남기는 "가상 린트" 과정을 거쳐야 함.
- **Dead Code Allowance**:
    - **이슈**: DTO에는 존재하지만 비즈니스 로직에서 당장 쓰지 않는 필드(예: `audio_url`)는 경고 대상이 됨.
    - **해결**: 의도된 미사용 필드에는 `#[allow(dead_code)]`를 명시하여, "나중에 쓸 것"임을 컴파일러에게 알려야 함.

### 🛠️ Implementation Patterns & Decisions
- **Polymorphic DTO Mapping**:
    - **결정**: `StudyTask` 테이블의 평면적(Flat) 컬럼들(`choice_1`, `typing_text` 등)을 API 응답 시 `TaskPayload` Enum으로 묶어 처리.
    - **이유**: 프론트엔드에서 문제 유형(`choice`, `typing`, `voice`)에 따라 UI를 분기(`switch case`)하기 가장 최적화된 구조임.
- **Stateless Service & Repo**:
    - **결정**: 모든 Service와 Repo는 상태(State)를 가지지 않는 빈 구조체(`struct StudyService;`)로 정의하고, 메서드는 정적(`StudyService::list(...)`)으로 호출.
    - **이유**: 인스턴스 생성 비용 제거 및 `AppState` 주입의 단순화.