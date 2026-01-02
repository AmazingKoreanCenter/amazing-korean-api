# LLM_PATCHS_TEMPLATE.md (MCP 버전)

# PATCH REQUEST — <모듈/단계 이름> (예: Phase 3-5 Video Progress)

**ROLE**:
- 당신은 **MCP(Model Context Protocol)** 도구를 사용할 수 있는 **Amazing Korean API 전담 AI 에이전트**입니다.
- **AGENTS.md**에 정의된 "Codex Agent"의 규칙(Full File Replacement, SSOT 준수 등)을 엄격히 따릅니다.

**OBJECTIVE**:
- <작업 목표 요약>
- 예: "`docs/AMK_API_MASTER.md`의 Phase 3-5 `/videos/{id}/progress` 엔드포인트를 구현하고, 관련 DB 마이그레이션을 작성하시오."

**MCP ACTIONS (필수 수행)**:
작업을 시작하기 전에 다음 **도구(Tools)**를 사용하여 컨텍스트를 직접 확보하십시오.
1.  **Read Specs**:
    - `docs/AGENTS.md`: 작업 규칙 및 금지 사항 확인.
    - `docs/AMK_API_MASTER.md`: 해당 Phase의 엔드포인트 스펙(HTTP, 검증, 에러, 로그 정책) 확인. (SSOT)
    - `amk_schema_patched.sql`: 관련 테이블 스키마 및 제약조건 확인.
2.  **Read Code**:
    - 수정하거나 참조할 기존 파일들(예: `src/api/video/...`)의 내용을 읽으십시오.
    - 기존의 `handler`, `service`, `repo` 패턴과 네이밍 컨벤션을 파악하여 일관성을 유지하십시오.

**IMPLEMENTATION STEPS**:
1.  **Analyze**: 읽어들인 파일들을 바탕으로 구현할 스펙(HTTP Method, Path, DTO, DB Action)을 분석했음을 짧게 언급하십시오.
2.  **Code**: **PATCH RULES**에 따라 코드를 작성하십시오.

**PATCH RULES (AGENTS.md 요약)**:
1.  **Full File Replacement**: 수정되는 파일은 반드시 **처음부터 끝까지 전체 코드(Full Content)**를 출력해야 합니다. (`// ... existing code` 등 생략 금지)
2.  **Compile-Ready**: 출력된 코드는 복사-붙여넣기 후 즉시 `cargo check`를 통과해야 합니다. (unused import 제거)
3.  **Schema & Migration**: DB 변경이 필요하면 `amk_schema_patched.sql`을 기준으로 새 마이그레이션 파일(`YYYYMMDDHHMMSS_name.sql`)을 생성하십시오.
4.  **Strict Mode**: `cargo clippy -- -D warnings` 기준을 준수하십시오.

**OUTPUT FORMAT**:

// FILE: src/api/.../filename.rs
<FILE CONTENT START>
... (전체 코드) ...
<FILE CONTENT END>

// FILE: migrations/2025..._name.sql
... (마이그레이션 쿼리, 필요한 경우) ...

// FILE: docs/AMK_API_MASTER.md
... (스펙 변경이나 완료 체크가 필요한 경우, 해당 섹션 업데이트 제안) ...

# cURL SMOKE
```bash
# 구현한 기능을 검증할 수 있는 cURL 명령어
```

## 🔄 AMK API Development Standard Operating Procedure (SOP)

모든 Phase 작업 시 아래 5단계를 엄격하게 준수한다.

### Step 1: Spec & Context Analysis (목표 및 스펙 확인)
- **Source:** `docs/AMK_API_MASTER.md`의 해당 Phase 테이블 참조.
- **Pre-check:** DB 스키마 및 데이터 상태 사전 검증 (SQL 조회 등).
- **Goal:** 엔드포인트(URL, Method), 핵심 로직(Validation, Auth), 트랜잭션 필요 여부 파악.

### Step 2: Generate Prompt Specification File (프롬프트 명세서 생성)
- **Action:** 완결된 하나의 마크다운 파일(`.md`) 생성.
- **Naming Convention:** `[Phase]-[Num].[Method]_[FeatureName].md` (예: `4-4.GET_study_task_status.md`).
- **Required Sections (Strict):**
  1. **ROLE & OBJECTIVE:** 역할 및 구현 목표.
  2. **CONTEXT (SSOT):** 참조 문서 및 비즈니스 로직(Validation, Aggregation 등).
  3. **CONTRACT:** Request/Response DTO 구조(JSON) 명세.
  4. **IMPLEMENTATION STEPS:** 작업 순서 명시 (DTO → Repo → Service → Handler → Router → Docs).
  5. **FILE PATCHES:** 수정 대상 파일 목록.
  6. **cURL SMOKE TEST:** 구현 즉시 실행 가능한 테스트 명령어.

### Step 3: Trigger Execution (실행 명령 전달)
- **Action:** 채팅창에 중복 설명 없이, 파일을 참조(`@`)하여 실행만 지시.
- **Prompt Format:**
  > "Please implement the feature described in @[FILENAME]. Follow the implementation steps, business logic, and file structure defined in the specification strictly."

### Step 4: Verification & Troubleshooting (검증 및 트러블슈팅)
- **Action:** 작성된 코드를 `cURL`로 테스트 수행.
- **Loop:** 컴파일 에러나 런타임 에러(500, 4xx) 발생 시 원인 분석 및 수정 반복.

### Step 5: Retrospective & Issue Logging (회고 및 이슈 정리)
- **Action:** 기능 구현 완료 후, 해당 Phase에서 발생한 주요 이슈와 해결책 정리.
- **Format:** Markdown 형식으로 에러 메시지, 원인, 해결 방법을 기록.
- **Goal:** 동일한 실수 반복 방지 및 컨텍스트 강화.