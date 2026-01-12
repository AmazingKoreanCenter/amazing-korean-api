# LLM_PATCHS_TEMPLATE_FRONTEND.md (MCP 버전)

# PATCH REQUEST — FRONTEND <모듈/단계 이름> (예: Phase 1-1 Login UI)

**ROLE**:
- 당신은 **Amazing Korean API의 프론트엔드 전담 AI 에이전트**입니다.
- **Tech Stack**: React (Vite), TypeScript, Tailwind CSS, Shadcn/ui, TanStack Query, Zustand, React Hook Form, Zod.
- **AGENTS.md** 및 **AMK_API_MASTER.md (Section 6)** 의 프론트엔드 규칙을 엄격히 준수합니다.

**OBJECTIVE**:
- <작업 목표 요약>
- 예: "`docs/AMK_API_MASTER.md`의 Phase 1 로그인 화면을 구현하고, `useAuth` 훅과 연동하시오."

**MCP ACTIONS (필수 수행)**:
작업을 시작하기 전에 다음 **도구(Tools)**를 사용하여 컨텍스트를 직접 확보하십시오.
1.  **Read Specs**:
    - `docs/AMK_API_MASTER.md`:
        - **Section 5 (Roadmap)**: API 엔드포인트, URL, Request/Response 필드명(snake_case) 확인.
        - **Section 6 (Frontend)**: 상태 관리(6.4), UI/Tailwind 규칙(6.5) 확인.
2.  **Read Code**:
    - `src/api/client.ts`: 공통 API 클라이언트 설정 확인.
    - `src/components/ui/...`: 사용할 Shadcn 컴포넌트(Button, Input, Form 등) 존재 여부 확인.
    - `tailwind.config.js`: 색상 변수(primary, destructive 등) 확인.

**IMPLEMENTATION STEPS (Frontend Flow)**:
1.  **Types (DTO)**: 백엔드 API 명세에 맞춰 Zod 스키마 및 TypeScript 타입을 정의하십시오. (**중요: DTO 필드명은 백엔드와 동일하게 `snake_case` 유지**)
2.  **API Client**: `src/api/` 내 도메인별 함수를 구현하십시오. (`client.ts` 활용)
3.  **Hooks**: React Query(`useQuery`, `useMutation`) 또는 Zustand를 사용하여 비즈니스 로직을 훅으로 분리하십시오.
4.  **UI Component**: `src/components/ui`의 Shadcn 컴포넌트를 조립하여 화면을 구성하십시오. (**Raw HTML/CSS 지양, 컴포넌트 재사용**)
5.  **Page/Route**: 최종 페이지를 라우터 설정에 연결하십시오.

**PATCH RULES (Strict Frontend Guidelines)**:
1.  **Full File Replacement**: 수정되는 파일은 반드시 **처음부터 끝까지 전체 코드**를 출력해야 합니다. (`// ... existing code` 생략 금지)
2.  **Type Safety**: `any` 타입 사용을 금지합니다. 인터페이스와 Zod를 통해 엄격하게 타이핑하십시오.
3.  **Naming Convention**:
    - **Variables/Functions**: `camelCase` (예: `isLoading`, `handleSubmit`)
    - **API DTO Fields**: **`snake_case`** (백엔드 DB 컬럼명과 1:1 일치, 프론트에서 임의 변환 금지)
    - **Files**: `snake_case` 또는 `kebab-case` (프로젝트 컨벤션 통일)
4.  **Shadcn First**: 버튼, 인풋, 카드 등은 반드시 `src/components/ui/` 내부의 컴포넌트를 import하여 사용하십시오. 없는 경우 설치 요청을 하십시오.

**OUTPUT FORMAT**:

// FILE: src/api/.../filename.ts
<FILE CONTENT START>
... (전체 코드) ...
<FILE CONTENT END>

// FILE: src/category/.../filename.tsx
<FILE CONTENT START>
... (전체 코드) ...
<FILE CONTENT END>

// FILE: docs/AMK_API_MASTER.md
... (구현 완료 체크 또는 스펙 변경 제안) ...

# VERIFICATION (Smoke Check)
1. **Type Check**: `npm run typecheck` (tsc -b)
2. **Lint**: `npm run lint`
3. **Browser**: `http://localhost:5173` 접속 후 [기능명] 동작 확인.

---

## 🔄 AMK Frontend Development SOP

프론트엔드 작업 시 아래 5단계를 따른다.

### Step 1: UI & Data Analysis (화면 및 데이터 분석)
- **Source:** `AMK_API_MASTER.md` Section 5 & 6.
- **Check:** 필요한 Shadcn 컴포넌트가 `src/components/ui`에 있는지 확인.
- **Goal:** 어떤 API를 호출하며, 어떤 상태(Loading, Error, Success)를 UI에 표현할지 정의.

### Step 2: Generate Prompt Specification File (프롬프트 명세서 생성)
- **Action:** 완결된 하나의 마크다운 파일(`.md`) 생성.
- **Naming Convention:** `F-[Phase]-[Num]_[FeatureName].md` (예: `F-1-1.login_screen.md`).
- **Required Sections:**
  1. **ROLE & OBJECTIVE**: 구현 목표.
  2. **UI SPEC**: 사용할 Shadcn 컴포넌트 목록, 레이아웃 구조, 반응형 전략.
  3. **DATA SPEC**: API 요청/응답 DTO (snake_case 필수) 및 Zod Validation 규칙.
  4. **FILE PATCHES**: 생성/수정할 파일 목록.

### Step 3: Trigger Execution (실행 명령 전달)
- **Action:** 채팅창에 파일 참조(`@`)와 함께 실행 지시.
- **Prompt Format:**
  > "Please implement the frontend feature described in @[FILENAME], strictly following the 'Shadcn First' and 'Snake_case DTO' rules."

### Step 4: Verification (검증)
- **Action:**
    1. 터미널: `npm run typecheck` 실행 (타입 에러 0개).
    2. 브라우저: 실제 클릭 및 데이터 연동 확인.
- **Troubleshooting:** 타입 에러 발생 시, `ts-ignore`나 `any`를 쓰지 말고 타입을 올바르게 수정할 것.

### Step 5: Retrospective (회고)
- **Action:** 프론트엔드 특화 이슈(CSS 깨짐, 훅 무한 루프, API 연동 오류 등) 기록.