# 홈페이지 레이아웃 디자인 구현

> **날짜**: 2026-01-24
> **목표**: Header, Footer, Layout 컴포넌트 구현 및 홈페이지 리디자인

---

## 현재 상태

- 레이아웃 컴포넌트 없음 (Header, Footer, Nav 모두 없음)
- `home_page.tsx`: 개발용 DEV ROUTE MAP만 존재
- 모든 페이지가 개별적으로 렌더링됨 (공통 레이아웃 없음)
- shadcn/ui 컴포넌트 사용 중 (Button, Card 등)

---

## 구현 계획

### 1. Header 컴포넌트 (`components/layout/header.tsx`)

```
┌─────────────────────────────────────────────────────────────────┐
│ [Amazing Korean]    [소개] [영상] [학습] [수업]     [로그인/로그아웃] │
│      (Logo)           (Navigation)                  (Auth)       │
└─────────────────────────────────────────────────────────────────┘
```

**구성 요소:**
- **왼쪽**: "Amazing Korean" 로고 텍스트 (클릭 시 홈으로 이동)
- **가운데**: 네비게이션 메뉴
  - 소개 → `/about`
  - 영상 → `/videos`
  - 학습 → `/studies`
  - 수업 → `/lessons`
- **오른쪽**: 인증 버튼
  - 비로그인: [로그인] [회원가입]
  - 로그인: [마이페이지] [로그아웃]

**기능:**
- 로그인 상태에 따른 조건부 렌더링 (useAuthStore 활용)
- 반응형 디자인 (모바일: 햄버거 메뉴)
- 현재 페이지 활성화 표시

---

### 2. Footer 컴포넌트 (`components/layout/footer.tsx`)

**amazingkorean.net 참고 내용 (사용자 제공 필요):**

푸터에 포함될 예상 정보:
- 회사명/서비스명
- 연락처 정보 (이메일, 전화)
- 주소
- 이용약관, 개인정보처리방침 링크
- 저작권 표시 (Copyright © 2026 Amazing Korean)
- SNS 링크 (선택사항)

> ⚠️ **주의**: amazingkorean.net 푸터 내용을 웹에서 가져올 수 없었음.
> 사용자에게 푸터 내용 확인 필요.

---

### 3. RootLayout 컴포넌트 (`components/layout/root_layout.tsx`)

```tsx
<div className="min-h-screen flex flex-col">
  <Header />
  <main className="flex-1">
    <Outlet /> {/* 또는 children */}
  </main>
  <Footer />
</div>
```

---

### 4. 소개 페이지 (`category/about/page/about_page.tsx`)

- `/about` 라우트 추가
- 서비스 소개 내용

---

### 5. 홈페이지 리디자인 (`category/home/home_page.tsx`)

- DEV ROUTE MAP 제거
- 서비스 메인 콘텐츠 표시
- Hero 섹션 (서비스 소개)
- 주요 기능 소개 카드

---

## 수정 대상 파일

| 파일 | 작업 |
|------|------|
| `components/layout/header.tsx` | **신규 생성** |
| `components/layout/footer.tsx` | **신규 생성** |
| `components/layout/root_layout.tsx` | **신규 생성** |
| `category/about/page/about_page.tsx` | **신규 생성** |
| `app/routes.tsx` | RootLayout 적용, /about 라우트 추가 |
| `category/home/home_page.tsx` | 리디자인 |
| `index.html` | title 변경: "Amazing Korean" |

---

## 기술 스택

- React Router v6 `<Outlet />` 패턴
- shadcn/ui 컴포넌트
- Tailwind CSS
- zustand (useAuthStore)

---

## 검증

1. 모든 페이지에서 Header/Footer 표시 확인
2. 로그인/로그아웃 상태별 Header 변경 확인
3. 네비게이션 링크 동작 확인
4. 반응형 디자인 확인 (모바일/데스크톱)
5. `npm run build` 빌드 성공 확인

---

## ⚠️ 사용자 제공 예정

1. **푸터 내용**: 사용자가 직접 입력 예정 (회사 정보, 연락처 등)
2. **소개 페이지 내용**: 구현 시 확인 예정

---

## 구현 순서

1. `components/layout/` 폴더 생성
2. Header 컴포넌트 생성
3. Footer 컴포넌트 생성 (사용자 제공 내용 반영)
4. RootLayout 컴포넌트 생성
5. routes.tsx 수정 (Layout 적용)
6. home_page.tsx 리디자인
7. about_page.tsx 생성
8. index.html title 변경
9. 빌드 테스트
