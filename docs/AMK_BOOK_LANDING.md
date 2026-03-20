# `/book/:isbn` QR 코드 랜딩 페이지 구현 명세

> **대상 프로젝트**: amazing-korean-api (frontend)
> **최종 업데이트**: 2026-03-20

---

## 1. 배경

교재 속표지에 QR 코드가 삽입되어 있음 (URL: `https://amazingkorean.net/book/{isbn13}`).
사용자가 실물 교재의 QR을 스캔하면 해당 교재 정보 + 서비스 안내 페이지로 이동.
현재 `/book/:isbn` 라우트 미구현 상태.

### 핵심 결정: 순수 프론트엔드 구현

- ISBN → 언어/에디션 매핑은 정적 데이터 (DB 불필요)
- 백엔드 엔드포인트 없음
- 마케팅/디스커버리 랜딩 페이지 성격

---

## 2. ISBN 데이터

### 2.1 ISBN → 언어/에디션 매핑 (20개)

| 언어 | 언어명 | Student ISBN | Teacher ISBN |
|------|--------|-------------|-------------|
| en | English | 979-11-997727-0-0 | 979-11-998162-0-6 |
| zh_cn | 简体中文 | 979-11-997727-1-7 | 979-11-998162-1-3 |
| ja | 日本語 | 979-11-997727-2-4 | 979-11-998162-2-0 |
| th | ภาษาไทย | 979-11-997727-3-1 | 979-11-998162-3-7 |
| ne | नेपाली | 979-11-997727-4-8 | 979-11-998162-4-4 |
| vi | Tiếng Việt | 979-11-997727-5-5 | 979-11-998162-5-1 |
| ru | Русский | 979-11-997727-6-2 | 979-11-998162-6-8 |
| km | ភាសាខ្មែរ | 979-11-997727-7-9 | 979-11-998162-7-5 |
| tl | Filipino | 979-11-997727-8-6 | 979-11-998162-8-2 |
| id | Bahasa Indonesia | 979-11-997727-9-3 | 979-11-998162-9-9 |

### 2.2 데이터 출처

`amazing-korean-books/scripts/textbook/data/cover.json` → `languages[lang].isbn`

ISBN이 없는 12개 언어(mn, my, zh_tw, hi, si, es, pt, fr, de, uz, kk, tg)는 QR 코드 미삽입 → 이 라우트로 유입 없음.

---

## 3. 기술 스택 (API 프로젝트 기준)

| 항목 | 기술 |
|------|------|
| 프레임워크 | React 19 + Vite + TypeScript |
| 라우팅 | React Router v7 (SPA) |
| UI | shadcn/ui + Tailwind CSS |
| 상태 | Zustand (auth), TanStack Query (서버) |
| i18n | react-i18next (22개 언어) |
| SEO | `PageMeta` 컴포넌트 (`titleKey`, `descriptionKey`) |
| 아이콘 | lucide-react |

---

## 4. 파일 구조

```
frontend/src/category/book/
├── types.ts                    # BookInfo 타입, ISBN 유효성 스키마
├── book_data.ts                # ISBN → BookInfo 정적 룩업 맵
└── page/
    └── book_landing_page.tsx   # 메인 랜딩 페이지 컴포넌트
```

### 4.1 `types.ts`

```typescript
import { z } from "zod";

export const bookEditionSchema = z.enum(["student", "teacher"]);
export type BookEdition = z.infer<typeof bookEditionSchema>;

export interface BookInfo {
  isbn13: string;           // "9791199772705"
  language: string;         // "en"
  edition: BookEdition;     // "student" | "teacher"
  nameLocal: string;        // "English"
  nameKorean: string;       // "영어"
  sealColor: string;        // "#1A3C72" (표지 컬러)
  flagFile: string;         // "emoji_u1f1ec_1f1e7.svg"
}
```

### 4.2 `book_data.ts`

```typescript
import type { BookInfo } from "./types";

// cover.json 기반 정적 맵 (20개 ISBN)
const BOOK_MAP: Record<string, BookInfo> = {
  "9791199772700": { isbn13: "9791199772700", language: "en", edition: "student", nameLocal: "English", nameKorean: "영어", sealColor: "#1A3C72", flagFile: "emoji_u1f1ec_1f1e7.svg" },
  "9791199816206": { isbn13: "9791199816206", language: "en", edition: "teacher", nameLocal: "English", nameKorean: "영어", sealColor: "#1A3C72", flagFile: "emoji_u1f1ec_1f1e7.svg" },
  // ... 나머지 18개
};

export function findBookByISBN(isbn: string): BookInfo | null {
  // 하이픈 제거 + 13자리 정규화
  const normalized = isbn.replace(/-/g, "");
  return BOOK_MAP[normalized] ?? null;
}
```

### 4.3 `book_landing_page.tsx`

```typescript
import { useParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useEffect } from "react";
import { BookOpen, ShoppingCart, UserPlus, GraduationCap } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { PageMeta } from "@/components/page_meta";
import { useAuthStore } from "@/hooks/use_auth_store";

import { findBookByISBN } from "../book_data";

export function BookLandingPage() {
  const { isbn } = useParams<{ isbn: string }>();
  const { t, i18n } = useTranslation();
  const { isLoggedIn } = useAuthStore();
  const book = isbn ? findBookByISBN(isbn) : null;

  // ISBN → 언어 자동 전환
  useEffect(() => {
    if (book) {
      i18n.changeLanguage(book.language);
    }
  }, [book, i18n]);

  if (!book) return <NotFoundView />;
  return <BookInfoView book={book} isLoggedIn={isLoggedIn} />;
}
```

---

## 5. 라우트 등록

**파일**: `frontend/src/app/routes.tsx`

```tsx
// 기존 public 라우트 사이에 추가
<Route path="/book/:isbn" element={<BookLandingPage />} />
```

`/ebook` 라우트 아래에 배치 (public, 인증 불필요).

---

## 6. 페이지 레이아웃

```
┌──────────────────────────────────────────┐
│  Header (RootLayout)                     │
├──────────────────────────────────────────┤
│                                          │
│  [🇻🇳] Tiếng Việt                        │
│                                          │
│  Amazing Korean Basic                    │
│  놀라운 한국어 기초                        │
│                                          │
│  Student's Edition · 학생용               │
│  ISBN 979-11-997727-5-5                  │
│  124p · A4 · ₩25,000                    │
│                                          │
├──────────────────────────────────────────┤
│                                          │
│  [📖 E-book 구매]    [📦 교재 주문]       │
│  [👤 무료 회원가입]   [🎓 학습 시작]       │
│                                          │
├──────────────────────────────────────────┤
│                                          │
│  Amazing Korean 시리즈                    │
│  "언어의 차이를 인정하고..."              │
│                                          │
│  다른 언어 교재 보기                      │
│  [🇬🇧 EN] [🇯🇵 JA] [🇨🇳 ZH] ...         │
│                                          │
├──────────────────────────────────────────┤
│  Footer (RootLayout)                     │
└──────────────────────────────────────────┘
```

### 6.1 섹션별 상세

**Hero 영역**
- 국기 SVG + 언어명 (nameLocal + nameKorean)
- 교재 제목: "Amazing Korean Basic" / "놀라운 한국어 기초"
- 에디션: Student's Edition / Teacher's Edition
- 메타: ISBN, 페이지 수(124p), 판형(A4), 가격(₩25,000)

**CTA 버튼 영역**
| 상태 | 버튼 구성 |
|------|----------|
| 비로그인 | E-book 구매(`/ebook`), 교재 주문(`/textbook`), 무료 회원가입(`/signup`), 학습 시작(`/`) |
| 로그인 | E-book 구매(`/ebook`), 교재 주문(`/textbook`), 내 E-book(`/ebook/my`), 학습하기(`/`) |

**시리즈 소개**
- `cover.json`의 `backCoverMessage` 인용문
- 다른 언어 교재 링크: ISBN 있는 10개 언어 국기 버튼 → 각 `/book/{isbn}`

**잘못된 ISBN**
- "교재를 찾을 수 없습니다" + 홈(`/`) 및 교재 카탈로그(`/textbook`) 링크

### 6.2 모바일 최적화

QR 스캔은 대부분 모바일 → 모바일 퍼스트 디자인:
- CTA 버튼: 세로 풀너비 스택
- 국기 + 교재 정보: 단일 컬럼
- 다른 언어: 가로 스크롤 또는 그리드 2열

---

## 7. i18n 키

**파일**: `frontend/src/i18n/locales/{lang}.json` (22개)

`ko.json`에 `book` 네임스페이스 추가:

```json
"book": {
  "title": "교재 정보",
  "editionStudent": "학생용",
  "editionTeacher": "교사용",
  "pages": "{{count}}페이지",
  "price": "정가",
  "buyEbook": "E-book 구매하기",
  "orderTextbook": "교재 주문하기",
  "signupFree": "무료 회원가입",
  "startLearning": "학습 시작하기",
  "myEbooks": "내 E-book",
  "notFound": "교재를 찾을 수 없습니다",
  "notFoundDesc": "유효하지 않은 ISBN입니다. 홈페이지로 이동하거나 교재 목록을 확인하세요.",
  "goHome": "홈으로",
  "viewCatalog": "교재 목록",
  "seriesIntro": "Amazing Korean 시리즈",
  "otherLanguages": "다른 언어 교재"
}
```

`seo` 네임스페이스에 추가:

```json
"seo": {
  "book": {
    "title": "Amazing Korean Basic {{language}} - 교재 정보",
    "description": "Amazing Korean Basic {{language}} 교재의 상세 정보와 구매 안내입니다."
  }
}
```

---

## 8. 언어 자동 전환 로직

```
사용자 QR 스캔
  → /book/9791199772755
  → findBookByISBN("9791199772755")
  → { language: "vi", edition: "student", ... }
  → i18n.changeLanguage("vi")
  → UI 전체가 베트남어로 전환
```

- ISBN에서 파악된 언어가 i18n 지원 언어(22개)에 해당하면 자동 전환
- 사용자가 수동으로 언어를 변경하면 해당 설정 유지 (localStorage)

---

## 9. SEO

```tsx
<PageMeta
  titleKey="seo.book.title"
  titleParams={{ language: book.nameLocal }}
  descriptionKey="seo.book.description"
  descriptionParams={{ language: book.nameLocal }}
/>
```

- 동적 타이틀: `Amazing Korean Basic Vietnamese - 교재 정보`
- OG 이미지: 추후 교재 표지 이미지 연동 가능 (현재는 기본 OG 이미지)

---

## 10. 구현 순서

| 단계 | 작업 | 파일 |
|:----:|------|------|
| 1 | 타입 + ISBN 데이터 모듈 | `book/types.ts`, `book/book_data.ts` |
| 2 | 랜딩 페이지 컴포넌트 | `book/page/book_landing_page.tsx` |
| 3 | 라우트 등록 | `app/routes.tsx` |
| 4 | i18n 키 추가 | `i18n/locales/*.json` (22개 파일) |
| 5 | SEO 메타 추가 | `i18n/locales/*.json` seo 섹션 |
| 6 | 빌드 확인 | `cd frontend && npm run build` |

---

## 11. 검증 체크리스트

- [ ] 유효한 ISBN 접속 → 교재 정보 정상 표시
- [ ] ISBN에 따른 i18n 언어 자동 전환 동작
- [ ] 잘못된 ISBN → "교재를 찾을 수 없습니다" 페이지
- [ ] 비로그인 CTA: E-book 구매, 교재 주문, 회원가입, 학습 시작
- [ ] 로그인 CTA: E-book 구매, 교재 주문, 내 E-book, 학습하기
- [ ] 다른 언어 교재 링크 → 해당 ISBN 랜딩으로 이동
- [ ] 모바일 반응형 (세로 CTA, 단일 컬럼)
- [ ] `npm run build` 통과

---

## 12. 참조 파일

| 위치 | 파일 | 용도 |
|------|------|------|
| books 프로젝트 | `scripts/textbook/data/cover.json` | ISBN + 언어 메타데이터 원본 |
| books 프로젝트 | `scripts/textbook/lib/barcode.js` | QR 코드 생성 로직 (`generateQRCodeSVG`) |
| books 프로젝트 | `scripts/textbook/components/cover_inner.js` | QR 배치 (속표지 우상단 20mm×20mm) |
| books 프로젝트 | `docs/AMK_BOOKS_MASTER.md` §7.1 | QR 랜딩 교차참조 |
| API 프로젝트 | `frontend/src/app/routes.tsx` | 라우트 등록 위치 |
| API 프로젝트 | `frontend/src/category/textbook/` | 유사 패턴 참조 (types, hook, page 구조) |
| API 프로젝트 | `frontend/src/i18n/locales/ko.json` | i18n 키 추가 위치 |
