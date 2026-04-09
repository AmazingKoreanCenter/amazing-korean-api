/**
 * Figma 레퍼런스 캡처 대상 페이지 목록
 *
 * 우선순위는 project_figma_plan.md 기준.
 * - 1순위: 공개 핵심 (Home, About, FAQ, ComingSoon)
 * - 2순위: Book 도메인
 * - 3순위: 인증
 * - 4순위: 법적
 *
 * 5순위(MyPage/Settings, 로그인 필요)는 이번 Phase 제외.
 */

export type PageSpec = {
  /** 우선순위 (1~4) */
  priority: 1 | 2 | 3 | 4;
  /** 도메인 그룹명 — 출력 폴더에 사용 */
  group: string;
  /** 캡처 파일명 prefix (kebab-case) */
  slug: string;
  /** 접근 경로 */
  path: string;
  /** 설명 (문서용) */
  note?: string;
};

// Book Landing 샘플 ISBN — frontend/src/category/book/book_data.ts 첫 엔트리
const SAMPLE_ISBN = "9791199772700"; // English / student edition

export const CAPTURE_PAGES: PageSpec[] = [
  // ── 1순위: 공개 핵심 ────────────────────────────────
  { priority: 1, group: "01-public", slug: "home", path: "/" },
  { priority: 1, group: "01-public", slug: "about", path: "/about" },
  { priority: 1, group: "01-public", slug: "faq", path: "/faq" },
  {
    priority: 1,
    group: "01-public",
    slug: "coming-soon",
    path: "/pricing",
    note: "Videos/Studies/Lessons/Pricing 공용 ComingSoonPage",
  },

  // ── 2순위: Book 도메인 ──────────────────────────────
  { priority: 2, group: "02-book", slug: "book-hub", path: "/book" },
  { priority: 2, group: "02-book", slug: "textbook-catalog", path: "/book/textbook" },
  { priority: 2, group: "02-book", slug: "ebook-catalog", path: "/book/ebook" },
  {
    priority: 2,
    group: "02-book",
    slug: "book-landing",
    path: `/book/${SAMPLE_ISBN}`,
    note: `샘플 ISBN ${SAMPLE_ISBN} (English / student)`,
  },

  // ── 3순위: 인증 ──────────────────────────────────────
  { priority: 3, group: "03-auth", slug: "login", path: "/login" },
  { priority: 3, group: "03-auth", slug: "signup", path: "/signup" },
  {
    priority: 3,
    group: "03-auth",
    slug: "request-reset",
    path: "/request-reset-password",
  },
  { priority: 3, group: "03-auth", slug: "verify-email", path: "/verify-email" },
  { priority: 3, group: "03-auth", slug: "account-recovery", path: "/find-id" },

  // ── 4순위: 법적 ──────────────────────────────────────
  { priority: 4, group: "04-legal", slug: "terms", path: "/terms" },
  { priority: 4, group: "04-legal", slug: "privacy", path: "/privacy" },
  { priority: 4, group: "04-legal", slug: "refund-policy", path: "/refund-policy" },
];

export const THEMES = ["light", "dark"] as const;
export type Theme = (typeof THEMES)[number];
