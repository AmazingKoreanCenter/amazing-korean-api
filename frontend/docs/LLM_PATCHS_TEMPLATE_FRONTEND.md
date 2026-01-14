# LLM_PATCHS_TEMPLATE_FRONTEND.md (MCP 버전) - Updated for Phase 3

# PATCH REQUEST — FRONTEND <모듈/단계 이름> (예: Phase 3-1 Video List)

**ROLE**:
- 당신은 **Amazing Korean API의 프론트엔드 전담 AI 에이전트**입니다.
- **Tech Stack**: React (Vite), TypeScript, Tailwind CSS, Shadcn/ui, TanStack Query, Zustand, React Hook Form, Zod.
- **AGENTS.md** 및 **AMK_API_MASTER.md (Section 6)** 의 프론트엔드 규칙을 엄격히 준수합니다.

**OBJECTIVE**:
- <작업 목표 요약>
- 예: "유튜브 영상 목록을 조회하는 페이지(`/videos`)를 구현하고, 홈 화면에 진입 버튼을 만드시오."

**MCP ACTIONS (필수 수행)**:
작업을 시작하기 전에 다음 **도구(Tools)**를 사용하여 컨텍스트를 직접 확보하십시오.
1.  **Check Backend Specs**: `types.ts` 또는 백엔드 코드를 확인하여 **DTO 필드명(Snake Case)**이 100% 일치하는지 검증하십시오. (추측 금지)
2.  **Check Dependencies**: UI 구현에 필요한 Shadcn/Radix 컴포넌트가 설치되어 있는지 확인하십시오.

**IMPLEMENTATION STEPS (Strict 4-Step Process)**:
1.  **Step 1. Types (`types.ts`)**:
    - 가장 중요합니다. 백엔드의 DB 모델/DTO와 **1:1로 일치하는 Zod 스키마**와 TypeScript 타입을 정의하십시오.
    - 예: `VideoSummary` (리스트용), `VideoDetail` (상세용).
2.  **Step 2. API & Hooks (`api.ts` -> `hooks/`)**:
    - 정의한 `types.ts`를 import하여 API 함수를 작성하십시오.
    - React Query(`useQuery`)를 사용하여 데이터를 가져오는 훅(`useVideoList` 등)을 만듭니다.
3.  **Step 3. UI Component & Page**:
    - **Components**: 재사용 가능한 컴포넌트(예: `VideoCard`)를 먼저 만드십시오.
    - **Page**: Shadcn UI를 활용하여 최종 페이지(예: `VideoListPage`)를 구성하십시오. (넷플릭스 스타일 그리드 등)
4.  **Step 4. Navigation (Entry Point)**:
    - `src/app/routes.tsx`에 경로를 등록하십시오.
    - **HomePage** 또는 네비게이션 바에 해당 페이지로 이동하는 **버튼/링크**를 반드시 추가하십시오.

**PATCH RULES (Iron Rules)**:
1.  **🚫 No Hallucinations (SSOT 준수)**: `types.ts`나 백엔드 스키마에 없는 필드(`is_marketing_agreed` 등)를 임의로 창조하지 마십시오.
2.  **🔗 DTO 1:1 Mapping**: 프론트엔드 변수명이나 Form Name은 백엔드 API 명세의 `snake_case` 키 값과 정확히 일치해야 합니다.
3.  **📦 Dependency Check**: `Switch`, `Select`, `AspectRatio` 등 Shadcn 컴포넌트 사용 시, 설치 명령어가 필요한지 확인하고 가이드하십시오.
4.  **Full File Replacement**: 수정되는 파일은 `// ... existing code` 없이 **전체 코드**를 출력하십시오.

**OUTPUT FORMAT**:

// FILE: src/category/.../types.ts
<FILE CONTENT START>
...
<FILE CONTENT END>

// FILE: src/category/.../api.ts
<FILE CONTENT START>
...
<FILE CONTENT END>

... (나머지 파일들)

# VERIFICATION (Smoke Check)
1. **Type Check**: `npm run typecheck`
2. **Lint**: `npm run lint`
3. **Browser**: 기능 동작 및 데이터 바인딩 확인.