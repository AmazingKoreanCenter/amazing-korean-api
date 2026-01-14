# LLM_PATCHS_TEMPLATE_FRONTEND.md (MCP 버전) 

# PATCH REQUEST — FRONTEND <모듈/단계 이름>

**ROLE**:
- 당신은 **Amazing Korean API의 프론트엔드 전담 AI 에이전트**입니다.
- **Tech Stack**: React (Vite), TypeScript, Tailwind CSS, Shadcn/ui, TanStack Query, Zustand, React Hook Form, Zod.
- **AGENTS.md** 및 **AMK_API_MASTER.md (Section 6)** 의 프론트엔드 규칙을 엄격히 준수합니다.

**OBJECTIVE**:
- <작업 목표 요약>
- 예: "**Vimeo** 영상 목록을 조회하는 페이지(`/videos`)를 구현하고, 홈 화면에 진입 버튼을 만드시오."

**MCP ACTIONS (필수 수행)**:
작업을 시작하기 전에 다음 **도구(Tools)**를 사용하여 컨텍스트를 직접 확보하십시오.
1.  **Read Category Types (`types.ts`)**: 작업 대상 카테고리 폴더 내의 `types.ts` 파일을 읽으십시오.
    - *Rule*: `types.ts`는 이미 백엔드 스펙과 검증이 완료된 **절대 기준(ReadOnly)**입니다. **절대 수정하거나 새로 생성하지 말고**, 정의된 타입과 스키마를 그대로 Import 하여 사용하십시오.
2.  **Focus on Category**: 현재 작업 지시가 내려진 **해당 카테고리 폴더 내에서만** 작업하십시오.
    - *Rule*: 다른 카테고리 폴더를 넘나들며 복잡하게 엮지 말고, 현재 모듈의 구현에만 집중하십시오.
3.  **Check Dependencies**: UI 구현에 필요한 Shadcn/Radix 컴포넌트가 설치되어 있는지 확인하십시오.

**IMPLEMENTATION STEPS (Strict 5-Step Process)**:
1.  **Step 1. Read & Import Types (`types.ts`)**:
    - **중요**: `src/category/*/types.ts` 파일의 코드를 읽고, 구현할 페이지/컴포넌트에서 필요한 타입(DTO)을 Import 하십시오.
    - **금지**: `types.ts` 내용을 수정하거나, 중복된 타입을 새로 정의하지 마십시오.
2.  **Step 2. API & Hooks (`api.ts` -> `hooks/`)**:
    - **API**: `src/api/client.ts`의 Interceptor를 신뢰하십시오. 함수 인자로 `accessToken`을 불필요하게 넘기지 마십시오.
    - **Hooks**: `useMutation`/`useQuery` 작성 시 에러 핸들링(`onError`)을 반드시 포함하십시오.
3.  **Step 3. UI Component & Page**:
    - **Routing**: 내부 페이지 이동 시 절대 `<a>` 태그를 쓰지 말고, `react-router-dom`의 **`<Link>`** 컴포넌트를 사용하십시오.
    - **Feedback**: 에러 발생 시 `toast` 등을 통해 사용자에게 피드백을 주십시오. (Silent Failure 금지)
4.  **Step 4. Navigation (Entry Point)**:
    - `src/app/routes.tsx`에 경로를 등록하십시오.
    - **HomePage** 또는 네비게이션 바에 해당 페이지로 이동하는 **버튼/링크**를 반드시 추가하십시오.
5.  **Step 5. Dependency Clean-up (Refactoring Check)**:
    - 작업 중 임포트 경로가 변경되거나 사용하지 않는 변수가 생겼다면 깔끔하게 정리하십시오.

**PATCH RULES (Iron Rules)**:
1.  **🚫 No Type Hallucinations**: `types.ts`에 정의된 필드명만 사용하십시오. 화면 구현을 위해 임의로 변수(`is_marketing_agreed` 등)를 창조하지 마십시오.
2.  **🔗 DTO 1:1 Mapping**: 프론트엔드 Form Name은 `types.ts`의 키 값(Snake Case)과 정확히 일치해야 합니다.
3.  **🚦 No Silent Failures**: 4xx/5xx 에러 발생 시 `return;`으로 끝내지 말고, 반드시 `toast.error()` 등으로 사용자에게 이유를 알려주십시오.
4.  **⚡ Performance Routing**: `href="..."` 대신 `to="..."` (<Link>)를 사용하여 App State 초기화를 막으십시오.
5.  **📦 Dependency Check**: Shadcn 컴포넌트(`Switch`, `Select` 등) 사용 시, 설치 명령어를 최상단에 명시하십시오.
6.  **Full File Replacement**: 수정되는 파일은 `// ... existing code` 없이 **전체 코드**를 출력하십시오.

**PREREQUISITES (Dependencies)**:

```bash
npx shadcn@latest add ...
```

// FILE: src/category/.../video_api.ts
<FILE CONTENT START>
...
<FILE CONTENT END>
... (나머지 파일들)