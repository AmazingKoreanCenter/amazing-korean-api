# `/book/:isbn` QR 코드 랜딩 페이지 구현 계획

## Context

교재 속표지에 QR 코드를 삽입함 (URL: `amazingkorean.net/book/{isbn13}`).
사용자가 실물 교재의 QR을 스캔하면 해당 교재 정보 + 서비스 안내 페이지로 연결해야 함.
현재 `/book/:isbn` 라우트 미구현 상태.

## 핵심 결정: 순수 프론트엔드 구현 (백엔드 엔드포인트 불필요)

- ISBN → 언어/에디션 매핑은 `cover.json`의 정적 데이터
- DB 조회, 인증, 사용자별 로직 없음
- 마케팅/디스커버리 랜딩 페이지 성격

## ISBN → 언어 매핑 (20개 ISBN)

| 언어 | Student ISBN | Teacher ISBN |
|------|-------------|-------------|
| en | 979-11-997727-0-0 | 979-11-998162-0-6 |
| zh_cn | 979-11-997727-1-7 | 979-11-998162-1-3 |
| ja | 979-11-997727-2-4 | 979-11-998162-2-0 |
| th | 979-11-997727-3-1 | 979-11-998162-3-7 |
| ne | 979-11-997727-4-8 | 979-11-998162-4-4 |
| vi | 979-11-997727-5-5 | 979-11-998162-5-1 |
| ru | 979-11-997727-6-2 | 979-11-998162-6-8 |
| km | 979-11-997727-7-9 | 979-11-998162-7-5 |
| tl | 979-11-997727-8-6 | 979-11-998162-8-2 |
| id | 979-11-997727-9-3 | 979-11-998162-9-9 |

## 구현 단계

### 1단계: 파일 구조 생성

```
frontend/src/category/book/
├── types.ts                    # ISBN → metadata 타입
├── book_data.ts                # cover.json 기반 정적 ISBN 룩업
└── page/
    └── book_landing_page.tsx   # 메인 페이지 컴포넌트
```

### 2단계: ISBN 데이터 모듈 (`book_data.ts`)

- `cover.json` 데이터를 프론트엔드용 정적 맵으로 변환
- `findBookByISBN(isbn: string)` → `{ language, edition, nameLocal, nameKorean, nameEnglish, sealColor, ... } | null`
- ISBN 포맷 검증 (13자리, 979-11-xxx 패턴)

### 3단계: 랜딩 페이지 컴포넌트

**레이아웃 구성:**

```
┌──────────────────────────────────────┐
│  Amazing Korean 로고 + 네비게이션    │
├──────────────────────────────────────┤
│                                      │
│  [국기] {언어명}                      │
│  Amazing Korean Basic                │
│  놀라운 한국어 기초                   │
│                                      │
│  {Student's/Teacher's Edition}       │
│  ISBN: 979-11-xxxxxx-x-x            │
│  124p | A4 | ₩25,000                │
│                                      │
├──────────────────────────────────────┤
│                                      │
│  [E-book 구매하기]  [교재 주문하기]   │
│  [무료 회원가입]    [학습 시작하기]   │
│                                      │
├──────────────────────────────────────┤
│                                      │
│  시리즈 소개 / 다른 언어 교재        │
│                                      │
└──────────────────────────────────────┘
```

**언어 자동 전환:**
- ISBN에서 언어 파악 → `i18n.changeLanguage(lang)` 호출
- 예: 베트남어 교재 QR 스캔 → UI가 베트남어로 자동 전환
- 지원 언어(22개)에 해당하면 전환, 미지원이면 현재 설정 유지

**CTA 분기:**
- 비로그인: "E-book 구매" → `/ebook`, "교재 주문" → `/textbook`, "회원가입" → `/signup`
- 로그인: "E-book 구매" → `/ebook`, "교재 주문" → `/textbook`, "내 E-book" → `/ebook/my`, "학습하기" → `/`

**잘못된 ISBN**: "교재를 찾을 수 없습니다" + 홈/카탈로그 링크

### 4단계: 라우트 등록

파일: `frontend/src/app/routes.tsx`

```tsx
// Public routes (RootLayout 내부)
<Route path="/book/:isbn" element={<BookLandingPage />} />
```

기존 public 라우트(`/textbook`, `/ebook`) 옆에 배치.

### 5단계: i18n 키 추가

파일: `frontend/src/i18n/locales/ko.json`, `en.json` (+ 나머지 20개 언어)

```json
"book": {
  "title": "교재 정보",
  "edition_student": "학생용",
  "edition_teacher": "교사용",
  "pages": "페이지",
  "price": "정가",
  "buy_ebook": "E-book 구매하기",
  "order_textbook": "교재 주문하기",
  "signup_free": "무료 회원가입",
  "start_learning": "학습 시작하기",
  "my_ebooks": "내 E-book",
  "not_found": "교재를 찾을 수 없습니다",
  "series_intro": "Amazing Korean 시리즈",
  "other_languages": "다른 언어 교재"
}
```

### 6단계: SEO 메타 태그

- `<PageMeta>` 컴포넌트 사용 (기존 패턴)
- 동적 타이틀: `Amazing Korean Basic - {언어명}`
- OG 태그: 교재 커버 이미지, 설명

## 핵심 파일

| 파일 | 변경 내용 |
|------|----------|
| `frontend/src/category/book/book_data.ts` | **신규** — ISBN 정적 룩업 |
| `frontend/src/category/book/types.ts` | **신규** — 타입 정의 |
| `frontend/src/category/book/page/book_landing_page.tsx` | **신규** — 랜딩 페이지 |
| `frontend/src/app/routes.tsx` | 라우트 추가 |
| `frontend/src/i18n/locales/*.json` | i18n 키 추가 (22개 파일) |

## 검증

```bash
cd frontend && npm run build   # 빌드 확인
```

- 유효한 ISBN으로 접속 → 교재 정보 표시 확인
- 잘못된 ISBN → 404 처리 확인
- 비로그인/로그인 상태에서 CTA 버튼 분기 확인
- 모바일 반응형 확인 (QR 스캔은 대부분 모바일)
