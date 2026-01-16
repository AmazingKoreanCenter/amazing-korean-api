---
title: AMK_FRONTEND_STATUS - 현재까지 프론트 작업 진행 상황
updated: 2026-01-15
owner: HYMN Co., Ltd. (Amazing Korean)
audience: frontend / lead / LLM assistant
---

## 1. 개요 & 계약 (Reference)
- **SSoT 문서**: `AMK_API_MASTER.md` (API 스펙 원본)
- **Tech Stack**: Vite, React, TypeScript, Tailwind, Shadcn/ui, React Query, Zustand

## 2. 프로젝트 설정 (Config)
- [x] **Vite Proxy (`vite.config.ts`)**
    - **API Prefix 전략**: `/api`로 시작하는 요청만 백엔드(`localhost:8080`)로 포워딩하여 UI 라우팅 충돌 방지.
    - **Rewrite**: 백엔드 전달 시 `/api` 접두어 제거 (`/api/videos` → `/videos`).

- [x] **API Client (`client.ts`)**
    - **Base URL**: `/api`로 표준화 (Vite Proxy와 연동).
    - **Interceptor**: 요청 시 JWT(Bearer) 자동 주입 및 401 발생 시 토큰 갱신 로직 탑재.

- [x] **Shadcn/ui (Design System)**
    - **Core**: `components/ui` 폴더 내 모듈형 컴포넌트 아키텍처.
    - **Installed Components**: Button, Card, Input, Form, **Badge, Skeleton, AspectRatio**, Toast.

- [x] **Routing (`routes.tsx`)**
    - **React Router v6**: `Routes` 및 `Route` 컴포넌트를 활용한 선언적 라우팅 구조.
    - **Route Guard**: `PrivateRoute` 컴포넌트로 감싸진 경로(`/user/*`, `/settings`)에 대한 인증 강제 (비로그인 시 차단).
    - **Path Structure**:
        - **Public**: Home(`/`), Auth(`/login`, `/signup`, `/find-id`, `/reset-password`), Video(`/videos`, `/videos/:videoId`), Health(`/health`).
        - **Private**: User(`/user/me`, `/user/edit`), Settings(`/settings`).

- [x] **Media Engine**
    - **Vimeo Integration**: `@vimeo/player` SDK 설치 및 Iframe 제어 환경 구성 (Native Event Handling).

## 3. 화면 & 컴포넌트 현황 (UI Checklist)
> 화면 단위 구현 및 API 연동 여부 체크 (Phase 3-2 완료 기준)

### Common (Layout)
- [x] **Navbar**: 로그인 상태(`useAuthStore`)에 따른 메뉴 분기 (로그인/비로그인)
- [x] **PrivateRoute**: 비로그인 접근 차단 및 리다이렉트 처리

### Auth (Category: auth)
- [x] `/login`: JWT 로그인 / 토큰 저장 / 에러 토스트 처리
- [x] `/signup`: React Hook Form + Zod 유효성 검사 적용 완료
- [x] `/find-id`: 아이디 찾기 UI 및 API 연동
- [x] `/reset-password`: 비밀번호 재설정 UI 및 API 연동

### Video (Category: video) - ✅ Phase 3 진행 중
- [x] `/videos` (List): 강의 목록 조회 (Grid Layout, Badge 적용)
- [x] `/videos/:videoId` (Detail):
    - **Player**: `@vimeo/player` SDK 연동 및 재생/종료 이벤트 감지
    - **Metadata**: 제목, 태그, 자막 정보 바인딩
    - **Error**: 없는 영상 접근 시 404 UI / 목록으로 돌아가기 구현
- [x] **Video Progress (GET)** `Phase 3-3`
    - 진도율 조회 API 연동 (`GET /progress`).
    - **Policy**: 숏폼 특성상 이어보기(Resume) 제외, 완료 상태(`is_completed`) 확인 위주.
- [x] **Video Progress (POST)** `Phase 3-4`
    - **Logic**: `onPause` (Debounce), `onEnded` (100% 강제) 이벤트 시 서버 저장.
    - **Data Flow**: Player Event → Mutation → Server DB Upsert → Query Invalidate.

### User (Category: user)
- [x] `/user/me` (MyPage): 내 정보 조회 (Profile Card)
- [x] `/user/edit`: 회원 정보 수정 폼 구현
- [ ] `/settings`: **(작업 중)**
    - UI: `Switch` (알림), `Select` (언어) 컴포넌트 배치 완료
    - Logic: `useMutation` 연결 대기 중 (낙관적 업데이트 필요)

### Etc
- [x] `/`: 홈 화면 (랜딩 페이지)
- [x] `/health`: 서버 상태 확인용 헬스 체크 페이지

## 4. 상태 관리 & 훅 (State & Hooks)

- **Client State (Global)**
    - **Store**: `useAuthStore` (Zustand)
    - **Role**:
        - JWT Access Token 관리 (메모리 저장)
        - 사용자 기본 정보(`user_id`, `email`, `role`) 캐싱
        - 로그인(`login`) / 로그아웃(`logout`) 액션 처리

- **Server State (TanStack Query)**
    - **Query Key Convention**: `[Domain, Scope, ID/Params]` 계층 구조 준수
    - **Active Keys**:
        - `['user', 'me']`: 내 정보 조회 (Profile)
        - `['videos', 'list', { page, ... }]`: 비디오 목록 (Pagination 포함)
        - `['videos', 'detail', videoId]`: 비디오 상세 정보 (Phase 3-2 완료)
        - `['videos', 'progress', videoId]`: 학습 진도율 (Phase 3-3 예정)

- **Custom Hooks (Business Logic)**
    - **Auth**: `useLogin`, `useSignup`, `useLogout`
    - **Video**: `useVideoList`, `useVideoDetail`, `useVideoProgress` (예정)
    - **UI**: `useToast` (Shadcn/ui Notification)

## 5. Backlog & UX Plan (Upgrade)
> `...upgrade.md` 파일들의 주요 로드맵 및 개선 계획 통합

### 🎬 Video Player (Priority)
- [ ] **Resume Capability**: `timeupdate` 이벤트를 활용한 10초 단위 시청 기록 저장 & 이어보기 구현.
- [ ] **Auto-Play Next**: 영상 종료(`ended`) 시 5초 카운트다운 후 다음 강의로 자동 이동.
- [ ] **Custom UI**: Vimeo 기본 컨트롤러를 숨기고, Shadcn 기반의 오버레이(재생/정지, 볼륨) 적용.
- [ ] **Prefetching**: 목록 페이지에서 마우스 호버 시 상세 데이터 미리 로드하여 진입 속도 단축.

### 🛠 Admin & Automation
- [ ] **Vimeo Sync**: URL 입력 시 Vimeo API를 호출하여 제목/썸네일/재생시간 자동 동기화.
- [ ] **Status Webhook**: Vimeo에서 영상 삭제/비공개 시 DB 상태(`video_state`) 자동 업데이트.

### 👤 User & Settings
- [ ] **Theme Sync**: 다크/라이트 모드 설정 변경 시 서버 프로필(`theme_preference`)에 저장.
- [ ] **Profile Image**: 이미지 업로드 시 브라우저 내 자르기(Crop) 및 리사이징 기능 구현.

## 6. Frontend Dev Log (Integrated Key Learnings)

### 📝 Output Formatting Rules (답변 형식 준수)
- **Code Block Escape (Quadruple Backticks)**:
    - 답변에 마크다운 파일 내용이나 ` ``` `(3중 백틱)이 포함된 코드를 작성할 때는, **반드시 4중 백틱(```` ` ````)으로 감싸서** 렌더링이 깨지지 않도록 해야 함.

> Phase 2(Auth) ~ Phase 3(Video) 전체 진행 과정의 핵심 교훈 통합

### 📡 Network & Infra
- **Vite Proxy & Route Collision (Critical)**
    - **이슈**: 프론트 라우트(`/videos`)와 백엔드 API 엔드포인트가 겹쳐서 HTML이나 JSON이 잘못 리턴되는 현상.
    - **해결**: API 요청은 무조건 `/api` 접두어를 붙이고, `vite.config.ts`에서 Proxy로 분기 처리 (`rewrite` 필수).
- **Axios Interceptor**
    - **교훈**: API 함수마다 `token`을 인자로 넘기지 말 것. `client.ts`의 Interceptor에서 `Authorization` 헤더를 일괄 주입하고, 401 발생 시 토큰 갱신(Refresh) 로직을 중앙에서 처리해야 함.

### 🛡️ Data & Types
- **Type Hallucination (Data Mismatch)**
    - **이슈**: 백엔드 DB 컬럼은 `video_url_vimeo`인데, 프론트에서 익숙한 `url`이나 `src`로 추측해서 코딩하다 렌더링 실패.
    - **해결**: `types.ts`는 백엔드 DTO(`snake_case`)와 **100% 일치**해야 함. 프론트 편의를 위해 카멜케이스로 변환하지 말고 그대로 사용할 것.
- **Zod Schema Validation**
    - **교훈**: 폼 제출 시 데이터가 백엔드로 넘어가지 않는다면, 90% 확률로 `zod` 스키마의 유효성 검사 실패(타입 불일치, 필수값 누락)임. `console.log(errors)`를 습관화할 것.

### 🎬 Media & Player
- **Vimeo Security (403 Forbidden)**
    - **이슈**: 영상 ID가 맞는데 재생이 안 되는 경우.
    - **해결**: Vimeo 설정의 **"Embed domains"**에 `localhost`와 개발 서버 IP가 등록되어 있는지 확인. 라이브러리(`react-player`)보다 Native SDK(`@vimeo/player`)가 보안 이슈 디버깅에 유리함.

### 🧩 UI & Components
- **Shadcn/ui Customization**
    - **교훈**: `components/ui` 폴더 내 파일은 라이브러리가 아니라 **내 소스코드**임. 스타일 수정이 필요하면 `node_modules`를 뒤지지 말고 해당 파일을 직접 수정할 것.
- **React Hook Form Reset**
    - **이슈**: 수정 페이지(`EditProfile`) 진입 시 Input이 비어있음.
    - **해결**: `defaultValues`는 초기 렌더링에만 관여함. 비동기 데이터 로딩이 끝난 시점(`useEffect`)에 `reset(loadedData)`를 호출해야 값이 채워짐.

### 🎥 Video & Player
- **Vimeo CSP & Event Issue**
    - **이슈**: Vimeo Player 로드 시 Blob 이미지 차단(CSP) 및 Passive Event 리스너 경고 발생.
    - **결론**: 기능 동작에 영향이 없으므로 Low Priority로 분류. 추후 `vite.config` CSP 완화 고려.
- **Auto-Resume UX Cancelled**
    - **결정**: 초기 기획과 달리 영상이 짧아(10분 미만) 이어보기 기능은 오히려 UX를 해친다고 판단하여 제거함. DTO의 `last_position` 필드 의존성 삭제.

### 🔐 Auth & Security
- **401 Log is Normal**
    - **현상**: 페이지 진입 시 콘솔에 401 에러가 뜨고 직후 200이 뜸.
    - **해석**: Axios Interceptor가 만료된 토큰을 감지하고 `Refresh`를 수행하는 정상 과정임. "에러 아님".

### 🔄 Data Flow & State
- **Video Progress Persistence**
    - **성과**: Vimeo Player의 이벤트(`pause`, `ended`)와 React Query Mutation을 연동하여 실시간 진도율 저장 성공.
    - **특이사항**: "재학습" 시나리오(완료 후 다시 볼 때)에서도 `is_completed: true`는 유지되면서 `progress`는 현재 위치로 갱신되는 백엔드 로직 확인. 사용자에게 혼란을 주지 않는 적절한 동작임.