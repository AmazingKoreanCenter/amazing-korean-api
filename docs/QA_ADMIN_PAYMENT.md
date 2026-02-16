# QA Report — Phase 5: Admin Payment Management + Promo Code

**Date**: 2026-02-16
**Scope**: 관리자 결제 관리 패널 (구독/트랜잭션/수동 수강권) + 프로모션 코드 UI
**Method**: Code review + Build verification (브라우저 렌더링 불가)
**Result**: **47/48 PASS, 1 BUG 발견**

---

## Summary

| Category | Items | Pass | Fail | Bug |
|----------|-------|------|------|-----|
| 1-1. 구독 목록 | 10 | 10 | 0 | 0 |
| 1-2. 구독 상세 | 10 | 10 | 0 | 0 |
| 1-3. 트랜잭션 목록 | 8 | 8 | 0 | 0 |
| 1-4. 수동 수강권 | 9 | 9 | 0 | 0 |
| 2. 프로모션 코드 | 5 | 5 | 0 | 0 |
| 3. 빌드 검증 | 2 | 2 | 0 | 0 |
| 라우팅/사이드바 | 3 | 2 | 1 | 1 |
| API 경로 매칭 | 1 | 1 | 0 | 0 |
| **합계** | **48** | **47** | **1** | **1** |

---

## Bug Found

### NAV-1: 사이드바 "Payments" 링크 → 빈 페이지 [MEDIUM]

**파일**: `frontend/src/category/admin/page/admin_layout.tsx:10`
**현상**: 사이드바 "Payments" 링크의 path가 `/admin/payment`이지만, routes.tsx에 해당 라우트가 없음
**영향**: 클릭 시 AdminLayout 내부 Outlet이 비어서 빈 콘텐츠 영역만 표시
**수정안**:
- Option A: `admin_layout.tsx` line 10 — path를 `/admin/payment/subscriptions`로 변경
- Option B: `routes.tsx`에 `<Route path="payment" element={<Navigate to="payment/subscriptions" replace />} />` 추가

---

## 1-1. 구독 목록 페이지 (`/admin/payment/subscriptions`)

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| SL-1 | 페이지 로딩 시 스켈레톤 표시 | PASS | `admin_subscriptions_page.tsx:176-183` — 5행 × 8열 스켈레톤 |
| SL-2 | 테이블 컬럼: ID, Email, Status, Interval, Price, Period End, Created, Actions | PASS | `admin_subscriptions_page.tsx:149-173` — 8개 컬럼 정의 |
| SL-3 | 이메일/닉네임 검색 (form submit) | PASS | FE: `handleSearch` → `params.q`, BE: `service.rs:103-111` blind index(@) 또는 ILIKE |
| SL-4 | 상태 필터 (5종 + All) | PASS | FE: SelectItem 6개, BE: `service.rs:92-99` 5종 검증 |
| SL-5 | 5개 컬럼 정렬 (id, created_at, status, billing_interval, price) | PASS | FE: `SortField` 타입 5개, BE: `service.rs:82-84` + `repo.rs:74-80` match |
| SL-6 | 정렬 토글 (asc ↔ desc) + 아이콘 | PASS | `handleSort` 동일 필드 클릭 시 토글, `SortIcon` 컴포넌트 |
| SL-7 | 페이지네이션 (max 5 links) | PASS | `admin_subscriptions_page.tsx:239-252` Math.min(5, total_pages) |
| SL-8 | Detail 버튼 → `/admin/payment/subscriptions/:id` | PASS | `admin_subscriptions_page.tsx:215-220` Link 컴포넌트 |
| SL-9 | Transactions / Manual Grants 네비게이션 버튼 | PASS | `admin_subscriptions_page.tsx:97-109` 2개 Button asChild |
| SL-10 | 에러/빈 상태 처리 | PASS | `admin_subscriptions_page.tsx:184-195` isError + empty |

### Backend Cross-Check
- **RBAC**: `service.rs:76` check_admin_rbac (HYMN/Admin/Manager)
- **이메일 복호화**: `service.rs:126-128` CryptoService.decrypt
- **감사 로그**: `service.rs:130-139` LIST_SUBSCRIPTIONS 기록
- **SQL 인젝션 방지**: `repo.rs:39-63` 바인드 파라미터 사용, sort/order는 match로 화이트리스트
- **페이지 크기 제한**: `service.rs:79` clamp(1, 100)

---

## 1-2. 구독 상세 페이지 (`/admin/payment/subscriptions/:id`)

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| SD-1 | 구독 정보 카드 (Interval, Price, Period, Paddle ID) | PASS | `admin_subscription_detail.tsx:124-163` 8개 필드 |
| SD-2 | 사용자 정보 카드 (User ID link, Email, Nickname, Role) | PASS | `admin_subscription_detail.tsx:167-191` Link to /admin/users/:id |
| SD-3 | 트랜잭션 테이블 (ID, Status, Amount, Tax, Currency, Date) | PASS | `admin_subscription_detail.tsx:238-276` 6개 컬럼 |
| SD-4 | 빈 트랜잭션 처리 | PASS | `admin_subscription_detail.tsx:244` "No transactions yet" |
| SD-5 | Pause 버튼 (active only) | PASS | `admin_subscription_detail.tsx:200-210` `sub.status === "active"` |
| SD-6 | Resume 버튼 (paused only) | PASS | `admin_subscription_detail.tsx:211-220` `sub.status === "paused"` |
| SD-7 | Cancel 버튼 (not canceled) → Dialog | PASS | `admin_subscription_detail.tsx:194,222-230` `sub.status !== "canceled"` |
| SD-8 | Cancel Dialog: Period End + Immediately | PASS | `admin_subscription_detail.tsx:289-302` 2개 버튼 |
| SD-9 | Back 버튼 → `/admin/payment/subscriptions` | PASS | `admin_subscription_detail.tsx:110-115` Link 컴포넌트 |
| SD-10 | 에러/로딩 상태 | PASS | `admin_subscription_detail.tsx:87-102` Skeleton + error |

### Backend Cross-Check
- **구독 조회**: `repo.rs:107-127` subscription_id로 조회
- **사용자 조회**: `repo.rs:130-146` user_id로 조회
- **트랜잭션 조회**: `repo.rs:153-174` subscription_id 기준 DESC 정렬
- **Cancel 로직**: `service.rs:212-272` 상태 검증 → CancelEffectiveFrom → Paddle API → audit
- **Pause 로직**: `service.rs:274-322` Active만 가능 → Paddle API → audit
- **Resume 로직**: `service.rs:324-372` Paused만 가능 → Paddle API → audit
- **Payment Provider null 체크**: 모든 mutation에서 `st.payment.as_ref().ok_or_else(ServiceUnavailable)`

---

## 1-3. 트랜잭션 목록 페이지 (`/admin/payment/transactions`)

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| TL-1 | 이메일 검색 | PASS | FE: form submit, BE: `service.rs:410-418` blind index 검색 |
| TL-2 | 상태 필터 (Completed, Refunded, Partially Refunded) | PASS | FE: `admin_transactions_page.tsx:111-114`, BE: `service.rs:400-407` |
| TL-3 | 4개 컬럼 정렬 (id, occurred_at, amount, status) | PASS | FE: `SortField` 4개, BE: `repo.rs:229-233` match |
| TL-4 | Sub 링크 → `/admin/payment/subscriptions/:id` | PASS | `admin_transactions_page.tsx:182-188` subscription_id 있을 때만 Link |
| TL-5 | 페이지네이션 | PASS | 동일 패턴 (max 5 links) |
| TL-6 | Back → Subscriptions | PASS | `admin_transactions_page.tsx:78-83` ArrowLeft + Link |
| TL-7 | 에러/빈 상태 | PASS | `admin_transactions_page.tsx:156-167` 처리 |
| TL-8 | 테이블 컬럼: ID, Email, Status, Amount, Tax, Currency, Interval, Date, Sub | PASS | 9개 컬럼 확인 |

### Backend Cross-Check
- **SQL 인젝션 방지**: `repo.rs:201-221` 바인드 파라미터
- **이메일 복호화**: `service.rs:431-433`
- **감사 로그**: `service.rs:435-444` LIST_TRANSACTIONS

---

## 1-4. 수동 수강권 페이지 (`/admin/payment/grants`)

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| GR-1 | "Grant Courses" 버튼 → 다이얼로그 | PASS | `admin_grants_page.tsx:109-112` setGrantDialogOpen(true) |
| GR-2 | 다이얼로그: User ID (number), Expiry (datetime-local), Reason (Textarea) | PASS | `admin_grants_page.tsx:241-265` 3개 입력 필드 |
| GR-3 | 프론트엔드 유효성 검증 (User ID > 0, Reason 필수) | PASS | `admin_grants_page.tsx:55-63` parseInt + trim 검증 |
| GR-4 | expire_at → ISO string 변환 | PASS | `admin_grants_page.tsx:68` `new Date(grantExpireAt).toISOString()` |
| GR-5 | 성공 시 다이얼로그 닫기 + 폼 초기화 + toast | PASS | `admin_grants_page.tsx:72-77` 3개 state 리셋 |
| GR-6 | 테이블: User ID (link), Email, Courses, Expires, Revoke | PASS | `admin_grants_page.tsx:122-129` 5개 컬럼 |
| GR-7 | Revoke 확인 다이얼로그 | PASS | `admin_grants_page.tsx:279-301` DialogDescription에 user_id 표시 |
| GR-8 | 페이지네이션 | PASS | 동일 패턴 (max 5 links) |
| GR-9 | Back → Subscriptions | PASS | `admin_grants_page.tsx:101-105` Link |

### Backend Cross-Check
- **grant_all_courses**: `payment/repo.rs:303-325` UPSERT → active 코스 전체에 user_course 생성
- **revoke_all_courses**: `payment/repo.rs:328-342` active → false
- **사용자 존재 확인**: `service.rs:477-479` find_user → 없으면 BadRequest
- **감사 로그**: `service.rs:497-511` GRANT_COURSES, `service.rs:588-600` REVOKE_COURSES
- **list_manual_grants 쿼리**: `repo.rs:261-298` LEFT JOIN subscriptions → 활성 구독 없는 사용자만 필터

---

## 2. 프로모션 코드 UI (`/pricing`)

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| PC-1 | 프로모 코드 입력 필드 (비구독 시만 표시) | PASS | `pricing_page.tsx:338` `!hasActiveSub` 조건 |
| PC-2 | Tag 아이콘 + Input + placeholder | PASS | `pricing_page.tsx:342-348` Tag 아이콘, `pl-9`, i18n key |
| PC-3 | Clear 버튼 (코드 입력 시만 표시) | PASS | `pricing_page.tsx:350-358` `promoCode &&` 조건 |
| PC-4 | Paddle checkout에 discount code 전달 | PASS | `pricing_page.tsx:72` → `use_paddle.ts:53` `discountCode` |
| PC-5 | i18n 키 매칭 (en/ko 모두) | PASS | en: "Enter promo code"/"Clear", ko: "프로모션 코드 입력"/"삭제" |

### use_paddle.ts 상세 검증
- `openCheckout(priceId: string, discountCode?: string)` — Optional 파라미터
- `paddle.Checkout.open({ discountCode: discountCode || undefined })` — 빈 문자열 방지
- `customData: { user_id: String(userId) }` — 사용자 ID 전달
- `pricing_page.tsx:72` — `promoCode.trim() || undefined` — 공백만 있는 경우도 처리

---

## 3. 빌드 검증

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| B-1 | `cargo check` | PASS | 백엔드 컴파일 성공 |
| B-2 | `npm run build` | PASS | tsc + vite 빌드 성공 (10.25s) |

---

## 라우팅 + 사이드바 검증

| ID | Test | Result | Evidence |
|----|------|--------|----------|
| RT-1 | Admin routes 4개 등록 | PASS | `routes.tsx:144-147` payment/subscriptions, :id, transactions, grants |
| RT-2 | Admin sidebar "Payments" 메뉴 | **BUG** | `admin_layout.tsx:10` path `/admin/payment` → 빈 페이지 (NAV-1) |
| RT-3 | Sidebar active state (startsWith) | PASS | `admin_layout.tsx:18-23` `/admin/payment/*` 모두 하이라이트 |

---

## API 경로 매칭 (Frontend ↔ Backend)

| Frontend (admin_api.ts) | Backend Route | Match |
|--------------------------|---------------|-------|
| `GET /admin/payment/subscriptions` | `GET /subscriptions` (nested `/admin/payment`) | PASS |
| `GET /admin/payment/subscriptions/:id` | `GET /subscriptions/{id}` | PASS |
| `POST /admin/payment/subscriptions/:id/cancel` | `POST /subscriptions/{id}/cancel` | PASS |
| `POST /admin/payment/subscriptions/:id/pause` | `POST /subscriptions/{id}/pause` | PASS |
| `POST /admin/payment/subscriptions/:id/resume` | `POST /subscriptions/{id}/resume` | PASS |
| `GET /admin/payment/transactions` | `GET /transactions` | PASS |
| `POST /admin/payment/grants` | `POST /grants` | PASS |
| `GET /admin/payment/grants` | `GET /grants` | PASS |
| `DELETE /admin/payment/grants/:userId` | `DELETE /grants/{user_id}` | PASS |

---

## Type 매칭 (Frontend ↔ Backend)

| DTO | Frontend | Backend | Match |
|-----|----------|---------|-------|
| AdminSubSummary | 8 fields | 8 fields (sqlx::FromRow) | PASS |
| AdminSubDetail | 14 fields | 14 fields (sqlx::FromRow) | PASS |
| AdminSubUser | 4 fields | 4 fields (sqlx::FromRow) | PASS |
| AdminTxnSummary | 10 fields | 10 fields (sqlx::FromRow) | PASS |
| AdminGrantReq | user_id, expire_at?, reason | user_id, expire_at: Option<DateTime>, reason | PASS |
| AdminGrantRes | user_id, courses_granted, expire_at | user_id, courses_granted: u64, expire_at: Option<String> | PASS |
| AdminGrantSummary | 4 fields | 4 fields (sqlx::FromRow) | PASS |
| AdminCancelSubReq | { immediately: boolean } | { immediately: bool, #[serde(default)] } | PASS |

---

## Hook 검증 (TanStack Query)

| Hook | Query Key | Invalidation | Notes |
|------|-----------|-------------|-------|
| useAdminSubscriptions | `["admin","payment","subscriptions","list",params]` | — | — |
| useAdminSubscriptionDetail | `["admin","payment","subscriptions","detail",id]` | — | `enabled: id > 0` |
| useAdminCancelSubscription | — | subscriptions + detail(id) | `{ id, data }` 구조 |
| useAdminPauseSubscription | — | subscriptions + detail(id) | — |
| useAdminResumeSubscription | — | subscriptions + detail(id) | — |
| useAdminTransactions | `["admin","payment","transactions","list",params]` | — | — |
| useAdminGrants | `["admin","payment","grants","list",params]` | — | — |
| useCreateAdminGrant | — | grants 전체 | — |
| useRevokeAdminGrant | — | grants 전체 | — |

---

## 보안 검증 체크리스트

| Check | Result |
|-------|--------|
| 모든 admin API에 RBAC 검증 (HYMN/Admin/Manager) | PASS |
| 모든 admin API에 감사 로그 | PASS |
| IP 주소 감사 로그 암호화 (AES-256-GCM) | PASS |
| 이메일 복호화는 서버에서만 (blind index 검색) | PASS |
| SQL 인젝션 방지 (바인드 파라미터 + match 화이트리스트) | PASS |
| 페이지 크기 제한 (max 100) | PASS |
| PaymentProvider null 체크 (ServiceUnavailable) | PASS |
| 구독 상태 검증 후 Paddle API 호출 | PASS |

---

## i18n 프로모션 코드 키

| Key | en | ko |
|-----|----|----|
| payment.promoCodePlaceholder | "Enter promo code" | "프로모션 코드 입력" |
| payment.promoCodeClear | "Clear" | "삭제" |

총 payment.* 키: **45개** (Phase 3: 29 + Phase 4: 14 + Phase 5: 2)

---

## Conclusion

전체 **48개 항목** 중 **47개 PASS**, **1개 BUG** (NAV-1: 사이드바 링크).
NAV-1은 심각도 MEDIUM — 사용자가 직접 `/admin/payment/subscriptions`로 이동하면 정상 작동하지만, 사이드바 클릭 시 빈 페이지가 나타나므로 수정 필요.
