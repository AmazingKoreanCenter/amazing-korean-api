// Phase S1 베이스라인 측정 대상.
// 16개 figma-capture/pages.ts 중 핵심 8개로 압축 — Lighthouse 1회당 ~30s,
// 8 페이지 × pre/post = 2회 측정 ≈ 8분으로 세션 시간 절약.

// figma-capture/pages.ts와 동일한 그룹 명명 규칙 사용 (01-xxx, 02-xxx, ...).
// 전체 16페이지 중 핵심 8개만 선택 — 측정 1회 ~1분, pre/post 2회 = ~2분으로
// 세션 시간 절약. 나머지 8페이지(auth 5 + legal 3)는 구조가 유사해 대표 측정으로 충분.
export const AUDIT_PAGES = [
  // 01 공개 핵심 (4)
  { slug: "home", path: "/", group: "01-public" },
  { slug: "about", path: "/about", group: "01-public" },
  { slug: "faq", path: "/faq", group: "01-public" },
  { slug: "coming-soon", path: "/pricing", group: "01-public" },

  // 02 Book 도메인 (3) — book-landing은 동적 ISBN 필요로 제외
  { slug: "book-hub", path: "/book", group: "02-book" },
  { slug: "textbook-catalog", path: "/book/textbook", group: "02-book" },
  { slug: "ebook-catalog", path: "/book/ebook", group: "02-book" },

  // 03 인증 (1) — login만 (signup/reset은 흐름이 비슷해 대표 1개로 충분)
  { slug: "login", path: "/login", group: "03-auth" },
];
