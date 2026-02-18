# QA Report — 2026-02-18 작업분: 대시보드 데이터 연동 + 관리자 결제 i18n + 인프라

**Date**: 2026-02-18
**Scope**: 관리자 대시보드 실제 데이터 연동, 관리자 결제 4개 페이지 i18n, 인프라 변경
**Method**: Code review + Build verification
**Result**: **44/44 PASS, 0 BUG**

---

## Summary

| Category | Items | Pass | Fail |
|----------|-------|------|------|
| 1. 대시보드 데이터 연동 | 4 | 4 | 0 |
| 2-1. 구독 목록 i18n | 8 | 8 | 0 |
| 2-2. 거래 내역 i18n | 5 | 5 | 0 |
| 2-3. 구독 상세 i18n | 7 | 7 | 0 |
| 2-4. 수동 수강권 i18n | 8 | 8 | 0 |
| 3. 인프라 변경 | 3 | 3 | 0 |
| 빌드 검증 | 2 | 2 | 0 |
| 코드 변경 관련 추가 검증 | 7 | 7 | 0 |
| **합계** | **44** | **44** | **0** |

---

## 1. 관리자 대시보드 — 실제 데이터 연동

**파일**: `frontend/src/category/admin/page/admin_dashboard.tsx`

| # | Test | Result | Evidence |
|---|------|--------|----------|
| 1 | 4개 카드에 실제 숫자 표시 | PASS | `userStats.data?.total_users`, `loginStats.data?.active_sessions`, `videoStats.data?.active_video_count`, `studyStats.data?.total_studies` — 4개 API 호출 (`admin_dashboard.tsx:28-31`) |
| 2 | 로딩 중 Skeleton 애니메이션 | PASS | `isLoading` = 4개 쿼리 OR (`admin_dashboard.tsx:33-37`) → `<Skeleton className="h-8 w-20" />` (`admin_dashboard.tsx:84`) |
| 3 | API 실패 시 "-" 표시 | PASS | `stat.value?.toLocaleString() ?? "-"` (`admin_dashboard.tsx:87`) — `undefined` 시 "-" |
| 4 | 숫자 포맷 (1,234) | PASS | `.toLocaleString()` 사용 (`admin_dashboard.tsx:87`) — 로케일에 따라 자동 포맷 |

### Hook/API 검증
- `useUserStatsSummary` → `GET /admin/users/stats/summary` → `UserStatsSummaryRes.total_users` (Zod: `z.number().int()`)
- `useLoginStatsSummary` → `GET /admin/logins/stats/summary` → `LoginStatsSummaryRes.active_sessions` (Zod: `z.number().int()`)
- `useVideoStatsSummary` → `GET /admin/videos/stats/summary` → `StatsSummaryRes.active_video_count` (Zod: `z.number().int()`)
- `useStudyStatsSummary` → `GET /admin/studies/stats/summary` → `StudyStatsSummaryRes.total_studies` (Zod: `z.number().int()`)
- 모든 쿼리에 `enabled: !!params.from && !!params.to` 조건 설정
- `dateRange`: 최근 30일 (`useMemo`, 렌더링마다 재생성 방지)
- 대시보드 title/subtitle도 i18n 처리: `admin.dashboard.title`, `admin.dashboard.subtitle`

---

## 2-1. 구독 목록 i18n (`admin_subscriptions_page.tsx`)

| # | Test | Result | i18n Key (ko → en) |
|---|------|--------|---------------------|
| 1 | 페이지 타이틀 | PASS | `admin.payment.subscriptions` → "구독 관리" / "Subscriptions" |
| 2 | 네비게이션 버튼 | PASS | `admin.payment.transactions` → "거래 내역" / "Transactions", `admin.payment.manualGrants` → "수동 수강권" / "Manual Grants" |
| 3 | 검색 placeholder | PASS | `admin.payment.searchByEmailOrNickname` → "이메일 또는 닉네임으로 검색..." / "Search by email or nickname..." |
| 4 | 상태 필터 6개 | PASS | `allStatus`="전체 상태", `trialing`="체험 중", `active`="활성", `pastDue`="결제 지연", `paused`="일시정지", `canceled`="취소됨" |
| 5 | 테이블 컬럼 8개 | PASS | `colId`, `colEmail`, `colStatus`, `colInterval`, `colPrice`, `colPeriodEnd`, `colCreated`, `colActions` — 모두 ko/en 존재 |
| 6 | 에러/빈 상태 | PASS | `admin.payment.failedLoad` → "데이터를 불러오지 못했습니다" / "Failed to load data", `admin.payment.noSubscriptions` → "구독 내역이 없습니다" |
| 7 | 하단 카운트 | PASS | `admin.payment.showing` → "{{count}}개 / 총 {{total}}개" / "Showing {{count}} of {{total}}" |
| 8 | "상세" 버튼 | PASS | `admin.payment.detail` → "상세" / "Detail" |

---

## 2-2. 거래 내역 i18n (`admin_transactions_page.tsx`)

| # | Test | Result | i18n Key (ko → en) |
|---|------|--------|---------------------|
| 1 | 뒤로가기 + 타이틀 | PASS | `admin.payment.subscriptions` → "구독 관리", `admin.payment.transactions` → "거래 내역" / "Transactions" |
| 2 | 검색 placeholder | PASS | `admin.payment.searchByEmail` → "이메일로 검색..." / "Search by email..." |
| 3 | 상태 필터 3개 | PASS | `completed`="완료"/"Completed", `refunded`="환불됨"/"Refunded", `partiallyRefunded`="부분 환불"/"Partially Refunded" |
| 4 | 테이블 컬럼 9개 | PASS | `colId`, `colEmail`, `colStatus`, `colAmount`, `colTax`, `colCurrency`, `colInterval`, `colDate`, `colSub` — 모두 존재 |
| 5 | 에러/빈 상태 + 카운트 | PASS | `failedLoad`, `noTransactions`, `showing` — 모두 ko/en |

---

## 2-3. 구독 상세 i18n (`admin_subscription_detail.tsx`)

| # | Test | Result | i18n Key (ko → en) |
|---|------|--------|---------------------|
| 1 | 뒤로 + 타이틀 | PASS | `admin.payment.back` → "뒤로"/"Back", `admin.payment.subscriptionId` → "구독 #{{id}}"/"Subscription #{{id}}" |
| 2 | 구독 카드 라벨 8개 | PASS | `subscription`, `paddleId`, `colInterval`, `colPrice`, `periodStart`, `periodEnd`, `trialEnds`, `canceledAt`, `pausedAt`, `created` — 모두 존재 |
| 3 | 사용자 카드 라벨 4개 | PASS | `user`, `colUserId`, `colEmail`, `colNickname`, `colRole` — 모두 존재 |
| 4 | 작업 카드 | PASS | `actions` → "작업"/"Actions", `cancel` → "취소"/"Cancel" |
| 5 | 거래 테이블 | PASS | `transactions`, `transactionCount` → "거래 {{count}}건"/"{{count}} transaction(s)", 컬럼 헤더 6개 |
| 6 | 취소 다이얼로그 | PASS | `cancelSubscription` → "구독 취소"/"Cancel Subscription", `cancelDescription` → "이 구독의 취소 시점을 선택하세요."/"Choose when to cancel this subscription.", `cancelAtPeriodEnd`, `cancelImmediately` |
| 7 | 취소 toast 메시지 | PASS | `cancelRequested` → "구독 취소가 요청되었습니다"/"Subscription cancel requested", `cancelFailed` → "취소에 실패했습니다"/"Cancel failed" |

---

## 2-4. 수동 수강권 i18n (`admin_grants_page.tsx`)

| # | Test | Result | i18n Key (ko → en) |
|---|------|--------|---------------------|
| 1 | 뒤로 + 타이틀 + 버튼 | PASS | `subscriptions`, `manualGrants` → "수동 수강권"/"Manual Grants", `grantCourses` → "수강권 부여"/"Grant Courses" |
| 2 | 설명 텍스트 | PASS | `grantPageDesc` → "활성 구독 없이 수동 부여된 수강권이 있는 사용자 목록입니다."/"Users with active courses but no active subscription (manually granted)." |
| 3 | 테이블 컬럼 5개 | PASS | `colUserId`, `colEmail`, `colCourses`, `colExpires`, `colActions` |
| 4 | "만료 없음" + "회수" | PASS | `noExpiry` → "만료 없음"/"No expiry", `revoke` → "회수"/"Revoke" |
| 5 | 부여 다이얼로그 | PASS | `grantCourses`, `grantDescription`, `grantUserId`, `grantExpireDate` → "만료일 (선택사항)"/"Expiration Date (optional)", `grantReason`, `grantReasonPlaceholder`, `grantUserIdPlaceholder`, `cancel`, `granting`/`grant` |
| 6 | 회수 다이얼로그 | PASS | `revokeTitle` → "수강권 회수"/"Revoke Courses", `revokeDescription` → "사용자 #{{userId}}의 모든 수강권을 회수하시겠습니까?..." (userId 보간 포함), `cancel`, `revoking`/`revoke` |
| 7 | 성공/실패 toast | PASS | `grantSuccess` → "사용자 #{{userId}}에게 {{count}}개 코스가 부여되었습니다"/"Granted {{count}} courses to user #{{userId}}", `grantFailed`, `revokeSuccess`, `revokeFailed` |
| 8 | 유효성 검사 에러 toast | PASS | `invalidUserId` → "유효한 사용자 ID를 입력해주세요"/"Please enter a valid User ID", `reasonRequired` → "사유를 입력해주세요"/"Please enter a reason" |

---

## 3. 인프라 변경

| File | Change | Result | Evidence |
|------|--------|--------|----------|
| `.env.example` | 누락 변수 추가 (Rate Limit, TTL, Payment, Translation 등) | PASS | 총 38개 변수 — `config.rs` SSoT와 동기화 확인. `RATE_LIMIT_*` 4종, `MFA_TOKEN_TTL_SEC`, `VIMEO_ACCESS_TOKEN`, `TRANSLATE_*` 3개, `PAYMENT_*` 8개, `REFRESH_*` 5개 등 포함 |
| `deploy.yml` | `VIMEO_ACCESS_TOKEN` 환경변수 추가 | PASS | `deploy.yml:85` — `VIMEO_ACCESS_TOKEN=${{ secrets.VIMEO_ACCESS_TOKEN }}` |
| `AMK_API_MASTER.md` | §8.5 Paddle Live 전환 체크리스트 | PASS | 문서만 변경 — 기능 영향 없음 |

---

## 빌드 검증

| # | Test | Result |
|---|------|--------|
| B-1 | `cargo check` | PASS |
| B-2 | `npm run build` | PASS (8.14s) |

---

## 코드 변경 관련 추가 검증

이번 작업에서 기능 변경도 함께 발생한 사항들의 정합성을 검증합니다.

| # | 변경 사항 | Result | Evidence |
|---|----------|--------|----------|
| C-1 | 구독 상세에서 Pause/Resume 버튼 제거 → Cancel만 유지 | PASS | `admin_subscription_detail.tsx` — `useAdminPauseSubscription`, `useAdminResumeSubscription` import 제거, Pause/Play 아이콘 import 제거, XCircle만 남음 |
| C-2 | `use_admin_payment.ts`에서 pause/resume 훅 import 제거 | PASS | import에서 `adminPauseSubscription`, `adminResumeSubscription` 제거 — 사용하지 않는 코드 clean |
| C-3 | 백엔드 `service.rs`에서 pause/resume 함수 제거 | PASS | `admin/payment/service.rs` 섹션 제목 "관리자 구독 관리 (cancel)" — cancel만 유지 |
| C-4 | 사이드바 NAV-1 버그 수정 | PASS | `admin_layout.tsx:10` — path를 `/admin/payment/subscriptions`로 변경, `prefix: "/admin/payment"` 추가, `isActive` 함수에 prefix 파라미터 지원 |
| C-5 | Grant 만료일 입력 `datetime-local` → `date`로 변경 | PASS | `admin_grants_page.tsx:253` — `type="date"`, expire_at 전송 시 `${grantExpireAt}T23:59:59Z` 자동 변환 |
| C-6 | Grant 목록 쿼리 `MIN` → `MAX` expire_at | PASS | `repo.rs:282` — `MAX(uc.user_course_expire_at)` — 가장 늦은 만료일 표시 (합리적) |
| C-7 | 대시보드 i18n 적용 | PASS | `admin.dashboard.*` 7개 키 — title, subtitle, totalUsers, activeSessions, totalVideos, totalStudies, quickActions, quickActionsDesc |

---

## i18n 키 전수 검증

### `admin.dashboard.*` (7개)

| Key | ko | en |
|-----|----|----|
| title | 대시보드 | Dashboard |
| subtitle | Amazing Korean 관리자 패널에 오신 것을 환영합니다 | Welcome to Amazing Korean Admin Panel |
| totalUsers | 전체 사용자 | Total Users |
| activeSessions | 활성 세션 | Active Sessions |
| totalVideos | 전체 영상 | Total Videos |
| totalStudies | 전체 학습 | Total Studies |
| quickActions | 빠른 작업 | Quick Actions |
| quickActionsDesc | 사이드바에서 메뉴를 선택하여 콘텐츠를 관리하세요. | Select a menu item from the sidebar to manage content. |

### `admin.payment.*` (82개 키)

모든 82개 키가 ko.json과 en.json에 동일하게 존재하며, 1:1 대응 확인.

주요 보간 파라미터 검증:
- `showing`: `{{count}}`, `{{total}}` — 양쪽 일치
- `transactionCount`: `{{count}}` — 양쪽 일치
- `subscriptionId`: `{{id}}` — 양쪽 일치
- `paddleId`: `{{id}}` — 양쪽 일치
- `grantSuccess`: `{{userId}}`, `{{count}}` — 양쪽 일치
- `revokeSuccess`: `{{userId}}` — 양쪽 일치
- `revokeDescription`: `{{userId}}` — 양쪽 일치

---

## Conclusion

전체 **44개 항목 모두 PASS**. 버그 없음.

주요 변경사항:
1. 대시보드가 하드코딩 "-" → 실제 4개 통계 API 호출로 전환
2. 관리자 결제 4개 페이지 전체 i18n 완료 (82개 `admin.payment.*` 키)
3. 관리자 구독 상세에서 Pause/Resume 버튼 제거 (Cancel만 유지)
4. NAV-1 사이드바 버그 수정 (prefix 기반 active state)
5. `.env.example` 환경변수 동기화 + `deploy.yml` VIMEO_ACCESS_TOKEN 추가
