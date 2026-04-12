# AMK_STATUS — 작업 현황 & 로드맵

> 완료 항목, 진행 예정, Paddle 전환, 보안 전략, 콘텐츠 개선.
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)
> 교재/출판: `amazing-korean-books` 프로젝트 참조

---

## 작업 현황

### 8.1 완료 항목 ✅

| # | 항목 | 카테고리 | 내역 | 완료일 | 향후 확장 |
|:-:|------|---------|------|:------:|----------|
| 1 | Admin 통계 API | 기능 | users/logins/studies/videos 통계 엔드포인트 + 프론트 UI | 2026-01 | 시스템 모니터링 (DB/Redis) |
| 2 | RBAC + Admin 감사 로그 | 보안 | role_guard, IP Allowlist, actor_user_id 전달, AdminRoute, 에러 페이지 | 2026-02-02 | manager class 기반 접근, 세분화 권한 |
| 3 | 코드 일관성 + 네이밍 | 코드 품질 | 함수명/URL 통일, Stateless 패턴, Refresh Token 포맷 등 8건 | 2026-02-02 | — |
| 4 | 내부 DB 작업 | 인프라 | Redis 보안(인증+포트), 시청 시간 추적, Study 레이트리밋, Course 도메인, 수강권 정책 | 2026-02-02 | — |
| 5 | Google OAuth | 외부 API | Authorization Code Flow, 프론트엔드 소셜 로그인 | 2026-02-03 | Apple OAuth (비용 보류) |
| 6 | Login/Login_log 개선 | 기능 | UA 서버파싱(woothee), 세션 컬럼 활성화, JWT jti, Geo 기본값 | 2026-02-05 | — |
| 7 | DB 암호화 (Phase 1~3) | 보안 | AES-256-GCM + HMAC Blind Index, 키 로테이션, 55+ call sites 적용 | 2026-02-08 | AWS KMS envelope → HSM |
| 8 | 프로덕션 배포 + 하드닝 | 인프라 | 통합 마이그레이션, Redis 보안, 보안 헤더, Swagger/Health 숨김, 404 Fallback | 2026-02-10 | — |
| 9 | 이메일 시스템 (Resend) | 외부 API | 회원가입 인증, 계정 복구, Rate Limiting, 관리자 초대, 도메인 검증 | 2026-02-09 | — |
| 10 | 다국어 (i18n) | 기능 | 21개 언어, 번역 CRUD API 7개, `?lang=` fallback, Noto Sans 동적 로딩 (Google Translate 해지 — 2026-03-24) | 2026-02-14 | — |
| 11 | 세션 보안 + MFA | 보안 | 역할별 TTL, 토큰 재사용 탐지 (409 Conflict), TOTP MFA + 백업 코드 10개, 강제 설정 가드 | 2026-02-14 | 동시 세션 제한, step-up MFA |
| 12 | 결제 시스템 (Paddle) | 외부 API | Webhook 9종, 구독 취소, 수강권 자동 부여/회수, 관리자 9개 API, Pricing UI (Paddle.js) | 2026-02-16 | Paddle Live 전환 |
| 13 | Design System v2/v3 | UI | 공유 컴포넌트 6개 (PaginationBar, EmptyState, SkeletonGrid, ListStatsBar, StatCard, Card CVA), 다크모드 (next-themes, CSS 변수 60+ 토큰, 22개 로케일), UI/UX 가이드라인 문서화 | 2026-02-19 | — |
| 14 | Paddle KYB 서류 제출 | 결제 | 사업자등록증 + 주주명세서 (한/영) Paddle Dashboard 업로드 | 2026-02-19 | Live 전환 심사 대기 (2~4 영업일) |
| 15 | CEO 영문 이름 통일 | 관리 | i18n 18개 로케일 `Kyoung Ryun KIM`, noscript `KIM KYEONGRYUN` (사업자등록증 기준) | 2026-02-19 | — |
| 16 | 교재 주문 시스템 | 기능 | 비회원 주문, 계좌이체, 20개 언어 × 2종, ₩25,000/권, 최소 10권, DB 4 ENUM + 3 테이블, 백엔드 8 API, 프론트 5 페이지, 견적서/주문확인서 인쇄, 약관 동의 모달, 상태 머신 검증, 이메일 알림 (주문확인/상태변경), Rate Limiting, Advisory Lock, Soft Delete | 2026-03-03 | 카드/Paddle 결제 |
| 17 | 교재 HTML 재구축 시스템 | 콘텐츠 | JSON(11) + JS 컴포넌트(10) + CSS(9) → HTML → PDF 자동 생성 파이프라인, 120페이지, 원본 99.2% 일치, Puppeteer CSS 전수 감사 | 2026-03-02 | 20개 언어 자동 생성 토대 |
| 18 | 교재 번역 Wave 1 | 콘텐츠 | ja, zh_cn, id, th — 923항목 × 4언어 번역, translate_extract/merge 도구, PDF 생성 완료 | 2026-03-02 | Wave 2~5 (16개 언어) |
| 19 | QR 교재 랜딩 페이지 | 프론트 | `/book/:isbn` — 교재 QR 스캔 → 서비스 연결. 10개 언어 × 2종(학생/교사) ISBN 20개, 3섹션 구조 (Hero+CTA, 서비스안내+다른언어, 하단CTA), PageMeta 동적 SEO, changeLanguage 자동 전환, 국기 SVG 10개 | 2026-03-20 | — |
| 21 | Coming Soon + 에러 페이지 개선 | 프론트 | 영상/학습/레슨 → ComingSoonPage (HeroSection + Feature 미리보기 + E-book CTA), 에러 페이지 RootLayout 통합 (Header/Footer 유지), i18n `comingSoon.*` 13키 추가 | 2026-03-23 | 콘텐츠 오픈 시 원래 컴포넌트 복원 |
| 20 | 홈/소개 페이지 문구 & UI 개선 | 프론트 | 플레이스홀더 숫자(1,000+ 영상, 50+ 강사, 10,000+ 수강생) → 실제 차별점(20개 언어, 500+ 핵심 문장, TOPIK 연계)으로 교체, Feature 3번 "1:1 수업" → "교재로 정리하기" (미구현→구현 기능), Trust Indicators text-gradient, Feature/Value 카드 호버 강화, 아이콘 교체 (Layers/Languages/GraduationCap/BookMarked), 문서 검증 후 미구현 기능 5개 수정 | 2026-03-23 | — |
| 22 | 교재 카탈로그 리디자인 | 프론트 | 표지 이미지 기반 카탈로그 페이지(`/textbook`), 주문 폼 분리(`/textbook/order`), 헤더 "교재" 메뉴 추가, 카탈로그→주문 URL 파라미터 연동, 주문 항목 표지 썸네일 | 2026-03-23 | — |
| 23 | 교재 카탈로그 캐러셀 뷰 | 프론트 | 그리드 ↔ 캐러셀 뷰 토글 (localStorage, 기본 carousel), Swiper v12 Coverflow 캐러셀 (국가 씰 SVG + Thumbs 동기화), 학생/교사 탭, 언어 검색 필터, 50/50 레이아웃 (씰 리스트 + 선택 교재 상세), ISBN 뱃지, 반응형 (터치 스와이프), 22개 로케일 i18n | 2026-03-25 | — |
| 24 | 교재 상세 모달 | 프론트 | 교재 클릭 → 상세 모달 (겉표지/속표지/목차 갤러리 + 설명 + 주문 버튼), 그리드/캐러셀 양쪽 뷰에서 재사용, 이미지 fallback 처리, 22개 로케일 i18n | 2026-03-24 | 속표지/목차 이미지 추가 |
| 25 | 구매이력 + 비회원 주문 차단 | 기능 | `textbook.user_id` 컬럼 추가 (마이그레이션), 주문 생성 인증 필수화 (`AuthUser`), `GET /textbook/my` 내 주문 목록 API, 마이페이지 "구매이력" 버튼, 주문 폼 사용자 정보 자동 채움, `/textbook/order` PrivateRoute 이동 | 2026-03-24 | — |
| 26 | 견적서/주문확인서 사용자 공개 | 프론트 | 사용자용 견적서/주문확인서 인쇄 페이지 (`/textbook/order/:code/print?type=quote\|confirmation`), 주문 상태 페이지에 견적서/주문확인서 버튼, 주문 완료 화면에 견적서 버튼, `window.print()` + PDF 저장 안내, 22개 로케일 i18n | 2026-03-24 | — |
| 27 | E-book 구매 완료 안내 페이지 | 프론트 | 구매 완료 전용 페이지 (`/ebook/purchase-complete`), 구매코드 표시+복사, 에디션/가격/결제수단 요약, 계좌이체 입금안내, `location.state` 데이터 전달 (직접 접근 시 fallback), 22개 로케일 i18n | 2026-03-24 | — |
| 28 | E-book 샘플 미리보기 | 프론트 | 카탈로그 언어 카드에 "미리보기" 버튼, 모달 (표지/목차/샘플1/샘플2 이미지 갤러리 + 가격/페이지수 + 구매 버튼), 정적 이미지 방식 (`/ebook-previews/{edition}/{language}/`), 이미지 fallback, 22개 로케일 i18n | 2026-03-24 | 이미지 업로드 |
| 29 | E-book 이메일 알림 | 기능 | 구매 접수(pending) 확인 이메일 + 결제 완료(completed) 열람 안내 이메일, Resend 패턴 (fire-and-forget), 관리자 상태 변경 + Paddle webhook 양쪽 대응, 암호화 이메일 복호화(CryptoService) | 2026-03-24 | — |
| 30 | E-book 환불 정책 | 법적 | 환불 정책 제5조 E-book 섹션 추가 (pending 즉시취소, 미열람 7일 환불, 열람 후 불가, Paddle 별도), 카탈로그에 환불 정책 링크, 22개 로케일 i18n | 2026-03-24 | — |
| 31 | E-book 모바일 최적화 | 프론트 | 터치 스와이프 페이지 네비게이션 (50px 수평 임계값), 모바일(768px 미만) spread 자동 비활성화+토글 숨김, 모바일 UI 축소 (페이지/줌 텍스트 숨김, 하단 바 compact, 슬라이더 flex 확장) | 2026-03-24 | — |
| 32 | E-book Paddle 결제 연동 | 기능 | 카탈로그 결제 방식 선택 (계좌이체/카드결제), usePaddle 훅의 openEbookCheckout() 연동, checkout.completed 이벤트 → 완료 페이지, custom_data { type: "ebook", purchase_code } 전달, 22개 로케일 i18n | 2026-03-24 | — |
| 33 | 교재 ISBN 발급 상태 표시 | 기능 | 카탈로그 API에 isbn_ready 필드 추가 (9개 발급 완료), 카드/캐러셀/모달에 미발급 시 "약 1주 추가 소요" 안내, 주문 페이지 ISBN 미발급 언어 선택 시 알림, tl 누락 수정 | 2026-03-24 | — |
| 34 | Book 허브 + 라우트 재구성 | 프론트 | `/textbook/*`→`/book/textbook/*`, `/ebook/*`→`/book/ebook/*` 전면 이동, `/book` 허브 랜딩 (교재 소개 + i18n 기반 표지 + 샘플 5장 + CTA), 기존 경로 리다이렉트, 헤더 nav.book, E-book 카탈로그 Tabs 통일, 22개 로케일 i18n | 2026-03-25 | 샘플 이미지 220장 생성 |
| 35 | 교재 그리드/상세 모달 개선 | 프론트 | 그리드 카드 제목 통일 + ISBN 뱃지→모달 이동 + 버튼 "상세보기", 상세 모달 좌우 스와이프 갤러리 + ISBN 뱃지 우측 배치, 가격 우측 정렬 | 2026-03-25 | — |
| 36 | 교재 캐러셀 모바일 최적화 | 프론트 | 모바일: 상단 Coverflow 숨김 + 하단 Thumbs만 표시, 교재 상세 세로 쌓기 (표지 위 + 설명 아래) | 2026-03-25 | — |
| 37 | E-book 카탈로그 출판본 패턴 적용 | 프론트 | E-book 카탈로그 전면 리라이트 (출판본 패턴 통일): 그리드 CoverCard + 캐러셀 SealList + 상세 모달 (좌우 스와이프), HeroSection/Tabs/검색/뷰토글, SealList SealItem 인터페이스 일반화, 표지 이미지 공유, 22개 로케일 i18n 14키 추가 | 2026-03-25 | 구매 섹션 최종 디자인 결정 대기 |
| 38 | Book 허브 갤러리 + 라이트박스 + 가격 통일 + UI 개선 | 풀스택 | 허브 6슬라이드 갤러리(키워드 태그+스펙 카드), 상세 모달 3→6이미지 확장, blur 라이트박스(createPortal+Radix Dialog 호환), 카탈로그 도서/E-book 전환 탭, E-book 백엔드 가격 단일화(15,000 KRW/$9.99), Zod 유효성 검증 i18n(auth+주문), 주문 안내 이동, /book/ 링크 수정, Google Translation API 잔여물 정리, 캐러셀 씰 링 제거+크기 조정, 카드 이미지 구분선, 모달 학생/교사 표기 숨김+간격 조정, E-book "곧 출판 예정" warning 뱃지, 뱃지 크기 통일, 22개 로케일 i18n 50+키 | 2026-03-26 | — |
| 39 | 교재 주문 안내 카드 UI 개선 | 프론트 | 주문 안내 4열 그리드, 30부 할인 카드(BadgePercent), 카드별 색상(blue/emerald/violet/amber), "무료 배송" 문구, whitespace-pre-line | 2026-03-27 | 20개 locale 미반영 |
| 40 | E-book 뷰어 보안 강화 | 보안 | CORS `x-ebook-viewer`/`x-ebook-session` 허용, session_id 비교, Content-Disposition/Referrer-Policy/Cache-Control `no-store`, Rate Limit TOCTOU 수정(3곳), 마이크로도트 y분산, Heartbeat Canvas 클리어, print CSS 강화 | 2026-03-28 | Error Boundary 보류 |
| 41 | Gemini 코드 리뷰 반영 | 코드 품질 | 마이그레이션 문서 HHMMSS 예시 모순 수정, embla-carousel-react 미사용 패키지 제거, queryClient 불필요 의존성 제거, TiledPageCanvas Promise.allSettled 부분 렌더링 | 2026-03-28 | — |
| 42 | E-book 저작권 보호 고지 | 보안/법적 | 뷰어 최초 진입 시 저작권법 제104조의2 고지 모달 (ShieldCheck + sessionStorage), 22개 locale 번역 5키 | 2026-03-29 | — |
| 43 | 워터마크 진위확인 API | 보안 | `GET /admin/ebook/verify/{watermark_id}` — 관리자 전용, ebook_access_log + ebook_purchase JOIN 조회 | 2026-03-29 | — |
| 44 | 이미지 AES-256-GCM 암호화 저장 | 보안 | `encrypt_bytes`/`decrypt_bytes` + `EBOOK_IMAGES_ENCRYPTED` 피처 플래그, 페이지+타일 복호화 로직, 테스트 25개 | 2026-03-29 | CLI 암호화 도구 별도 |
| 45 | DevTools 감지 | 보안 | `devtools_detect.ts` 신규, 창 크기+console getter 2초 폴링, 3초 유예 블러, DevTools 닫으면 복원 | 2026-03-29 | — |
| 46 | 요청별 HMAC 서명 | 보안 | 세션별 32바이트 secret, Web Crypto HMAC-SHA256, ±30초 타임스탬프 윈도우, 상수시간 비교, 페이지/타일 요청 서명 검증 | 2026-03-29 | — |
| 47 | 홈/소개 페이지 리디자인 | 프론트 | Hero 한 줄 타이틀, 핵심가치 3카드, 기능 4카드(준비중 뱃지), 소개 차별점 3카드 상세, Stats 카드(2블록), "30,000" 삭제, 22개 locale 전면 교체, HeroSection 다국어 정렬 | 2026-03-30 | 다국어 반응형 디자인 규격 |
| 48 | 앱 로드맵 + 문서 정비 | 문서 | AMK_APP_ROADMAP.md 신규(Flutter+Tauri, 리스크 11건), STATUS 순서 재정렬, 6개 문서 참조 갱신, archive 4건 삭제, 메모리 정비 | 2026-03-30 | — |
| 49 | 디자인 시스템 v4 (V1-1~V1-8) | UI | 토큰 정리(dead code 삭제+container/motion-reduce), max-w-[1350px]→토큰, sections/→blocks/ 리네이밍, AuthLayout 추출(6 인증 페이지), CoverCard+FeatureGrid 블록, SectionContainer 확대(4파일), lazy loading 전수, 문서 동기화 | 2026-03-31 | — |
| 50 | 디자인 시스템 v4 (V1-5 DataTable) | UI | DataTable+useDataTable 블록 추출(`blocks/data_table.tsx`), 관리자 3페이지(users/lessons/videos) 적용, 검색+정렬+선택+페이지네이션 공통화, 각 ~200줄 감소 | 2026-03-31 | — |
| 51 | 디자인 시스템 v4 (V1-9~V1-10) | UI | 색상 토큰 교체(7파일): status badge→status-warning/success, neutral-900→surface-inverted, coming-soon 배지 토큰화, text-white→surface-inverted-foreground, 장식용 8건 의도적 유지, 문서 최종 동기화 | 2026-03-31 | — |
| 52 | 모바일 UX 79건 수정 | UI | `@media (pointer: coarse)` 터치 타겟 44px(45건), 고정 그리드 반응형(5건), 모달 뷰포트 제한(3건), 타이포그래피 가독성(7건), 패딩/간격 반응형(13건), Dialog 닫기·라이트박스·모달 네비 확대(4건), header 스크롤 잠금+햄버거 확대(2건). §04 Mobile Checklist 완료 | 2026-04-01 | — |
| 53 | 모바일 백엔드 API 5건 | 백엔드 | `POST /auth/google-mobile` (Google ID token 직접 검증), `POST /auth/apple-mobile` (Apple JWKS RS256, email 부재 처리), `POST /auth/mfa/login-mobile` (MFA JSON body), `POST /ebook/purchase/iap` (RevenueCat 영수증 검증, status=completed), `POST /payment/webhook/revenuecat` (Bearer 인증, 멱등성). OAuthUserInfo 일반화, `src/external/apple.rs`·`revenuecat.rs` 신규, DB enum 확장, 환경변수 5개 추가 | 2026-04-07 | 개발자 계정 등록 후 실 테스트 |
| 54 | 동시 세션 수 제한 | 보안 | 역할별 최대 세션 수 (HYMN:2, Admin:2, Manager:3, Learner:5). `enforce_session_limit()` — 유령 세션 정리(SMEMBERS+EXISTS) + SCARD 카운트 + 정책 분기. Learner: FIFO 자동 퇴장(가장 오래된 세션 강제 로그아웃). Admin/Manager/HYMN: 로그인 거부(403). `login()` + `create_oauth_session()` 2곳 적용. 환경변수 4개 | 2026-04-08 | — |
| 55 | 디자인 시스템 v4.2 보강 (전수 조사 19건) | UI/문서 | §00 Visual Theme & Atmosphere 신규, §01 Color Tokens HSL+Hex 병기 전면 재구성, §01 Shadow/Radius/Typography 정확값 명시, §03 CTA 패턴 3변형, §04 Responsive Behavior 신규, §05 Do/Don't 통합, §07-B Agent Prompt Guide 신규. 코드 수정 5건: index.css `--warning-foreground` WCAG AA, ebook_viewer 토큰 교체 15건, pagination_bar `rounded-md`, about_page CTA `hover:shadow-xl` | 2026-04-09 | — |
| 57 | 속도 개선 Phase S1+S2: 측정 인프라 + Quick Win 번들 분할 | 성능 | `frontend/perf-audit/` 신규 — Lighthouse 기반 자동 측정 (8페이지, Playwright Chromium 동적 탐색). `vite.config.ts` manualChunks — vendor 11종 분리. `routes.tsx` React.lazy — Admin 30+, Auth 보조 5, Legal 3, Textbook/Ebook 후속 5, Error 3, ComingSoon/FAQ 총 50+ 페이지 lazy 전환. **메인 번들 1,620KB → 271KB (-83.3%)**, home Performance 48→72(+24), TBT 2572→315ms(-88%). npm audit 3건 동시 패치(axios 1.15.0 + vite 7.3.2 + basic-ftp override). 코드 품질 13건 수정. 목표 Lighthouse 90+ (현재 48~78) | 2026-04-10 | Phase S3: faq/book-hub LCP 12~17s 조사 + Font preload + 이미지 최적화 + K6 백엔드 |
| 58 | 속도 개선 Phase S3: LCP 수정 + Font preload + 이미지 최적화 + K6 + 프로덕션 측정 | 성능 | **S3-1**: book-hub LCP 이미지 `loading="lazy"` → `eager` + `fetchPriority="high"` (초기 슬라이드만). **S3-2**: Pretendard CSS `preload` 힌트 추가, Noto Color Emoji 비동기 로딩 전환 (`media="print" onload`). **S3-3**: 로고 PNG 6000×4000 1.7MB → 1200×800 52KB(-97%), favicon 분리(32px+192px), 인증서 PNG→WebP(256KB→40KB, 100KB→28KB). **S3-4**: `k6/` 디렉터리 신규 — config.js(목표치), scenario_smoke.js(VU=1), scenario_load.js(10→100 VU ramp-up). K6 v0.56.0 설치. **S3-5**: 프로덕션 Lighthouse 베이스라인 — home Perf 52, about 34, faq 26(LCP 14.1s), book-hub 33(LCP 18.6s), login 35. 배포 후 재측정 필요 | 2026-04-12 | 배포 후 프로덕션 재측정으로 효과 확인 |
| 59 | 한글 자판 연습 Phase 1 — DB + 백엔드 + 관리자 CRUD + 세션 API + 프론트 타입/훅 + HangulKeyboard | 기능/풀스택 | 설계 플랜 P1~P10 중 **P1~P6 완료 (60%)**. **P6**: `frontend/src/category/study/component/writing/` 신규 — `keyboard_layout.ts` (2벌식 KS X 5002 DUBEOLSIK_ROWS + KeyCap 타입 + findKeyForJamo + decomposeSyllable (U+AC00~U+D7A3 초/중/종성 분해)), `HangulKeyboardKey.tsx` (개별 키 버튼, 기본/Shift/영문 3단 라벨, primary/amber ring highlight), `HangulKeyboard.tsx` (Props: highlightKeys/onKeyPress/visible/onToggle/level/disabled/className. 초급=항상 표시, 중급/고급=toggle 존중, 숨김 시 "키보드 보기" 버튼만 노출). i18n `study.writing.showKeyboard`/`hideKeyboard` 키 추가. **이하 P1~P5**: **P1**: 마이그레이션 `20260412_writing_practice.sql` — `study_task_kind_enum`에 `writing` 추가, 신규 enum 2종(writing_level: beginner/intermediate/advanced, writing_practice_type: jamo/syllable/word/sentence/paragraph), 테이블 2개 (`study_task_writing` 서브테이블, `writing_practice_session` 통계), 인덱스 7개. **P2**: `WritingPayload` DTO + `TaskPayload::Writing` + `SubmitAnswerReq::Writing{text,session_id}` + find_task_detail/find_answer_key SQL에 LEFT JOIN 추가. 초급 레벨에만 answer 응답 포함 (클라이언트 실시간 피드백). **P3**: 관리자 단일/벌크 CRUD 확장 (question/answer 재사용, writing_level/writing_practice_type/writing_hint/writing_keyboard_visible 신규 필드), create_task_writing repo 함수, update match Writing 분기. **P4**: 세션 API 4개 엔드포인트 — `POST /studies/writing/sessions` (시작), `PATCH /studies/writing/sessions/{id}` (완료, 서버 사이드 accuracy/CPM 계산), `GET /studies/writing/sessions` (목록, level/finished_only 필터 + 페이지네이션), `GET /studies/writing/stats` (days=1~365, total/avg_accuracy/avg_cpm + 레벨별 + 일별 추이 + 취약 글자 Top 10, jsonb_array_elements LATERAL JOIN). UPDATE WHERE user_id 소유권 검증. **P5**: 프론트 타입/API/훅 — `types.ts`에 Zod 스키마 11종 + discriminated union에 writing variant 추가, `writingPayloadSchema` 추가. `study_api.ts`에 세션 4개 함수 + sanitizeParams 제네릭화. `hook/use_writing_session.ts` 3개 훅(start/finish/list), `hook/use_writing_stats.ts` 1개 훅. `studyTaskKindSchema` 확장으로 기존 `Record<StudyTaskKind, ...>` 매핑 3곳 writing 엔트리 추가(PenLine 아이콘, i18n `study.kindWriting` 키). npm run build 클린 통과 | 2026-04-12 | P7~P10 프론트엔드 (연습 페이지 + study_task_page writing 분기, 통계 대시보드, 관리자 UI, 시드 데이터) |
| 56 | Figma Phase A 완료 + 상시 운영 도구로 전환 | UI/도구 | `frontend/figma-capture/` 신규 — Playwright 기반 레퍼런스 캡처 도구 일체. 1440×900 Retina viewport, Vite webServer 자동 기동, ko-KR 로캘. `document.fonts.ready` + 점진 스크롤 + img decoded 대기 + next-themes localStorage 주입으로 기존 34프레임의 3대 문제(한글/lazy/토큰) 근본 해결. 16 페이지 × Light/Dark = **32 PNG** 생성. textbook/ebook catalog API 모의 응답으로 백엔드 부재 시에도 카탈로그 렌더링 보장. **2026-04-10**: Figma Phase B/C 보류 결정 — Phase A 도구를 디자인 작업 상시 시각 레퍼런스 + 시각 회귀 감지 도구로 위치 재정의. 3계층 SSoT(AMK_DESIGN_SYSTEM.md + 32 PNG + 코드) 확정. 상세: `AMK_DESIGN_SYSTEM.md §08` | 2026-04-09 ~ 2026-04-10 | Phase B/C는 디자이너 영입/멀티 플랫폼 일관성 필요 시 재개 |

> **암호화 참고**: 대상 PII — `user_email`, `user_name`, `user_birthday`, `user_phone`, `oauth_email`, `oauth_subject`, `login_ip`, `admin_action_log.ip_address`
> **키 관리**: `ENCRYPTION_KEY_V{n}` (AES-256, 다중 버전) + `HMAC_KEY` (blind index), KeyRing 로드
> **다국어 참고**: 21개 언어 (아랍어 RTL 제외), Fallback: 사용자 언어 → en → ko, 공개 조건: `status = 'approved'`

### 8.2 진행 예정 항목

> **실행 순서 SSoT**: [`AMK_APP_ROADMAP.md §4`](./AMK_APP_ROADMAP.md) — 의존성 그래프, 크리티컬 패스, 병행 가능 구간 포함.

#### 핵심 (실행 순서)

| 순서 | 항목 | 카테고리 | 예상 | 내역 | 조건 |
|:----:|------|---------|:----:|------|------|
| 1 | **Paddle Live 전환** | 결제 | 1일 | GitHub Secrets 교체 + 배포 + E2E 검증 | 언제든 가능 |
| — | ~~e-book Paddle 연동~~ | — | — | — | **✅ 코드 구현 완료 (배포 대기)** |
| 2 | **교재 번역 + PDF 생성** | 콘텐츠 | 병행 | 22→34언어 확장 완료. 번역 33언어 검증 완료. **남은: 13언어 PDF 재생성 + 22언어 PDF 갱신** (`amazing-korean-books` 프로젝트) | 전 기간 병행 |
| 3 | **학습 콘텐츠 시딩** | 콘텐츠 | 2-3일 | 교재 JSON → DB 시딩, 실 콘텐츠 투입 | 해설용 출판본 완성 후 |
| 4 | **모바일 앱** | 앱 | ~21-23일 | **Flutter** (`amazing-korean-mobile` 별도 리포). 상세: [`AMK_APP_ROADMAP.md §2`](./AMK_APP_ROADMAP.md) | **지금 착수** |
| 5 | **데스크탑 앱** | 앱 | ~7.5일 | **Tauri 2.x** (`amazing-korean-desktop` 별도 리포). 상세: [`AMK_APP_ROADMAP.md §3`](./AMK_APP_ROADMAP.md) | **지금 착수** |
| 6 | **RDS/ElastiCache 이전** | 인프라 | 3-5일 | EC2 단일 DB → AWS RDS + ElastiCache | 앱 개발 이후 |
| — | ~~**동시 세션 수 제한**~~ | 보안 | ✅ | 역할별 동시 세션 상한 구현 완료 (HYMN:2/Admin:2/Manager:3/Learner:5). Learner FIFO 자동 퇴장, Admin+ 로그인 거부. 유령 세션 정리 포함 | — |
| — | ~~**모바일 OAuth 엔드포인트**~~ | 백엔드 | ✅ | `google-mobile` + `apple-mobile` + `mfa/login-mobile` 3건 구현 완료. `src/external/apple.rs` 신규. | — |
| — | ~~**IAP 결제 연동**~~ | 백엔드 | ✅ | DB enum 확장 + `POST /ebook/purchase/iap` + `POST /payment/webhook/revenuecat` 구현 완료. `src/external/revenuecat.rs` 신규. | — |
| — | ~~**모바일 인증 엔드포인트**~~ | 백엔드 | ✅ | `login-mobile` + `refresh-mobile` 구현 완료 | — |
| — | ~~**공유 Rust 크레이트 추출**~~ | 아키텍처 | ✅ | `amazing-korean-crypto` 크레이트 추출 완료 (Cargo 워크스페이���) | — |
| — | ~~**다국어 반응형 디자인 규격**~~ | UI | ✅ | 언어 그룹별 CSS 클래스 동적 관리, tracking-tight 조건부 해제, tall script line-height 보정, break-keep CJK 한정 | — |
| — | ~~**코드 점검**~~ | 품질 | ✅ | 점검 1~4 + 일괄 수정 + clippy 리팩토링 20건 완료. **clippy 0건** | — |
| — | ~~디자인 시스템~~ | — | — | — | ✅ §8.1 #13 |
| — | ~~E-book 웹 보안~~ | — | — | — | ✅ Phase 1 완료 (§8.1 #42~#46) |
| 10 | 다중 서버 구성 (HA) | 인프라 | — | ①nginx 복제 → ②ALB+EC2 → ③ECS Fargate | RDS 완료 후 |
| 11 | 시스템 모니터링 | 인프라 | — | DB/Redis 상태, Admin 대시보드 | 필요 시 |
| 12 | K6 성능 테스트 | 테스트 | `k6/` 디렉터리 세팅 완료 (smoke + load 시나리오) | 인증/조회/진도저장 부하 테스트, CI 연계 | 테스트 계정 생성 후 실행 |

**K6 성능 목표치 (엔드포인트별)**:

| 엔드포인트 | 목표 RPS | P95 응답시간 |
|----------|---------|-------------|
| 인증 (login/refresh) | 100 | < 200ms |
| 목록 조회 (videos/studies) | 200 | < 100ms |
| 상세 조회 | 300 | < 50ms |
| 진도 저장 (progress) | 100 | < 150ms |

**대표 시나리오**: 회원가입 → 로그인 → 비디오 조회 → 시청 → 진도 저장 → 학습 문제 풀이
| 7 | 마케팅/데이터 분석 | 기능 | 사용자 세그먼트, 리텐션 분석, 마케팅 자동화 | 데이터 기반 의사결정 | 사용자 확보 후 |

#### 검증된 리스크 (2026-03-31 코드베이스 팩트체크 완료)

| 작업 | 리스크 | 심각도 | 근거 |
|------|--------|:------:|------|
| Paddle Live | 12개 PADDLE_* Secret 일괄 교체 (누락 시 결제 실패) | CRITICAL | deploy.yml:87-98 |
| Paddle Live | Webhook Secret 1회성 (재확인 불가) | CRITICAL | AMK_DEPLOY_OPS.md:819 |
| Paddle Live | KYB/Onfido 인증 지연 가능 | HIGH | AMK_DEPLOY_OPS.md:781 |
| Paddle Live | SPF 레코드 병합 (Resend + Cloudflare) | MEDIUM | AMK_DEPLOY_OPS.md:857 |
| RDS 이전 | E-book 로컬 파일시스템 의존 (9곳 fs read) | CRITICAL | ebook/service.rs:51,261,502,516,525,605,620,629 + watermark.rs:13 |
| RDS 이전 | SSL 연결 필수 (현재 미사용) | HIGH | config.rs:97 (localhost 기본값) |
| RDS 이전 | ElastiCache AUTH 토큰 필요 (현재 인증 없음) | HIGH | config.rs:101 (redis://127.0.0.1:6379) |
| ~~동시 세션~~ | ~~제한 로직 미구현~~ ✅ 구현 완료 | — | enforce_session_limit() — SCARD + 유령 정리 + 역할별 정책 |
| ~~모바일 인증~~ | ~~login-mobile/refresh-mobile~~ ✅ 구현 완료 | — | auth/router.rs, handler.rs |
| ~~모바일 인증~~ | ~~X-Platform 헤더 검증~~ ✅ refresh-mobile에 적용 | — | auth/handler.rs:refresh_mobile |
| ~~Rust 크레이트~~ | ~~amazing-korean-crypto~~ ✅ 추출 완료 | — | crates/crypto/, Cargo.toml 워크스페이스 |
| Flutter | flutter_rust_bridge 버전 핀닝 필수 | HIGH | AMK_APP_ROADMAP.md R1 |
| Flutter | E-book 뷰어 메모리 OOM (14MB/페이지) | HIGH | AMK_APP_ROADMAP.md R7 |
| ~~Flutter~~ | ~~IAP receipt 검증 엔드포인트~~ ✅ 구현 완료 | — | POST /ebook/purchase/iap + POST /payment/webhook/revenuecat |
| Flutter | iOS isSecureTextEntry 비공식 API | MEDIUM | AMK_APP_ROADMAP.md R2 |
| Flutter | 앱 백그라운드 시 세션 만료 (TTL 90초) | MEDIUM | config.rs:325 |
| Tauri | macOS 캡처 방지 불가 (Apple 정책) | MEDIUM | AMK_APP_ROADMAP.md R5 (수용) |

> **팩트체크 방법**: 코드베이스 전수 grep + 파일별 라인 검증. 총 32개 주장 중 31개 확인, 1개 수정 (Secret 13→12개).

#### 보류/조건부

| # | 항목 | 카테고리 | 내역 | 예상 결과 | 조건/시점 |
|:-:|------|---------|------|----------|----------|
| 8 | ~~Apple OAuth~~ | 외부 API | Apple Sign In 구현 → **§8.2 핵심 순서 4a로 이동** | iOS 사용자 편의성 | 스펙 완료 (AMK_API_AUTH.md §5.3-17) |
| 9 | GeoIP 전환 | 인프라 | ip-api.com → MaxMind GeoLite2 로컬 DB | HTTPS, 무제한 쿼리 | 트래픽 증가 시 |
| 10 | step-up MFA | 보안 | 민감 작업 시 추가 인증 요구 | 결제/비밀번호 변경 시 보안 강화 | 필요 시 |
| 11 | ~~이메일 수신~~ | ~~외부 API~~ | ~~`support@amazingkorean.net` 수신~~ | ~~사용자 문의 처리~~ | ✅ Cloudflare Email Routing 설정 완료 (→ Gmail 포워딩) |
| 12 | 토큰 Redis 캐싱 | 보안 | 재발급 시 DB 조회 → Redis 캐시 | 동시 접속 성능 개선 | 동시접속 10K+ |
| 13 | enum sqlx::Type 전환 | 코드 품질 | 수동 match → `#[sqlx(type_name)]` derive | 보일러플레이트 감소 | 일괄 전환 시점 검토 |
| 14 | Keyset 페이징 | 기능 | page/size → keyset pagination | 대용량 테이블 성능 개선 | 데이터 1만 건+ |
| 15 | Lesson 통계 | 기능 | `/admin/lessons/stats` 구현 | 수업별 진행도 분석 | 필요 시 |
| 16 | 학습 문제 동적 생성 | 기능 | 커리큘럼 기반 문제 자동 생성/전달 | 학습 콘텐츠 확장 | 커리큘럼 완비 후 |
| 17 | 통계 비동기/배치 분리 | 인프라 | 집계 로직 비동기 처리 | API 응답 속도 개선 | 집계 복잡화 시 |
| 18 | OAuth 중복 통합 | 코드 품질 | auth repo/service 리팩토링 | 코드 중복 제거 | 세 번째 OAuth 추가 시 |
| 19 | manager 역할 구현 | 기능 | class 기반 접근 권한 부여 | 담당 학습자 범위 내 관리 | class 테이블 구현 후 |

#### 다국어 UI 참고 (21개 언어, LTR 전용)

| 항목 | 설명 |
|------|------|
| **폰트** | Noto Sans 패밀리 동적 로딩 (Latin/Cyrillic/CJK/Thai/Myanmar/Khmer/Sinhala/Devanagari) |
| **텍스트 길이** | 독일어 등 60%+ 길어질 수 있음 → 고정 폭 금지, flex/grid 사용, `text-overflow: ellipsis` |
| **줄 높이** | Thai/Myanmar/Khmer/Sinhala 결합 문자 → `line-height: 1.6~1.8` |

### 8.3 세부 검토 사항 — 한국어 발음 교정 AI (Pronunciation Coaching AI)

**현재 상태**: 조사 완료, 단계별 전략 확정 (2026-03-03)

**문제 정의**: 한국어 학습자의 발음 교정은 1:1 원어민 교사 없이는 사실상 불가능하다. **한 글자 단위 발음 교정이 초급 학습자에게 가장 중요**하며, 기존 서비스는 이를 지원하지 않는다.

**핵심 원칙**: 한국어는 비성조 언어 → 음높이 변화 거의 없음 → 한 글자씩 끊어서 발음 연습이 핵심.

#### 기존 API 비교 (조사 완료)

| API | 한 글자 | 음소별 점수 | 한국어 | 가격 | 평가 |
|-----|:-------:|:---------:|:------:|------|------|
| **SpeechSuper** | **지원** | **있음** (오발음 피드백 포함) | 4단계 (글자/단어/문장/단락) | $0.004/건 (단어) | **최적** |
| ETRI e-PreTX | 미지원 | 없음 (1~5점 단일 점수) | 문장만 | 무료 (1K/일) | 제한적 |
| Azure Speech | 미설계 | 있으나 음소명 미제공 (ko-KR) | 있음 | ~$1/시간 | 부적합 |

- **SpeechSuper**: 유일하게 한글 1자 음소별 평가 지원, REST + WebSocket, Rust SDK 제공
- **ETRI**: 2025.07 aiopen → e-PreTX 이전, PCM 전용, 자유 발화 시 전문가와 상관관계 없음 (Kim & Ko, 2022)

#### 3단계 전략 (확정)

**Phase 1** (현재): 따라하기 안내만 (녹음/판별 없음)

**Phase 2** (콘텐츠 완성 후): SpeechSuper API 프로토타이핑
- 한 글자 + 단어/문장 발음 평가, 음소별 점수 + 오발음 피드백
- Rust 백엔드 통합, 사용자 반응/데이터 수집
- 비용: $0.004/건 → 1,000명 활동 시 ~$1,680/월 (Growth $0.0028)

**Phase 3** (기술 검증 후): 커스텀 모델 개발
- **왜 한국어가 유리한가**: 음절 = 초성(19) + 중성(21) + 종성(28) 고정 구조, ~40개 음소 소규모 분류 문제, 성조 없음
- **베이스 모델**: `kresnik/wav2vec2-large-xlsr-korean` (Apache 2.0, WER 4.74%, CER 1.78%)
- **학습 데이터**: AIHub 71469 (1,030시간, 영어모국어 한국어 음성, 음소 시간 정렬 + 오류 태그) — HYMN 법인으로 접근 가능
- **아키텍처**: wav2vec2 파인튜닝 → 초성/중성/종성 3-way 분류 헤드 + GOP 점수화 (0~100)
- **학술 검증**: PER 10.25%, 전문가 일치 90% (2024, 1.56h 데이터), L1별 39개 오류 패턴 분류 (ICPhS 2023)
- **개발 비용**: GPU $200~$1,000, 추론 CPU 가능 (<100ms/음절, $50~200/월), 기간 2~3개월
- **장점**: API 비용 제거 + 데이터 주권 + L1별(20개 언어) 맞춤 피드백

#### L2 학습자 공통 오류 패턴 (ICPhS 2023)

1. 격음/경음 → 평음 대치 (ㅋ/ㄲ→ㄱ, ㅌ/ㄸ→ㄷ, ㅍ/ㅃ→ㅂ, ㅊ/ㅉ→ㅈ, ㅆ→ㅅ)
2. 종성(받침) 탈락
3. 이중모음 → 단모음 대치
4. L1별 고유 패턴 (베트남어: /l/→/n/ 종성, 일본어: /ŋ/ 삽입 등)

#### 조음 애니메이션 (Phase 15 연동)

- **15~17개 다이어그램**으로 한국어 전체 음소 커버 (자음 7 조음 위치 + 성문 1 + 모음 7~9)
- 평음/경음/격음은 같은 입 위치 → 성문 상태도로 차이 표현
- **Wikimedia CC0 SVG** 활용 (IPA 조음 단면도, 퍼블릭 도메인) → 자체 제작 최소화
- 기술: Figma → SVG → GSAP MorphSVG (무료) + React
- **한국어 전용 조음 애니메이션 도구 부재 → 차별화 기회** (경쟁사 분석 완료)

#### 맞춤형 학습 AI 확장 로드맵

```
데이터 플라이휠:
사용자 발음 녹음 → 모델 평가 → 피드백/재시도 → 데이터 축적 → 모델 재학습 → 정확도 향상 → 사용자 증가 → ...
```

1. **발음 평가 AI** (Phase 14) — 음소별 정확도 판정
2. **학습자 프로파일링** — L1 기반 취약 음소 자동 파악 + 개인별 반복 오류 패턴 추적
3. **맞춤형 학습 경로** — 취약 부분 자동 반복 출제, 조음 영상 재안내
4. **20개 언어별 특화 모델** — L1 간섭 오류 집중

**핵심 차별점**: 교재(콘텐츠) + AI(개인화) 조합 → 데이터가 쌓일수록 격차 벌어짐 → 진입 장벽

#### 사용 가능한 데이터셋

| 데이터셋 | 규모 | 음소 라벨 | 오류 태그 | 접근 |
|----------|------|----------|----------|------|
| **AIHub 교육용 영어모국어 한국어 음성 (71469)** | **1,030시간** | **있음 (시간축 정렬)** | **있음 (TextGrid)** | 한국 국적 |
| AIHub 외국인 한국어 발화 (505) | 4,302시간 | 없음 (단어 단위) | 없음 | 한국 국적 |
| KsponSpeech (한국어 자연발화) | 1,000시간 | 없음 | 없음 | 한국 국적 |
| Zeroth-Korean | ~50시간 | 없음 | 없음 | **오픈** |

#### 오픈소스 선례

| 프로젝트 | 아키텍처 | 특징 |
|----------|---------|------|
| DevTae/SpeechFeedback | Deep Speech 2 (CNN+BiGRU+CTC) | Docker+FastAPI, 한글→44 IPA 변환, AIHub 60만 샘플 |
| kresnik/wav2vec2-large-xlsr-korean | wav2vec2 XLSR Large (0.3B) | WER 4.74%, CER 1.78%, Apache 2.0 |
| ai-pronunciation-trainer | Whisper + Epitran + 규칙기반 | 영어/독일어만, 한국어 적용 시 커스텀 필요 |

#### 참고 링크

- SpeechSuper 한국어 데모: speechsuper.com/demo/korean/
- SpeechSuper 가격: speechsuper.com/pricing.html
- SpeechSuper API 샘플 (Rust 포함): github.com/speechsuper/SpeechSuper-API-Samples
- AIHub 교육용 한국어 음성: aihub.or.kr/aihubdata/data/view.do?dataSetSn=71469
- ETRI e-PreTX: epretx.etri.re.kr
- 2025 논문 (Whisper 파인튜닝): eksss.org/archive/view_article?pid=pss-17-1-51
- 2024 논문 (아동 발음장애): arxiv.org/html/2403.08187v1
- ICPhS 2023 (L2 오류): arxiv.org/abs/2306.10821

### 8.4 상시 모니터링 항목

프로젝트 전반에 걸쳐 지속적으로 수행해야 하는 조사·분석·모니터링 활동.

| # | 항목 | 분류 | 내역 | 주기 | 참고 |
|:-:|------|------|------|:----:|------|
| 1 | 한국어 교육 시장 조사 | 시장 | 경쟁사 동향 (신규 앱, 가격 변동, 기능 출시), TOPIK 응시자/학습자 통계, 시장 규모 업데이트 | 월 1회 | [`AMK_MARKET_ANALYSIS.md`](./AMK_MARKET_ANALYSIS.md) |
| 2 | 교육 앱 UX/UI 트렌드 | 시장 | 주요 교육 앱 UI 변화, 온보딩 플로우, 게이미피케이션 패턴, 접근성 트렌드 | 월 1회 | — |
| 3 | 결제/수익 모델 동향 | 시장 | Apple/Google IAP 정책 변경, 수수료율 변동, 지역별 가격 전략, 프로모션 사례 | 분기 1회 | [`AMK_MARKET_ANALYSIS.md §4`](./AMK_MARKET_ANALYSIS.md#4-모바일-앱-결제-전략) |
| 4 | AI/ML 기술 동향 | 기술 | LLM 경량화 (BitNet 후속), 음성인식 (Whisper 후속), 온디바이스 AI SDK, 발음 평가 API | 월 1회 | [`AMK_PIPELINE.md §11`](./AMK_PIPELINE.md) |
| 5 | 모바일 프레임워크 동향 | 기술 | Flutter / Tauri / Kotlin Multiplatform 변화, flutter_rust_bridge 업데이트, 크로스플랫폼 보안 사례 | 분기 1회 | [`AMK_APP_ROADMAP.md`](./AMK_APP_ROADMAP.md) |
| 6 | 인프라/보안 동향 | 기술 | AWS 신규 서비스, 컨테이너 오케스트레이션, 인증 표준 (Passkey 등), OWASP 업데이트 | 분기 1회 | [`AMK_DEPLOY_OPS.md`](./AMK_DEPLOY_OPS.md) |
| 7 | 규제/법률 동향 | 사업 | 교육 앱 개인정보보호 (COPPA, GDPR-K), DMA/DSA 후속 조치, 각국 앱스토어 규제 | 분기 1회 | — |

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

### 8.5 Paddle Live 전환 체크리스트

Sandbox → Live(프로덕션) 전환. 코드 구현 완료, Dashboard 설정 대부분 완료.

> **참고 문서**: [Go-live checklist (Paddle Developer)](https://developer.paddle.com/build/onboarding/go-live-checklist)

#### 완료 항목

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | 계정 인증 (KYB + Onfido) | ✅ |
| 2 | 도메인 인증 (`amazingkorean.net`) | ✅ |
| 3 | Products 생성 (구독 + E-book) | ✅ |
| 4 | Prices 생성 (1/3/6/12개월 정가 + E-book) | ✅ |
| 5 | API Key 발급 (`pdl_apikey_live_...`) | ✅ |
| 6 | Client Token 발급 (`live_...`) | ✅ |
| 7 | Webhook Destination (11개 이벤트, Secret Key 확보) | ✅ |
| 8 | Payment Methods (Card + PayPal/Apple Pay/Google Pay) | ✅ |
| 9 | Balance Currency (USD) + Sales Tax Settings | ✅ |
| 10 | Default Payment Link (`https://amazingkorean.net/pricing`) | ✅ |
| 11 | Retain (Payment Recovery, 이메일 브랜딩, Postmark 인증) | ✅ |
| 12 | pwCustomer 코드 (`use_paddle.ts` + 3개 호출 사이트) | ✅ |
| 13 | Retain 홈페이지 Paddle.js 초기화 (`home_page.tsx`) | ✅ |
| 14 | Cloudflare Email Routing (`support@amazingkorean.net` → Gmail) | ✅ |
| 15 | 환경변수 정리 (`PADDLE_PRODUCT_ID` 제거, `PADDLE_PRICE_EBOOK` 통일) | ✅ |
| 16 | Paddle Discount 3개 생성 (3/6/12개월 flat $5/$10/$20 off) | ✅ |
| 17 | Discount 코드 적용 (`config.rs`, `dto.rs`, `service.rs`, `use_paddle.ts`, `pricing_page.tsx`) | ✅ |
| 18 | GitHub Secrets Discount ID 3개 등록 | ✅ |

#### 남은 작업

##### ~~Step 1: Paddle Discount 생성 — 유저 (Dashboard)~~ ✅

##### ~~Step 2: 코드 수정 — Claude~~ ✅

Paddle Dashboard → **Catalog → Discounts** 에서 3개 생성:

| Discount | Type | Amount | Recur | Restrict To | 최종가 |
|----------|------|--------|-------|-------------|--------|
| 3개월 할인 | Flat | $5 off | Yes (영구) | 3개월 Price ID만 | $30 → $25 |
| 6개월 할인 | Flat | $10 off | Yes (영구) | 6개월 Price ID만 | $60 → $50 |
| 12개월 할인 | Flat | $20 off | Yes (영구) | 12개월 Price ID만 | $120 → $100 |

- Code: 비워두기 (코드 없이 생성 → `discountId`로 자동 적용)
- 생성 후 Discount ID (`dsc_...`) 3개 확보

##### Step 2: 코드 수정 — Claude

| 파일 | 변경 내용 |
|------|----------|
| `src/types.rs` | `price_cents()` 정가로 수정: 3개월 $30(3000), 6개월 $60(6000), 12개월 $120(12000) |
| `src/config.rs` | Discount ID 환경변수 3개 추가 (`PADDLE_DISCOUNT_MONTH_3/6/12`) |
| `docker-compose.prod.yml` | Discount ID 환경변수 3개 추가 |
| `.github/workflows/deploy.yml` | Discount ID Secrets 3개 추가 |
| `.env.example` | Discount ID 환경변수 3개 추가 |
| `src/api/payment/service.rs` | Plans API 응답에 `discount_id` 포함 |
| `frontend/src/category/payment/` | checkout 시 `discountId` 자동 적용 |

##### Step 3: GitHub Secrets 업데이트 — 유저

| Secret | 값 |
|--------|-----|
| `PADDLE_SANDBOX` | `false` |
| `PADDLE_API_KEY` | Live API Key (`pdl_apikey_live_...`) |
| `PADDLE_CLIENT_TOKEN` | Live Client Token (`live_...`) |
| `PADDLE_WEBHOOK_SECRET` | Live Webhook Secret (`pdl_ntfset_...`) |
| `PADDLE_PRICE_MONTH_1` | Live Price ID ($10) |
| `PADDLE_PRICE_MONTH_3` | Live Price ID ($30, 정가) |
| `PADDLE_PRICE_MONTH_6` | Live Price ID ($60, 정가) |
| `PADDLE_PRICE_MONTH_12` | Live Price ID ($120, 정가) |
| `PADDLE_PRICE_EBOOK` | Live Price ID ($10) |
| `PADDLE_DISCOUNT_MONTH_3` | Discount ID (`dsc_...`) |
| `PADDLE_DISCOUNT_MONTH_6` | Discount ID (`dsc_...`) |
| `PADDLE_DISCOUNT_MONTH_12` | Discount ID (`dsc_...`) |
| `PAYMENT_PROVIDER` | `paddle` |

##### Step 4: 배포 — 유저

1. 커밋 → push → GitHub Actions 자동 배포
2. 서버 로그 확인: `💳 Payment provider enabled: Paddle Billing (Production)`

##### Step 5: E2E 검증 — 유저

| # | 검증 항목 | 방법 | 기대 결과 |
|:-:|----------|------|----------|
| 1 | API Health | `curl https://api.amazingkorean.net/health` | 200 OK |
| 2 | Plans API | `curl https://api.amazingkorean.net/payment/plans` | `sandbox: false`, Live Price ID |
| 3 | E-book Catalog | `curl https://api.amazingkorean.net/ebook/catalog` | `sandbox: false`, Live Price ID |
| 4 | Webhook Simulator | Dashboard → Notifications → Webhook Simulator | 200 OK |
| 5 | 구독 실결제 | `/pricing` → 1개월 $10 → 카드 결제 | DB subscription + transaction 생성 |
| 6 | 구독 Discount | `/pricing` → 3개월 → 체크아웃에서 ~~$30~~ $25 표시 | Discount 자동 적용 |
| 7 | 구독 환불 | Dashboard에서 환불 | `adjustment.created` → transaction `refunded` |
| 8 | E-book 실결제 | `/ebook` → Paddle overlay | `ebook_purchase` 상태 `completed` |
| 9 | E-book 환불 | Dashboard에서 환불 | `ebook_purchase` 상태 `refunded` |
| 10 | Retain URL 검증 | Paddle Retain → Check URL `https://amazingkorean.net` | Paddle.js 감지 |
| 11 | 프론트 UX | `/pricing`, `/ebook`, `/ebook/my` 페이지 | 정상 로딩 + checkout 동작 |

> ⚠️ Live 모드 = 실제 결제. 테스트 후 반드시 Dashboard에서 환불 처리.

##### Step 6: 은행 — 유저

- 하나은행 세종중앙금융센터(044-867-1111)에 USD 계좌 영문 예금주명 등록 요청
- 등록 후 Paddle Dashboard → Payout Settings → Account Holder Name 입력

#### 가격 구조

| 구간 | 정가 (Paddle Price) | Discount | 최종가 (고객 결제) |
|------|--------------------|---------|--------------------|
| 1개월 | $10 | — | $10 |
| 3개월 | $30 | $5 off | $25 |
| 6개월 | $60 | $10 off | $50 |
| 12개월 | $120 | $20 off | $100 |
| E-book | $10 | — | $10 |

#### 결제수단 / 세금 참고

**결제수단**: Paddle 기본 수수료(5% + $0.50/건)에 **모든 결제수단 포함** (추가 비용 없음)
- 카드(Visa/MC/Amex), PayPal, Apple Pay, Google Pay, KakaoPay, NaverPay, Samsung Pay, Alipay, UnionPay, iDEAL 등
- 고객 위치/기기/통화에 따라 Paddle이 자동으로 적절한 결제수단 표시

**세금**: Paddle MoR(Merchant of Record)가 100개국+ **자동 처리**
- VAT (EU/UK), GST (호주/인도), Sales Tax (미국 주별) — 자동 계산 + 징수 + 납부
- 세금 관련 법적 책임도 Paddle에 귀속
- 설정: "Prices include tax" OFF 권장 (표시 가격 + 세금 별도)

[⬆️ 목차로 돌아가기](#-목차-table-of-contents)

---

### 8.6 학습 콘텐츠 보안 전략 (Content Protection)

> 자체 플랫폼(웹/앱)의 학습 콘텐츠와 외부 유통 EPUB3에 대한 7중 보안 아키텍처.

#### 유통 채널별 전략

| 채널 | 형식 | 보안 방식 |
|------|------|----------|
| **자체 웹/앱** | 인터랙티브 학습 플랫폼 (DB → API → 렌더링) | 구조적 보안 + 포렌식 마킹 + 플랫폼 보안 |
| **외부 플랫폼** (Amazon/Apple/Google/Kobo) | EPUB3 | 플랫폼 DRM 자동 적용 |

- 자체 사이트에서는 EPUB3 파일을 제공하지 않음 — **다운로드 가능한 파일 자체가 존재하지 않음**
- 교재 콘텐츠(JSON 데이터)를 서버에서 인터랙티브 학습 콘텐츠로 동적 렌더링
- EPUB3는 외부 플랫폼 유통 전용으로만 생성

#### Layer 1 — 구조적 보안 (콘텐츠 제공 방식 자체가 방어)

| 방어 요소 | 설명 |
|----------|------|
| 파일 없음 | 다운로드할 파일이 존재하지 않음 (EPUB3/PDF 미제공) |
| 서버 렌더링 | 콘텐츠가 DB → API → 조각 단위 전송, 전체 데이터 한 번에 노출 안 됨 |
| 페이지/단원 단위 접근 | 요청한 부분만 내려줌, 전체 책을 한 번에 수집 불가 |
| 인증 + 구독 확인 | 로그인 + 활성 구독 상태에서만 접근 가능 |
| 데이터 분해 | 어휘/문장/활용/조사가 개별 API 응답으로 분리 — 크롤링해도 원본 교재 형태로 재조립 극히 어려움 |

#### Layer 2 — 포렌식 워터마킹 (유출 시 추적, 실시간 동적 생성)

API 응답마다 구매자별 고유 마킹 패턴을 **실시간 동적 생성** — 세션마다 패턴이 바뀌어 패턴 분석 자체가 불가능.

| 층 | 기법 | 추적 대상 | 제거 난이도 | 비고 |
|:--:|------|----------|:---------:|------|
| 1 | **ZWC (Zero-Width Character) 삽입** | 텍스트 복사 | 쉬움 | 유니코드 영폭 문자로 구매자 ID 인코딩 |
| 2 | **동형 문자(Homoglyph) 치환** | ZWC 제거 후에도 생존 | 중간 | 육안 구분 불가 유니코드 문자 교체 |
| 3 | **이미지 LSB 삽입** | 이미지 캡처 | 어려움 | 픽셀 최하위 비트에 데이터 삽입 |
| 4 | **CSS 미세 변형** | 화면 캡처 | 어려움 | letter-spacing/color 0.01px/0.01% 단위 차이 |

- 인터랙티브 플랫폼 장점: EPUB3는 구매 시 1회 마킹이지만, 플랫폼에서는 **매 API 요청마다 패턴 변경 가능**
- 마킹 정보: 구매자 ID + 타임스탬프 + IP + 기기 + 세션 ID
- 4개 레이어 중 **하나만 살아남아도 법적 추적 근거 확보**
- "마킹이 되어있다"는 사실 자체가 심리적 유출 억제 효과

#### Layer 3 — 플랫폼 보안 (기존 인프라 활용)

| 환경 | 보안 조치 |
|------|----------|
| **웹** | 우클릭 차단, 텍스트 선택 차단 (`user-select: none`), 개발자도구 감지, 인쇄 차단 |
| **앱 (iOS/Android)** | 스크린샷/화면 녹화 차단 API, 암호화 캐시, 오프라인 읽기 시 암호화 유지 |
| **외부 플랫폼** | Amazon/Apple/Google/Kobo 자체 DRM 자동 적용 (별도 설정 불필요) |

#### 보안 한계 인식

- **어떤 DRM도 100% 완벽하지 않음** — 넷플릭스도 뚫림
- 핵심은 "완벽한 차단"이 아니라 **"비용 대비 효과"** — 대다수에게 충분히 어렵게 만드는 것
- 3중 방어 구조에서 콘텐츠를 완전히 탈취하려면: 인증 돌파 → 조각 데이터 전수 크롤링 → 원본 재조립 → 4중 마킹 전부 제거 필요
- ₩25,000 교재를 이 수준으로 뚫으려는 동기 대비 방어 비용이 충분히 합리적

#### 미해결 이슈

| # | 위치 | 내용 | 심각도 |
|:-:|------|------|:------:|
| ~~1~~ | ~~`src/api/admin/user/repo.rs:453`~~ | ~~`admin_get_user_logs()` 복호화 미적용~~ — **확인 결과 service.rs:822-826에서 복호화 구현 완료됨** (2026-03-30 검증). DTO `Option<String>`으로 LEFT JOIN NULL도 안전 처리 | ~~Medium~~ **해결됨** |

> DB 암호화 Phase 2 계획(Bug 1~8) — **전체 완료** 확인 (2026-03-30 재검증). 8개 Bug + Sub-Phase 2B~2D 모두 구현 완료.

#### 학습 콘텐츠 개선 방안

> 교재(JSON) 데이터를 웹 학습 콘텐츠로 변환하는 설계. 교재 페이지 순서 = 학습 순서.

##### 핵심 원칙

- 교재(JSON)의 내용이 곧 웹 학습 콘텐츠의 원본 데이터
- 교재 페이지 순서 = 학습 순서 (`page_manifest.json` 기준)
- 동영상/음성은 현재 미보유 → 추후 제작
- 기존 course/lesson/study DB 데이터는 테스트 데이터 → 삭제 가능

##### 교재 JSON 데이터 → DB 매핑

**데이터 소스** (`scripts/textbook/data/`)

| JSON 파일 | 내용 | 항목 수 | Study Program |
|-----------|------|---------|---------------|
| vocabulary.json | 어휘 카드 (한국어 + 20개 언어 번역) | 280+ | basic_word |
| sentences.json | 문법 예문 (한국어 + 번역) | 496+ | basic_900 |
| pronunciation.json | 한글 조합표 (자음×모음) | 테이블 7+ | basic_pronunciation |
| pronunciation_test.json | 발음 테스트 | 연습 문제 | basic_pronunciation |
| particles.json | 조사 활용표 | 테이블 | basic_900 |
| conjugation.json | 동사/형용사 활용 (현재/과거/미래) | 테이블 | basic_900 |
| structure.json | 문장 구조 (의문사 패턴) | 테이블 | basic_900 |
| appendix.json | 숫자, 문법 연습 | 테이블 | basic_900 |

**DB 계층 구조**

```
Course "놀라운 한국어 기초"
├── Part I. 발음 (Pronunciation)
│   ├── Lesson 1~7: pronunciation.json + pronunciation_test.json + vocabulary.json(발음)
├── Part II. 문법 기초 (Grammar Basics)
│   ├── Lesson 8~10: particles.json + structure.json
├── Part III~IV. 서술어/부사어 문법
│   ├── Lesson 11~30: sentences.json (섹션별 1 Lesson)
├── Part V. 동사 활용
│   ├── Lesson 31~33: conjugation.json
└── Part VI. 부록
    ├── Lesson 34~35: appendix.json
```

##### Lesson 구조 (설명 + 영상 + 문제)

```
Lesson = [설명 콘텐츠] + [Video (있으면)] + [Study Tasks]

- 설명: lesson_description에 해당 단원의 개념 설명
- Video: 강사 영상 (있을 경우 lesson_item kind=video)
- Study: lesson_item kind=task로 연결된 문제들
- Explain: study_task_explain에 문법 테이블/해설 저장
```

**Lesson 예시 (모음 1)**

```
Lesson "모음 1 (Vowel 1)"
├── [설명] "한국어에는 10개의 기본 모음이 있습니다..."
├── [Video] 입모양 발음 시범 영상 (추후)
├── [Study: choice × 5] ㄱ+ㅏ=? → [가,나,다,라]
├── [Study: typing × 3] 직접 타이핑 또는 클릭 순서 배열
└── [Study explain] 자음+모음 조합표 (pronunciation.json)
```

##### 문제 유형 (Study Task Kinds)

현재 DB 지원: choice, typing, voice

| Kind | 방식 | 교재 데이터 활용 |
|------|------|----------------|
| **choice** (4지선다) | 보기 중 정답 선택 | vocab → 뜻 맞추기, sentence → 번역 맞추기, pronunciation → 조합 맞추기 |
| **typing** (타이핑) | 직접 입력 | vocab → 한국어 쓰기, conjugation → 활용형 쓰기 |
| **typing** (클릭배열) | 형태소/블록 순서 배열 | sentence → 어순 배열, particles → 조사 선택 배열 |

**클릭 순서 배열형** (typing 확장 또는 신규 kind 'ordering')
- 문제: "나는 행복합니다"를 올바른 어순으로 배열하세요
- 보기: [행복합니다] [나는] ← 셔플된 블록
- 정답: [나는] [행복합니다]
- 기존 typing kind를 확장하거나 ordering kind 신규 추가 검토

**오답 생성 전략**
- **vocab choice**: 같은 카테고리(같은 페이지/챕터) 내 다른 어휘에서 랜덤 추출
- **sentence choice**: 같은 문법 섹션 내 다른 예문 번역에서 추출
- **pronunciation choice**: 유사 자모 조합에서 추출 (가/나/다/라)
- **particles choice**: 다른 조사를 오답으로 (는/를/에/로)

##### 영상 제작 전략 (2026-03-03 확정)

**핵심 방향: AI 기술 적극 도입**
- 강사 직접 촬영이 아닌 **AI 음성 + 애니메이션** 중심
- 목표: 일관된 퀄리티, 대량 생산 가능, 정확한 발음 전달

**한국어 발음 교육의 핵심 원칙**
- **성조 없음**: 음높이 변화 거의 없음 → 음율적이지 않게 한 글자씩 발음
- **한 글자씩**: 학습 시 반드시 한 글자 단위로 끊어서 발음 연습
- **남성 + 여성 음성**: 모든 발음을 남녀 AI 음성 2가지로 제공
- **입모양/혀 위치**: 실사 촬영 대신 **애니메이션**으로 시각화 (더 명확)
- **두 글자 이상**: 목차별 순차적 듣기 + 따라하기 연습

**파트별 영상 구성**

Part I. 발음 (1순위 — 텍스트 대체 불가)

| 요소 | 내용 |
|------|------|
| **AI 음성** | 한 글자씩 남성/여성 TTS, 성조 없이 평탄하게 |
| **애니메이션** | 입모양 + 혀 위치 다이어그램 (자음/모음별) |
| **자막** | 한글 + 발음기호 + 학습자 모국어 (20개 언어) |
| **흐름** | 글자 표시 → 애니메이션 → 남성 발음 → 여성 발음 → 따라하기 |
| **조합 연습** | 자음+모음 조합표 한 줄씩 순차 재생 + 듣기 |

Part II. 문법 기초

| 요소 | 내용 |
|------|------|
| **시제** | 핵심 동사 기준 → 현재/과거/미래 × 기본형/의문형, AI 발음 시범 |
| **조사** | 핵심 명사 기준 → 각 조사별 발음 시범 |
| **문장구조/의문사** | 동일 패턴: 텍스트 + AI 발음 |

Part III~VI. 문장/서술어/부사어/기타 문법

| 요소 | 내용 |
|------|------|
| **공통** | 각 목차의 문법/형태를 AI 발음으로 보여줌 |
| **예문** | 10개 예문 순차 재생 (남/여 음성) + 모국어 자막 |

**학습 흐름: 영상 → Study 복습 (확정)**

```
[영상] 전체 흐름을 한 번 봄 (수동적, 개요 파악)
  ↓
[Study] 자기 페이스로 연습 + 복습 (능동적, 반복)
  ↓
[이후 복습] Study만 반복 (영상 재시청은 선택적)
```

- 영상 = 설명/시범, Study = 연습/평가 — 역할 분리
- Study에서 audio_url로 개별 음성 재생 가능 → 인터랙티브 장점 흡수
- 인터랙티브 웹 콘텐츠 별도 구현 불필요 (기존 video + study 구조 유지)

**발음 평가 (음성 인식) — 단계적 도입**

| Phase | 범위 | 접근 방식 | 시기 |
|-------|------|----------|------|
| **Phase 1** | 따라하기 안내만 (녹음/판별 없음) | 없음 | 지금 |
| **Phase 2** | **한 글자 + 단어/문장** 발음 평가 | SpeechSuper API ($0.004/건) | 콘텐츠 완성 후 |
| **Phase 3** | 커스텀 AI로 전환 | wav2vec2 파인튜닝 + 초성/중성/종성 분류 | 기술 검증 후 |

- **한 글자 발음 교정이 초급 학습자에게 가장 중요** (핵심 원칙)
- Phase 2: SpeechSuper가 유일하게 한글 1자 음소별 평가 지원 → 프로토타이핑 + 사용자 검증
- Phase 3: AIHub 71469 데이터(1,030h, 음소 라벨+오류 태그) + wav2vec2-xlsr-korean 파인튜닝
- 커스텀 모델 장점: API 비용 제거, 데이터 주권, L1별 맞춤 피드백 (20개 언어)
- **상세 조사**: [pronunciation_ai_research.md](pronunciation_ai_research.md) (메모리 토픽 파일)

**입모양/혀 애니메이션 제작 방안 (조사 완료)**
- **15~17개 다이어그램**으로 전체 커버 (자음 7위치 + 성문 1 + 모음 7~9)
- 평음/경음/격음은 같은 입 위치 → 성문 상태도로 차이 표현
- **Wikimedia CC0 SVG** 활용 (IPA 조음 단면도, 퍼블릭 도메인) → 자체 제작 최소화
- 자체 제작 필요: ㅈ/ㅊ/ㅉ 치경구개음 SVG만
- **기술**: Figma → SVG path → GSAP MorphSVG (무료) + React
- **기간**: 3~6주 (Level 2: SVG 모핑)
- **한국어 전용 조음 애니메이션 도구 부재 → 차별화 기회**
- **상세 조사**: [AMK_PIPELINE.md §12.1 조음 애니메이션 조사](./AMK_PIPELINE.md#121-조음-애니메이션-조사-2026-03-03)

**AI 기술 스택**
- **TTS**: 한국어 고품질 AI 음성 (남/여) — Google Cloud TTS, CLOVA, OpenAI TTS 등 (조사 필요)
- **발음 평가**: Phase 2 SpeechSuper API → Phase 3 커스텀 wav2vec2 모델 (조사 완료, [상세](pronunciation_ai_research.md))
- **입모양 애니메이션**: CC0 SVG + Figma → GSAP MorphSVG + React (조사 완료, [상세](./AMK_PIPELINE.md#121-조음-애니메이션-조사-2026-03-03))
- **영상 자동 생성**: JSON 데이터 → 스크립트 → 영상 자동 렌더링 파이프라인
- **자막**: 교재 번역 데이터(20개 언어)에서 SRT/VTT 자동 생성

**다국어 자막 전략**
- 음성: 한국어 AI TTS (남/여)
- 자막: 교재 JSON translations에서 자동 추출 → 20개 언어
- 결과: 영상 템플릿 1개 → 자막만 교체로 20개 언어 커버

##### 구현 접근법: Seed Script

```
scripts/textbook/JSON → seed_script → DB (study + study_task + lesson + course)
```

1. JSON 파일 읽기 (vocabulary, sentences, pronunciation, ...)
2. Study 세트 생성 (program별: basic_pronunciation, basic_word, basic_900)
3. StudyTask 생성 (choice/typing 문제 자동 생성 + explain 해설)
4. Lesson 생성 + lesson_description에 단원 설명 + LessonItem에 task 연결
5. Course 생성 + course_lesson으로 Lesson 연결

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
