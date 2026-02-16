# QA Report — SEO + Paddle Webhook (Phase 2)

- **테스트 일시**: 2026-02-16
- **테스트 환경**: localhost:3000 (백엔드) + localhost:5173 (프론트엔드 dev)
- **PADDLE_WEBHOOK_SECRET**: 미설정 (Sandbox 연동 전)

---

## 요약

| 카테고리 | 항목 수 | PASS | FAIL | 검증 방법 |
|----------|---------|------|------|----------|
| A-1. 법적 페이지 코드 리뷰 | 6 | 6 | 0 | 파일 리뷰 |
| A-2. SEO 정적 파일 | 3 | 3 | 0 | HTTP 요청 |
| A-3. HTML 메타 태그 | 11 | 11 | 0 | HTTP 요청 |
| A-4. 프론트엔드 라우팅 | 6 | 6 | 0 | HTTP + 코드 리뷰 |
| A-5. i18n 번역 키 | 4 | 4 | 0 | JSON 비교 |
| B-1. Webhook 코드 리뷰 | 4 | 4 | 0 | 파일 리뷰 |
| B-2. Webhook API 테스트 | 3 | 3 | 0 | HTTP 요청 |
| 빌드 검증 | 2 | 2 | 0 | CLI |
| **합계** | **39** | **39** | **0** | |

**최종 결과**: 39/39 PASS — 이슈 없음

---

## A. SEO 수정 (프론트엔드)

### A-1. 법적 페이지 코드 리뷰

| 파일 | 확인 사항 | 결과 |
|------|----------|------|
| `legal_page.tsx` | 공통 레이아웃 (i18n 기반 title/intro/sections/contact) | **PASS** — ArrowLeft 뒤로가기, whitespace-pre-line, 이메일 링크 |
| `terms_page.tsx` | 7개 섹션 (`legal.terms.s1~s7`) | **PASS** |
| `privacy_page.tsx` | 7개 섹션 (`legal.privacy.s1~s7`) | **PASS** |
| `refund_policy_page.tsx` | 5개 섹션 (`legal.refund.s1~s5`) | **PASS** |
| `faq_page.tsx` | 8개 섹션 (`legal.faq.s1~s8`) | **PASS** |
| `footer.tsx` | Support 섹션에 FAQ/이용약관/개인정보/환불정책 링크 | **PASS** — /faq, /terms, /privacy, /refund-policy |

### A-2. SEO 정적 파일

| ID | 테스트 | 결과 |
|----|--------|------|
| SEO-1 | `GET /robots.txt` → 200 + 크롤러 규칙 정상 | **PASS** — Allow /, Disallow /admin/ /user/ + 인증/유틸 페이지, Sitemap URL |
| SEO-2 | `GET /sitemap.xml` → 200 + 유효한 XML (11개 URL) | **PASS** — /, /about, /videos, /studies, /lessons, /signup, /login, /terms, /privacy, /refund-policy, /faq |
| SEO-3 | `GET /` HTML → 메타 태그 포함 | **PASS** (하단 A-3 상세) |

### A-3. HTML 메타 태그

| 태그 | 존재 여부 | 결과 |
|------|----------|------|
| meta description | ✅ | **PASS** |
| link canonical | ✅ `https://amazingkorean.net/` | **PASS** |
| og:type | ✅ `website` | **PASS** |
| og:title | ✅ | **PASS** |
| og:description | ✅ | **PASS** |
| og:image | ✅ | **PASS** |
| og:url | ✅ | **PASS** |
| og:locale | ✅ `ko_KR` + `en_US` alternate | **PASS** |
| twitter:card | ✅ `summary` | **PASS** |
| twitter:title | ✅ | **PASS** |
| twitter:image | ✅ | **PASS** |

### A-4. 프론트엔드 라우팅

| ID | 테스트 | 결과 |
|----|--------|------|
| R-1 | `/terms` → 200 + SPA 엔트리포인트 | **PASS** |
| R-2 | `/privacy` → 200 + SPA 엔트리포인트 | **PASS** |
| R-3 | `/refund-policy` → 200 + SPA 엔트리포인트 | **PASS** |
| R-4 | `/faq` → 200 + SPA 엔트리포인트 | **PASS** |
| R-5 | `/register` → Navigate to `/signup` (client-side) | **PASS** — routes.tsx:78 |
| R-6 | routes.tsx 라우트 정의 확인 | **PASS** — 4개 법적 페이지 + /register 리다이렉트 |

### A-5. i18n 번역 키

| ID | 테스트 | 결과 |
|----|--------|------|
| I-1 | `legal.*` 키 ko/en 동기화 (38개) | **PASS** — terms(7), privacy(7), refund(5), faq(8) + 공통 |
| I-2 | `footer.refundPolicy` ko/en 존재 | **PASS** — "환불 정책" / "Refund Policy" |
| I-3 | `footer.faq/terms/privacy` ko/en 존재 | **PASS** |
| I-4 | `common.goHome` ko/en 존재 | **PASS** — "홈으로 이동" / "Go to Home" |

---

## B. Phase 2: Paddle Webhook (백엔드)

### B-1. Webhook 코드 리뷰

| 파일 | 확인 사항 | 결과 |
|------|----------|------|
| `handler.rs` | handle_webhook: Paddle-Signature 추출 → secret 확인 → body 파싱 → unmarshal (5분 허용) → 이벤트 처리 | **PASS** |
| `service.rs` | process_webhook_event: 멱등성(is_webhook_event_processed) → 8개 구독 이벤트 + 1개 트랜잭션 이벤트 → record_webhook_event | **PASS** |
| `repo.rs` | 3개 새 함수: grant_all_courses (UPSERT), revoke_all_courses, update_course_expiry + 멱등성 쿼리 | **PASS** |
| `router.rs` | POST /webhook 라우트 추가 | **PASS** |

**이벤트 처리 로직 상세:**

| Paddle 이벤트 | 처리 | 수강권 |
|---------------|------|--------|
| subscription.created | INSERT (trialing), custom_data→user_id | - |
| subscription.activated | status=active | grant_all_courses |
| subscription.resumed | (activated와 동일) | grant_all_courses |
| subscription.updated | status 업데이트 + period 갱신 | update_course_expiry (active/trialing) |
| subscription.canceled | status=canceled, canceled_at | update_course_expiry (period_end까지) |
| subscription.paused | status=paused, paused_at | revoke_all_courses |
| subscription.past_due | status=past_due | 유지 (유예 기간) |
| subscription.trialing | status=trialing | grant_all_courses (trial_end) |
| transaction.completed | INSERT transaction 레코드 | - |

### B-2. Webhook API 런타임 테스트

| ID | 테스트 | 기대 | 결과 |
|----|--------|------|------|
| W-1 | POST /payment/webhook (Paddle-Signature 없음) | 400 | **PASS** |
| W-2 | POST /payment/webhook (헤더 있음, secret 미설정) | 200 (에러 로깅) | **PASS** |
| W-3 | GET /payment/webhook | 405 | **PASS** |

### B-3. Paddle Sandbox 연동 테스트 (미수행)

> **선행 조건 미충족**으로 테스트 불가:
> 1. Paddle 대시보드에서 Webhook URL 설정 필요
> 2. PADDLE_WEBHOOK_SECRET .env 설정 필요
> 3. 로컬 서버 외부 노출 (ngrok 등) 필요
>
> Sandbox 연동 후 추가 테스트 항목:
> - subscription.activated 시뮬레이션 → DB subscriptions 확인
> - 동일 이벤트 2회 전송 → webhook_events 1건 확인 (멱등성)
> - user_course 수강권 생성 확인

---

## 빌드 검증

| 항목 | 결과 |
|------|------|
| `cargo check` | **PASS** |
| `npm run build` | **PASS** (TS 0 errors) |
