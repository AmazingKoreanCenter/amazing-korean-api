# AMK_FRONTEND — 프론트엔드 구조 & 규칙

> 프론트엔드 스택, 디렉터리 구조, 라우팅, 상태관리, UI/UX 규칙.
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)
> 디자인 시스템: [AMK_DESIGN_SYSTEM.md](./AMK_DESIGN_SYSTEM.md)

---

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
  - **i18n (다국어)**: **react-i18next** + **i18next** (ko/en 지원, 수동 전환 방식)
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
       - 향후 **Flutter 모바일 앱** / **Tauri 데스크탑 앱** 확장을 고려하여, 비즈니스 로직은 컴포넌트(UI) 내부에 작성하지 않는다.
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
    textbook/              # 교재 주문 (Public)
      page/
      hook/
      types.ts
      textbook_api.ts
    admin/
      # ... (동일 구조)
      textbook/            # 교재 주문 관리 (Admin)
        page/
        hook/
        types.ts

  components/            # 공용 컴포넌트 (Horizontal Slicing)
    ui/                  # ★ shadcn/ui 설치 경로 (Button, Dialog 등)
    layout/              # Header, Footer, Sidebar 등 레이아웃 조각
    sections/            # HeroSection, SectionContainer, PaginationBar 등 페이지 섹션
    shared/              # 도메인에 종속되지 않는 재사용 컴포넌트 (LoadingSpinner 등)
    page_meta.tsx        # ★ SEO: React 19 네이티브 metadata (title, canonical, OG/Twitter)

  i18n/                  # ★ 다국어(i18n) 모듈
    index.ts             # i18next 초기화, changeLanguage/getSavedLanguage 헬퍼
    locales/
      ko.json            # 한국어 번역 (기본 언어)
      en.json            # 영어 번역

  hooks/                 # 전역 Custom Hook
    use_auth.ts          # 인증 상태 관리 (Zustand + Logic)
    use_language_sync.ts # DB 언어 설정 ↔ i18n 동기화
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

#### 6.2.4 다국어(i18n) 아키텍처

> 목적: 한국어(ko)와 영어(en)를 지원하며, **사용자 수동 전환** 방식으로 동작한다. 브라우저 언어 자동 감지는 사용하지 않는다.

##### 지원 언어 & 기본값

| 코드 | 언어 | 비고 |
|------|------|------|
| `ko` | 한국어 | **기본 언어 (fallback)** |
| `en` | English | |

##### 언어 결정 우선순위

```
1. DB user_set_language (로그인 상태)
2. localStorage "language" 키
3. 기본값 "ko"
```

- **로그인 시**: `useLanguageSync` 훅이 DB의 `user_set_language`를 가져와 i18n + localStorage에 적용 (최초 1회)
- **비로그인 시**: localStorage에 저장된 언어를 유지
- **로그아웃 시**: 마지막 선택한 언어를 localStorage에서 유지

##### 번역 파일 구조

- 경로: `src/i18n/locales/{ko,en}.json`
- 네임스페이스 구조 (플랫 JSON, 도메인별 prefix):

```json
{
  "common": { "loading": "...", "save": "..." },
  "nav":    { "about": "...", "login": "..." },
  "footer": { "brandDescription": "...", "copyright": "..." },
  "auth":   { "loginTitle": "...", "signupButton": "..." },
  "user":   { "myPageTitle": "...", "settingsTitle": "..." },
  "home":   { "heroTitle": "...", "ctaStart": "..." },
  "about":  { "badge": "...", "missionTitle": "..." },
  "study":  { "listTitle": "...", "kindChoice": "..." },
  "lesson": { "listTitle": "...", "accessPaid": "..." },
  "video":  { "listTitle": "...", "emptyTitle": "..." },
  "error":  { "notFoundTitle": "...", "accessDeniedTitle": "..." }
}
```

- **규칙**: ko.json과 en.json의 키 구조는 **반드시 1:1 일치**해야 한다.
- **보간(Interpolation)**: `{{variable}}` 문법 사용 (예: `"총 {{count}}개"`)

##### 코드 사용 패턴

| 컨텍스트 | 패턴 | 예시 |
|----------|------|------|
| React 컴포넌트 내부 | `useTranslation` 훅 | `const { t } = useTranslation();` → `t("auth.loginTitle")` |
| React 컴포넌트 외부 (Hook, Zod 스키마 등) | `i18n.t()` 직접 호출 | `import i18n from "@/i18n";` → `i18n.t("common.requestFailed")` |
| 언어 변경 | `changeLanguage` 헬퍼 | `import { changeLanguage } from "@/i18n";` → `changeLanguage("en")` |

##### 언어 전환 UI & 동기화

- **헤더 토글**: Globe 아이콘 버튼으로 ko↔en 전환
  - 데스크톱: `"EN"` / `"KO"` 약어 표시
  - 모바일: `"English"` / `"한국어"` 전체 표시 (전환 대상 언어를 해당 언어로 표기)
  - 로그인 상태일 경우 `useUpdateSettings`로 DB에도 저장
- **설정 페이지**: Select 드롭다운으로 언어 선택 → 저장 시 DB + i18n 동시 적용
- **동기화**: 헤더 토글 변경 시 `i18n.language` 변경 감지를 통해 설정 페이지 form에 즉시 반영

##### 적용 범위

| 대상 | i18n 적용 | 비고 |
|------|-----------|------|
| 사용자 대면 페이지 (홈, 로그인, 학습 등) | O | 모든 UI 텍스트 `t()` 처리 |
| 레이아웃 (헤더, 푸터) | O | |
| 에러 페이지 (404, 403, 500) | O | |
| 관리자(Admin) 페이지 | X | 한국어 전용 (관리자가 한국어 사용자) |
| Zod 유효성 검증 메시지 | O | `i18n.t()` 패턴 사용 |
| Toast 알림 메시지 | O | Hook 내에서 `i18n.t()` 사용 |

---

### 6.3 라우팅 & 접근 제어

> 목적: 5. 기능 & API 로드맵의 "화면 경로"를 기준으로, **Code Splitting이 적용된 React Router 트리**와 **엄격한 접근 제어(Auth/Admin Guard)**를 정의한다.

#### 6.3.1 라우트 매핑 원칙 (Lazy Loading 필수)

- **라우트 정의 위치**
  - `src/app/router.tsx` 에서 **전체 라우트 트리**를 정의한다.
  - **성능 원칙**: 모든 페이지 컴포넌트는 `React.lazy`로 import하여, 초기 번들 사이즈를 최소화해야 한다.

- **파일명 패턴 (예시)**
  - `/` → `category/home/page/HomePage.tsx` (홈)
  - `/about` → `category/about/page/AboutPage.tsx` (소개)
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
const HomePage = lazy(() => import("@/category/home/page/HomePage"));
const AboutPage = lazy(() => import("@/category/about/page/AboutPage"));
const LoginPage = lazy(() => import("@/category/auth/page/LoginPage"));
const SignupPage = lazy(() => import("@/category/auth/page/SignupPage"));
const VideoListPage = lazy(() => import("@/category/video/page/VideoListPage"));
const VideoDetailPage = lazy(() => import("@/category/video/page/VideoDetailPage"));
const StudyListPage = lazy(() => import("@/category/study/page/StudyListPage"));
const LessonListPage = lazy(() => import("@/category/lesson/page/LessonListPage"));
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
            <Route path="/" element={<HomePage />} />
            <Route path="/about" element={<AboutPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/signup" element={<SignupPage />} />
            <Route path="/account-recovery" element={<AccountRecoveryPage />} />
            <Route path="/verify-email" element={<VerifyEmailPage />} />
            <Route path="/videos" element={<VideoListPage />} />
            <Route path="/videos/:video_id" element={<VideoDetailPage />} />
            <Route path="/studies" element={<StudyListPage />} />
            <Route path="/lessons" element={<LessonListPage />} />

            {/* Protected Routes (Member) */}
            <Route element={<RequireAuth />}>
              <Route path="/me" element={<MePage />} />
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

- **`RequireAdmin` (관리자 RBAC)** ✅ 구현 완료 (2026-02-01)
  - **로직**:
    - `RequireAuth` 통과 AND `user_auth_enum` IN `['HYMN', 'admin']`
    - ⚠️ `manager` 역할은 **admin 접근 불가** (향후 class 기반 접근 권한으로 별도 구현 예정)
  - **실패 시 처리**:
    - 인증은 되었으나 권한 부족 → `/403` 페이지로 리다이렉트
    - *절대 로그인 페이지로 튕겨내지 않는다 (무한 루프 방지).*
  - **백엔드 미들웨어** (`src/api/admin/role_guard.rs`):
    - HYMN/admin → 200 통과
    - manager → 403 "Access denied: Manager role requires class-based access"
    - learner → 403 "Access denied: Insufficient permissions for admin access"

- **에러 페이지** ✅ 구현 완료 (2026-02-01)
  - 위치: `frontend/src/category/error/page/`
  - 페이지 목록:
    | 라우트 | 컴포넌트 | 설명 |
    |--------|----------|------|
    | `/403` | `AccessDeniedPage` | 권한 없음 (ShieldX 아이콘) |
    | `/error` | `ErrorPage` | 서버 에러 (ServerCrash 아이콘, 재시도 버튼) |
    | `*` | `NotFoundPage` | 404 페이지 없음 (FileQuestion 아이콘) |

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

- **AppShell (`components/layout/RootLayout.tsx`)**
  - 앱의 최상위 껍데기.
  - 구성:
    - **Header**: 로고 + 햄버거 메뉴(모바일) / 네비게이션(데스크톱) + 로그인/로그아웃 버튼
    - **Main**: `max-w-screen-xl mx-auto px-4` (콘텐츠 중앙 정렬 및 가로 여백 확보)
    - **Footer**: 회사 정보, 연락처, 이용약관/개인정보처리방침 링크

- **Header 네비게이션 구조**
  ```
  ┌─────────────────────────────────────────────────────────────────┐
  │ [Amazing Korean]    [소개] [영상] [학습] [수업]     [로그인/로그아웃] │
  │      (Logo)           (Navigation)                  (Auth)       │
  └─────────────────────────────────────────────────────────────────┘
  ```
  - **왼쪽 (Logo)**: "Amazing Korean" 텍스트 로고 (클릭 시 `/` 홈으로 이동)
  - **가운데 (Navigation)**: 메인 메뉴
    | 메뉴명 | 라우트 | 설명 |
    |--------|--------|------|
    | 소개 | `/about` | 서비스 소개 |
    | 영상 | `/videos` | 영상 목록 |
    | 학습 | `/studies` | 학습 목록 |
    | 수업 | `/lessons` | 수업 목록 |
  - **오른쪽 (Auth)**: 인증 상태에 따른 조건부 렌더링
    - 비로그인: `[로그인]` `[회원가입]` 버튼
    - 로그인: `[마이페이지]` `[로그아웃]` 버튼

- **반응형 전략**
  - **Grid**: `grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6` 패턴을 기본으로 한다.
  - **Spacing**: 모바일에서는 `gap-4`, 데스크톱에서는 `gap-6` 이상을 사용하여 시원한 느낌을 준다.

#### 6.5.3 Tailwind & Color System (Theme)

> 상세 사용법, 금지 규칙, PR 체크리스트: [`docs/AMK_DESIGN_SYSTEM.md`](AMK_DESIGN_SYSTEM.md)

- **색상 토큰 (index.css 기반)**
  - `primary`: 브랜드 메인 (Navy `#051D55`) → 주요 액션, Footer
  - `secondary`: 보조 (Blue `#4F71EB`) → 서브 버튼, 데코
  - `accent`: 강조 (Cyan `#129DD8`) → 학습 완료, 아이콘
  - `destructive`: 위험/삭제/에러 (= error 통일)
  - `muted`: 비활성/배경
  - `brand-soft` / `brand-soft-alt`: Hero 그라데이션 배경
  - `status-success` / `warning` / `info`: 상태 색상 (HSL + foreground 세트)

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

### 6.6 프론트 테스트 & 로컬 개발 (요약)

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

> 빌드, 배포, CI/CD, EC2 유지보수 등은 [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) 참조

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
