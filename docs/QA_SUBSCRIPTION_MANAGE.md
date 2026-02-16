# Phase 4: 구독 관리 API (cancel / pause / resume) — QA 리포트

- **일시**: 2026-02-16
- **범위**: 백엔드 3파일 수정 + 프론트엔드 6파일 수정/생성
- **결과**: **38/38 PASS** | 이슈 0건

---

## 4. 빌드 검증

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| B-1 | `npm run build` | **PASS** | TS 0 에러, ✓ built in 7.14s |
| B-2 | `cargo check` | **PASS** | 컴파일 정상 |

---

## 1.1 인증 검증 (런타임)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Auth-1 | `POST /payment/subscription/cancel` 토큰 없이 | **PASS** | 401 `"Missing or invalid Authorization header"` |
| Auth-2 | `POST /payment/subscription/pause` 토큰 없이 | **PASS** | 401 동일 |
| Auth-3 | `POST /payment/subscription/resume` 토큰 없이 | **PASS** | 401 동일 |

**검증 방법**: `curl -X POST http://localhost:3000/payment/subscription/{cancel,pause,resume}` — 3개 모두 401 반환 확인

---

## 1.2 구독 없는 사용자 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| NoSub-1 | cancel → 400 "No active subscription" | **PASS** | `service.rs:103-105`: `get_active_subscription` → `None` → `BadRequest("No active subscription")` |
| NoSub-2 | pause → 400 "No active subscription" | **PASS** | `service.rs:138-140`: 동일 로직 |
| NoSub-3 | resume → 400 "No subscription found" | **PASS** | `service.rs:172-174`: `get_latest_subscription` → `None` → `BadRequest("No subscription found")` |

**코드 흐름 확인**:
- cancel/pause: `get_active_subscription` (trialing/active/past_due만 조회) → `None` → 400
- resume: `get_latest_subscription` (상태 무관) → `None` → 400

---

## 1.3 cancel — 상태별 동작 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Cancel-1 | active + immediately:false → NextBillingPeriod | **PASS** | `service.rs:107-111`: `CancelEffectiveFrom::NextBillingPeriod` → Paddle API 호출 |
| Cancel-2 | active + immediately:true → Immediately | **PASS** | `service.rs:107-111`: `CancelEffectiveFrom::Immediately` → Paddle API 호출 |
| Cancel-3 | trialing + cancel → 성공 | **PASS** | `get_active_subscription`이 trialing 포함 (repo.rs:80: `IN ('trialing', 'active', 'past_due')`) |
| Cancel-4 | paused → 400 | **PASS** | `get_active_subscription` WHERE 절에 paused 미포함 → `None` → 400 |

**Paddle SDK 호출 확인** (`payment.rs:157-166`):
```rust
self.client.subscription_cancel(provider_subscription_id)
    .effective_from(effective)  // EffectiveFrom::NextBillingPeriod | Immediately
    .send().await
```

---

## 1.4 pause — 상태별 동작 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Pause-1 | active → Paddle API 성공 | **PASS** | `service.rs:142-146`: `status != Active` 검사 통과 → `pause_subscription` 호출 |
| Pause-2 | trialing → 400 | **PASS** | `service.rs:142`: `sub.status != SubscriptionStatus::Active` → `BadRequest("Only active subscriptions can be paused")` |

**Paddle SDK 호출 확인** (`payment.rs:175-183`):
```rust
self.client.subscription_pause(provider_subscription_id)
    .send().await
```

---

## 1.5 resume — 상태별 동작 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Resume-1 | paused → Paddle API 성공 | **PASS** | `service.rs:176`: `status != Paused` 검사 통과 → `resume_subscription` 호출 |
| Resume-2 | active → 400 | **PASS** | `service.rs:176-179`: `status != Paused` → `BadRequest("Only paused subscriptions can be resumed")` |
| Resume-3 | canceled → 400 | **PASS** | 동일 로직 — Canceled != Paused → 400 |

**resume 특이점**: `get_latest_subscription` 사용 (상태 무관 조회) — paused 상태 구독을 찾을 수 있음 ✓

**Paddle SDK 호출 확인** (`payment.rs:192-201`):
```rust
self.client.subscription_resume(provider_subscription_id)
    .effective_from(chrono::Utc::now())  // 즉시 재개
    .send().await
```

---

## 1.6 Paddle API 연동 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Paddle-1 | cancel → Paddle SDK 호출 | **PASS** | `payment.rs:147-168`: `subscription_cancel().effective_from().send()` |
| Paddle-2 | pause → Paddle SDK 호출 | **PASS** | `payment.rs:171-185`: `subscription_pause().send()` |
| Paddle-3 | resume → Paddle SDK 호출 | **PASS** | `payment.rs:188-203`: `subscription_resume().effective_from(Utc::now()).send()` |

**참고**: 실제 Paddle Sandbox API 호출은 구독이 존재해야 테스트 가능. 코드 리뷰로 SDK 호출 패턴이 Phase 1에서 검증된 `get_subscription`과 일관됨을 확인.

---

## 1.7 Webhook 연동 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| WH-1 | cancel → subscription.canceled webhook | **PASS** | `service.rs:410-442`: 상태→Canceled, canceled_at 저장, 수강권 만료일→period_end |
| WH-2 | pause → subscription.paused webhook | **PASS** | `service.rs:444-476`: 상태→Paused, paused_at 저장, `revoke_all_courses` 호출 |
| WH-3 | resume → subscription.resumed webhook | **PASS** | `service.rs:224-226`: SubscriptionResumed → `handle_subscription_activated` (Active 상태 + 수강권 부여) |

**수강권 연동 상세**:
- **canceled**: `update_course_expiry(period_end)` — 기간 종료까지 유지 후 만료 ✓
- **paused**: `revoke_all_courses(user_id)` — 즉시 비활성화 ✓
- **resumed**: `grant_all_courses(user_id, period_end)` — UPSERT로 수강권 재부여 ✓

---

## 2.1 구독 배너 표시 — 상태별 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Banner-1 | active → 초록 배너 + [일시정지] + [구독 취소] | **PASS** | `pricing_page.tsx:121,175-195`: emerald 색상, `isActive && (<PauseCircle> + <CancelBtn>)` |
| Banner-2 | trialing → 초록 배너 + [구독 취소]만 | **PASS** | `pricing_page.tsx:197-208`: `isTrialing && (<CancelBtn>)` — 일시정지 없음 ✓ |
| Banner-3 | paused → 노란 배너 + [재개] + [구독 취소] | **PASS** | `pricing_page.tsx:121,209-231`: amber 색상, `isPaused && (<PlayCircle> + <CancelBtn>)` |
| Banner-4 | canceled → 빨간 배너 + 버튼 없음 + 만료일 | **PASS** | `pricing_page.tsx:123-124`: red 색상, `isCanceled` 조건에 관리 버튼 미포함, `expiresAt` 표시 (line 162-163) |
| Banner-5 | past_due → 초록 배너 + 버튼 없음 | **PASS** | `pricing_page.tsx:121,125`: 기본값 emerald, `isActive`/`isTrialing`/`isPaused` 모두 false → 버튼 미표시 |
| Banner-6 | 구독 없음 → 배너 미표시 | **PASS** | `pricing_page.tsx:128`: `hasSub && subscription` — 구독 없으면 IIFE 미실행 |

**배너 색상/아이콘 매핑** (pricing_page.tsx:121-131):

| 상태 | 배경 gradient | 아이콘 | 텍스트 |
|------|---------------|--------|--------|
| paused | amber-50→orange-50, border-amber-200 | PauseCircle (amber-600) | amber-900 |
| canceled | red-50→rose-50, border-red-200 | XCircle (red-600) | red-900 |
| 기타 | emerald-50→teal-50, border-emerald-200 | CheckCircle2 (emerald-600) | emerald-900 |

---

## 2.2 취소 다이얼로그 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Dialog-1 | [구독 취소] 클릭 → Dialog 오픈 | **PASS** | `setCancelDialogOpen(true)` (line 175, 188, 210, 226) |
| Dialog-2 | 제목: "구독을 취소하시겠습니까?" / "Cancel your subscription?" | **PASS** | `t("payment.cancelConfirmTitle")` — ko/en 확인됨 |
| Dialog-3 | 설명 텍스트 | **PASS** | `t("payment.cancelConfirmMessage")` — 즉시/기간종료 차이 설명 |
| Dialog-4 | [기간 종료 시 취소] → immediately:false | **PASS** | `pricing_page.tsx:250-253`: `cancelMutation.mutate({ immediately: false }, { onSuccess: () => setCancelDialogOpen(false) })` |
| Dialog-5 | [즉시 취소] → immediately:true | **PASS** | `pricing_page.tsx:261-264`: `cancelMutation.mutate({ immediately: true }, { onSuccess: () => setCancelDialogOpen(false) })` |
| Dialog-6 | API 호출 중 버튼 disabled | **PASS** | `disabled={cancelMutation.isPending}` (line 254, 265) |

---

## 2.3 일시정지 / 재개 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Manage-1 | [일시정지] → API 호출 + toast + 데이터 갱신 | **PASS** | `pauseMutation.mutate()` → `onSuccess: invalidateQueries(["payment","subscription"]) + toast.success` (use_manage_subscription.ts:30-31) |
| Manage-2 | [재개] → API 호출 + toast + 데이터 갱신 | **PASS** | `resumeMutation.mutate()` → `onSuccess: invalidateQueries(["payment","subscription"]) + toast.success` (use_manage_subscription.ts:46-47) |
| Manage-3 | 에러 시 실패 toast | **PASS** | `onError: toast.error(t("payment.{pause,resume}Failed"))` (use_manage_subscription.ts:34-35, 50-51) |

---

## 2.4 버튼 상태 관리 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| State-1 | 어느 mutation이든 pending → 모든 관리 버튼 disabled | **PASS** | `isBusy = cancelMutation.isPending \|\| pauseMutation.isPending \|\| resumeMutation.isPending` (line 134), 모든 Button에 `disabled={isBusy}` |
| State-2 | 성공 후 쿼리 invalidate | **PASS** | 3개 훅 모두 `queryClient.invalidateQueries({ queryKey: ["payment", "subscription"] })` |

---

## 2.5 반응형 (코드 리뷰)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| Resp-1 | 모바일: 세로 레이아웃 | **PASS** | `flex flex-col sm:flex-row` (line 151) — 기본 세로, sm 이상 가로 |
| Resp-2 | 데스크톱: 가로 레이아웃 | **PASS** | `items-start sm:items-center justify-between` — sm 이상에서 양쪽 정렬 |

---

## 3. i18n 번역 검증 (런타임)

| ID | 테스트 | 결과 | 비고 |
|----|--------|------|------|
| I-1 | 한국어 14개 Phase 4 키 | **PASS** | 모두 존재 + 값 정상 |
| I-2 | 영어 14개 Phase 4 키 | **PASS** | 모두 존재 + 값 정상 |
| I-3 | 전체 payment.* 키 동기화 | **PASS** | ko 43개 = en 43개, 차이 없음 |

**Phase 4 추가 키 목록** (14개):

| 키 | 한국어 | 영어 |
|----|--------|------|
| payment.cancelSubscription | 구독 취소 | Cancel Subscription |
| payment.pauseSubscription | 일시정지 | Pause |
| payment.resumeSubscription | 재개 | Resume |
| payment.cancelConfirmTitle | 구독을 취소하시겠습니까? | Cancel your subscription? |
| payment.cancelConfirmMessage | 즉시 취소하면 바로 콘텐츠 접근이 중단됩니다... | If you cancel immediately, your access will be revoked... |
| payment.cancelAtPeriodEnd | 기간 종료 시 취소 | Cancel at Period End |
| payment.cancelImmediate | 즉시 취소 | Cancel Immediately |
| payment.cancelSuccess | 구독이 취소되었습니다. | Your subscription has been canceled. |
| payment.cancelFailed | 구독 취소에 실패했습니다... | Failed to cancel subscription... |
| payment.pauseSuccess | 구독이 일시정지되었습니다. | Your subscription has been paused. |
| payment.pauseFailed | 구독 일시정지에 실패했습니다... | Failed to pause subscription... |
| payment.resumeSuccess | 구독이 재개되었습니다. | Your subscription has been resumed. |
| payment.resumeFailed | 구독 재개에 실패했습니다... | Failed to resume subscription... |
| payment.expiresAt | 만료일 | Expires |

---

## 코드 품질 리뷰

### 백엔드

**service.rs — cancel/pause/resume 메서드 (lines 89-193)**:
- PaymentProvider trait 추상화 적절 — Paddle 의존성 격리 ✓
- cancel: `CancelEffectiveFrom` enum으로 즉시/기간종료 분기 ✓
- pause: `status != Active` 검증 후 Paddle API 호출 ✓
- resume: `get_latest_subscription` 사용 (paused 포함) + `status != Paused` 검증 ✓
- 응답: Paddle API 호출 후 `get_subscription`으로 최신 DB 상태 반환 (webhook 반영 전이므로 즉시 갱신 아님) ✓
- tracing::info 로깅 — user_id, sub_id, action 포함 ✓

**handler.rs — 3개 핸들러 (lines 54-120)**:
- `AuthUser(auth_user)` extractor로 인증 강제 ✓
- `#[utoipa::path]` Swagger 문서 포함 ✓
- cancel만 `Json(req)` body 파싱, pause/resume은 body 없음 ✓

**dto.rs — CancelSubscriptionReq (lines 70-76)**:
- `#[serde(default)]` on `immediately` — body 없이 호출 시 false 기본값 ✓
- `Deserialize + ToSchema` 적용 ✓

**router.rs (lines 11-13)**:
- `post(handler::cancel_subscription)`, `post(handler::pause_subscription)`, `post(handler::resume_subscription)` ✓
- `/subscription/` 하위 경로로 일관된 네이밍 ✓

**payment.rs — Paddle SDK 호출 (lines 147-203)**:
- `cancel`: `effective_from(EffectiveFrom::NextBillingPeriod | Immediately)` ✓
- `pause`: 파라미터 없이 `subscription_pause().send()` ✓
- `resume`: `effective_from(chrono::Utc::now())` — 즉시 재개 ✓
- 에러 핸들링: `tracing::error` + `AppError::External` ✓

### 프론트엔드

**use_manage_subscription.ts**:
- 3개 mutation 훅 분리 (cancel, pause, resume) — 단일 책임 원칙 ✓
- `onSuccess`: queryClient.invalidateQueries + toast.success ✓
- `onError`: toast.error ✓
- `void queryClient.invalidateQueries` — Promise 무시 처리 ✓

**pricing_page.tsx — 구독 배너 IIFE**:
- `hasSub && subscription` 조건으로 배너 표시 제어 ✓
- 상태별 색상/아이콘/버튼 매핑이 명확한 로컬 변수로 분리 ✓
- `isBusy` 통합 변수로 모든 버튼 disable 관리 ✓
- 취소 다이얼로그: `onSuccess` 콜백에서 Dialog 닫기 ✓
- `canceled` 상태: `expiresAt` 표시 vs `nextBilling` 분기 (line 146-151) ✓

**payment_api.ts**:
- cancel: `{ method: "POST", data }` — body 전달 ✓
- pause/resume: `{ method: "POST" }` — body 없음 ✓
- 반환 타입 모두 `SubscriptionRes` 통일 ✓

**types.ts**:
- `CancelSubscriptionReq` interface — `immediately: boolean` ✓
- Zod 스키마 없이 interface 사용 (요청 전용, 검증 불필요) ✓

---

## 요약

| 카테고리 | 항목 수 | 결과 |
|----------|---------|------|
| 1.1 인증 검증 (런타임) | 3 | 3/3 PASS |
| 1.2 구독 없는 사용자 (코드 리뷰) | 3 | 3/3 PASS |
| 1.3 cancel 상태별 동작 | 4 | 4/4 PASS |
| 1.4 pause 상태별 동작 | 2 | 2/2 PASS |
| 1.5 resume 상태별 동작 | 3 | 3/3 PASS |
| 1.6 Paddle API 연동 | 3 | 3/3 PASS |
| 1.7 Webhook 연동 | 3 | 3/3 PASS |
| 2.1 구독 배너 (상태별) | 6 | 6/6 PASS |
| 2.2 취소 다이얼로그 | 6 | 6/6 PASS |
| 2.3 일시정지/재개 | 3 | 3/3 PASS |
| 2.4 버튼 상태 관리 | 2 | 2/2 PASS |
| 2.5 반응형 | 2 | 2/2 PASS |
| 3. i18n 번역 | 3 | 3/3 PASS |
| 4. 빌드 검증 | 2 | 2/2 PASS |
| **합계** | **45** | **45/45 PASS** |

> **Paddle Sandbox 실제 통합 테스트**: cancel/pause/resume API의 실제 Paddle Sandbox 호출은 구독이 존재해야 테스트 가능. Phase 3에서 체크아웃 완료 후 통합 테스트 진행 예정.
