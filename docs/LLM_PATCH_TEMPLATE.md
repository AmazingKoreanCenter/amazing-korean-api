# LLM_PATCH_TEMPLATE.md (공통)

# PATCH — <모듈/단계 이름> (예: Admin Videos B2 PUT /admin/videos/{id})

ROLE: Senior Rust Backend (Axum 0.8, SQLx 0.7+, utoipa 5, Rust stable 1.89)를 돕는 **AI 코딩 에이전트**  
(예: OpenAI Codex, Gemini 등 어떤 LLM이든 사용 가능)

OBJECTIVE:
- 무엇을 왜 바꾸는지 1~3줄로 요약.
- 예: "Phase 3-5 `/videos/{id}/progress`를 AMK_API_MASTER.md 스펙에 맞춰 구현/수정."

CONTEXT (프로젝트 규칙 요약):
- 스펙/엔드포인트:
  - AMK_API_MASTER.md의 **해당 Phase/엔드포인트/섹션**을 기준으로 한다.
  - **DB 스키마는 `amk_schema_patched.sql`과 AMK_API_MASTER.md 3.2(네이밍 규칙), 4.x(데이터 모델)를 기준으로 한다.**
  - 문서와 기존 코드/마이그레이션이 다르면, **문서 + `amk_schema_patched.sql`**을 정답으로 보고 코드를 맞춘다.
- 라우팅/상태:
  - `Router<AppState>`, `.with_state(state)`.
  - `FromRequestParts` 기반 Claims 추출(`Claims.sub` = user_id).
- DB:
  - `sqlx::query()` 위주, DTO-컬럼 1:1 매핑.
  - 에러는 AppError로 매핑.
  - **시간 컬럼은 기본적으로 `TIMESTAMPTZ (UTC)` 사용, CHECK/UNIQUE/FK 제약은 `amk_schema_patched.sql` 기준.**
  - **로그·진행도·통계는 *_LOG, *_STATUS, *_DAILY 테이블을 사용하고, 각 테이블의 UNIQUE/CHECK 규칙을 지킨다.**
  - **인증/세션 관련 작업은 `LOGIN`, `LOGIN_LOG`, `REDIS_SESSION`, `REDIS_REFRESH`, `REDIS_USER_SESSIONS` 및 관련 enum 규칙을 따른다.**
- Swagger:
  - tag 순서는 **health → auth → user → videos → admin** (수동 정렬 유지).
- 코딩 가드:
  - `cargo fmt` / `cargo clippy -D warnings` / `cargo check` 모두 통과해야 한다.
- 테스트:
  - Swagger `/docs` Try it out 또는 curl 스모크로 대표 시나리오 검증.

CONTRACT:
- [Phase] AMK_API_MASTER.md 기준 Phase/번호/표시
  - 예: Phase 3-5 비디오 진행도 `/videos/{id}/progress`
- [HTTP] 메서드/경로/쿼리/바디/응답/상태코드 예시
- [검증]
  - enum 값 · 범위 · 필수/옵션 규칙
  - **관련 CHECK/UNIQUE/FK 제약 (예: 진행도 0~100, UNIQUE (video_id, user_id), NOT NULL 규칙 등)**
- [DB]
  - 사용 테이블/뷰/함수 요약
  - 예: `USERS`, `USERS_SETTING`, `USERS_LOG`, `LOGIN`, `LOGIN_LOG`,
    `VIDEO`, `VIDEO_LOG`, `VIDEO_STAT_DAILY`, `STUDY`, `STUDY_TASK`, `STUDY_TASK_STATUS`,
    `LESSON`, `LESSON_ITEM`, `LESSON_PROGRESS`, `LIVE`, `LIVE_ZOOM` 등
  - 필요 시 DB 함수/뷰 (예: `api_upsert_video_progress(...)`)도 함께 명시
- [보안]
  - 인증 필요 여부, 롤(HYMN/admin/manager/learner) 조건(있다면)
  - 세션/리프레시 토큰 처리 규칙(로그인/로그아웃/회전/실패시 동작) 요약
- **[마이그레이션]**
  - 스키마 변경 여부 (테이블/컬럼/제약 추가/수정/삭제 등)
  - 어떤 내용을 새로운 마이그레이션 파일로 추가할지 한 줄 요약

PATCH RULES:
- 아래 FILE PATCHES에 **명시된 파일만** 수정 대상이다.
- 각 `// FILE: ...` 블록은 **파일 전체 교체본**이다. 부분 패치 금지.
- 컴파일 경고 0개(`-D warnings`)가 되도록 작성한다.
- 필요한 경우에만 새 타입/모듈/마이그레이션 파일을 추가하며, 추가 파일도 FILE PATCHES에 포함한다.
- **이미 적용된 마이그레이션 파일은 절대 수정/삭제하지 않고, 항상 새 마이그레이션 파일을 추가한다.**
  - 예: `migrations/20251124010101__add_study_task_status.sql`
- **스펙/스키마 변경이 포함된 작업이라면, docs/AMK_API_MASTER.md와 `amk_schema_patched.sql`의 해당 부분도 FILE PATCHES에 함께 포함한다.**
- 네이밍/enum/스키마는 AMK_API_MASTER.md 3.2, 4.x 섹션을 따른다.

ACCEPTANCE:
- `cargo fmt -- --check` ✔
- `cargo clippy -- -D warnings` ✔
- `cargo check` ✔
- curl 스모크 200/204 ✔ (예시는 맨 아래)
- 필요한 경우, 마이그레이션(`sqlx migrate run`) 및 기본 스모크 스크립트(`scripts/smoke_*.sh`)도 통과.

FILE PATCHES START
// FILE: src/api/____/____.rs
<여기에 전체 교체본>

// FILE: src/api/____/____.rs
<여기에 전체 교체본>

// FILE: migrations/20251124_____<name>.sql
<필요 시, 마이그레이션 전체 교체본>

// FILE: docs/AMK_API_MASTER.md
<스펙 변경이 들어갈 경우, 해당 섹션 전체 교체본>

// FILE: amk_schema_patched.sql
<DB 스키마 문서 변경이 있을 경우, 해당 부분 또는 전체 교체본>
FILE PATCHES END

# cURL SMOKE
```bash
# 예시 curl 시나리오들
# 1) 대표 성공 케이스
# 2) 대표 에러 케이스(401/403/400 등)
```