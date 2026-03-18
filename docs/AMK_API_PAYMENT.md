# AMK_API_PAYMENT — 결제 API 스펙

> Paddle Billing 기반 구독/결제 시스템 (관리자 + 사용자).
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

<details>
<summary><strong>5.10 Phase 10 — 관리자 결제/구독 관리 + 수동 수강권 ✅</strong></summary>

> 관리자가 구독/트랜잭션을 조회·관리하고, Paddle 없이 수동으로 수강권을 부여/회수할 수 있는 어드민 기능.

#### 10-1 : `GET /admin/payment/subscriptions` (구독 목록)

> 관리자가 전체 구독 목록을 조회한다. 이메일 검색, 상태 필터, 정렬, 페이지네이션 지원.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |
| `status` | string | N | 상태 필터 (trialing/active/past_due/paused/canceled) |
| `sort` | string | N | 정렬 기준 (id/created_at/status/billing_interval/price) |
| `order` | string | N | 정렬 방향 (asc/desc) |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "subscription_id": 1,
      "user_id": 42,
      "user_email": "user@example.com",
      "status": "active",
      "billing_interval": "month_3",
      "current_price_cents": 2500,
      "current_period_end": "2026-05-15T00:00:00Z",
      "created_at": "2026-02-15T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 50, "total_pages": 3 }
}
```

---

#### 10-2 : `GET /admin/payment/subscriptions/{id}` (구독 상세)

> 구독 상세 정보 + 사용자 정보 + 관련 트랜잭션 내역을 함께 반환.

**응답 (성공 200)**
```json
{
  "subscription": {
    "subscription_id": 1,
    "user_id": 42,
    "provider": "paddle",
    "provider_subscription_id": "sub_01...",
    "provider_customer_id": "ctm_01...",
    "status": "active",
    "billing_interval": "month_3",
    "current_price_cents": 2500,
    "currency": "USD",
    "current_period_start": "2026-02-15T00:00:00Z",
    "current_period_end": "2026-05-15T00:00:00Z",
    "trial_ends_at": null,
    "canceled_at": null,
    "paused_at": null,
    "created_at": "2026-02-15T00:00:00Z",
    "updated_at": "2026-02-15T00:00:00Z"
  },
  "user": {
    "user_id": 42,
    "email": "user@example.com",
    "nickname": "korean_learner",
    "user_auth": "LEARNER"
  },
  "transactions": [
    {
      "transaction_id": 1,
      "status": "completed",
      "amount_cents": 2500,
      "tax_cents": 250,
      "currency": "USD",
      "occurred_at": "2026-02-15T00:00:00Z"
    }
  ]
}
```

---

#### 10-3 : `POST /admin/payment/subscriptions/{id}/cancel` (관리자 구독 취소)

> 관리자가 사용자의 구독을 취소한다. Paddle API 호출 후 감사 로그 기록.

**요청 Body**
```json
{ "immediately": true }
```

- `immediately: true` → 즉시 취소
- `immediately: false` → 다음 결제일에 취소

**응답**: `200 OK` (빈 JSON)

---

#### 10-4 : `GET /admin/payment/transactions` (트랜잭션 목록)

> 전체 트랜잭션 목록 조회. 이메일 검색, 상태 필터, 정렬, 페이지네이션.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |
| `status` | string | N | 상태 필터 (completed/refunded) |
| `sort` | string | N | 정렬 기준 (id/occurred_at/status/amount) |
| `order` | string | N | 정렬 방향 (asc/desc) |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "transaction_id": 1,
      "subscription_id": 1,
      "user_id": 42,
      "user_email": "user@example.com",
      "status": "completed",
      "amount_cents": 2500,
      "tax_cents": 250,
      "currency": "USD",
      "billing_interval": "month_3",
      "occurred_at": "2026-02-15T00:00:00Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 10, "total_pages": 1 }
}
```

---

#### 10-5 : `POST /admin/payment/grants` (수동 수강권 부여)

> Paddle 구독 없이 관리자가 직접 사용자에게 수강권을 부여한다 (VIP, CS 대응, 이벤트 등).

**요청 Body**
```json
{
  "user_id": 42,
  "expire_at": "2026-12-31T23:59:59Z",
  "reason": "VIP 사용자 수동 부여"
}
```

- `expire_at`: 선택. null이면 무기한.
- `reason`: 필수. 감사 로그에 기록.

**응답 (성공 201)**
```json
{
  "user_id": 42,
  "courses_granted": 5,
  "expire_at": "2026-12-31T23:59:59Z",
  "granted_by": 1,
  "reason": "VIP 사용자 수동 부여",
  "created_at": "2026-02-16T00:00:00Z"
}
```

---

#### 10-6 : `GET /admin/payment/grants` (수동 부여 내역 조회)

> 구독 없이 수강권이 활성화된 사용자 목록 조회.

**Query Parameters**
| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| `page` | i64 | N | 페이지 번호 (기본 1) |
| `size` | i64 | N | 페이지 크기 (기본 20, 최대 100) |
| `q` | string | N | 이메일/닉네임 검색 |

**응답 (성공 200)**
```json
{
  "items": [
    {
      "user_id": 42,
      "user_email": "user@example.com",
      "user_nickname": "korean_learner",
      "active_courses": 5,
      "earliest_enrolled": "2026-01-01T00:00:00Z",
      "latest_expire": "2026-12-31T23:59:59Z"
    }
  ],
  "meta": { "page": 1, "size": 20, "total_count": 3, "total_pages": 1 }
}
```

---

#### 10-7 : `DELETE /admin/payment/grants/{userId}` (수동 수강권 회수)

> 사용자의 모든 수강권을 회수한다.

**응답**: `204 No Content`

</details>

---

<details>
<summary><strong>5.11 Phase 11 — 사용자 결제 (Paddle Billing) ✅</strong></summary>

> Paddle Billing 기반 구독 결제. 플랜 조회, 구독 상태 확인, Webhook 수신.

#### 11-1 : `GET /payment/plans` (플랜 목록)

> 공개 엔드포인트. 구독 플랜 목록 + Paddle Client Token 반환.

**인증**: 불필요 (공개)

**응답 (성공 200)**
```json
{
  "plans": [
    {
      "price_id": "pri_01khg4rcvq9ewz1n1rs9zd59rp",
      "interval": "month_1",
      "price_cents": 1000,
      "currency": "USD",
      "label": "1 Month"
    }
  ],
  "client_token": "test_53998ff59a87110b9c389e35880",
  "sandbox": true
}
```

---

#### 11-2 : `GET /payment/subscription` (내 구독 상태)

> 인증된 사용자의 현재 구독 정보 조회. 구독이 없으면 404.

**인증**: Bearer Token (필수)

**응답 (성공 200)**
```json
{
  "subscription_id": 1,
  "status": "active",
  "billing_interval": "month_3",
  "current_price_cents": 2500,
  "currency": "USD",
  "current_period_start": "2026-02-15T00:00:00Z",
  "current_period_end": "2026-05-15T00:00:00Z",
  "trial_ends_at": null,
  "canceled_at": null,
  "paused_at": null,
  "created_at": "2026-02-15T00:00:00Z",
  "management_urls": {
    "cancel": "https://...",
    "update_payment_method": "https://..."
  }
}
```

---

#### 11-3 : `POST /payment/webhook` (Paddle Webhook)

> Paddle에서 호출하는 Webhook 엔드포인트. 서명 검증 후 이벤트 처리.

**인증**: Paddle HMAC-SHA256 서명 검증 (Paddle-Signature 헤더)

**처리 이벤트**: subscription.created/activated/updated/canceled/paused/resumed/trialing/past_due, transaction.completed

**응답**: `200 OK` (항상)

</details>

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
