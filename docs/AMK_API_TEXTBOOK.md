# AMK_API_TEXTBOOK — 교재 주문 API 스펙

> 비회원 교재 주문 시스템 (계좌이체, 20언어 × 2종).
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.12 Phase 12 — 교재 주문 (Textbook Ordering)

> 비회원 교재 주문 시스템. 계좌이체 기반, 20개 언어 × 2종(학생용/교사용), ₩25,000/권, 최소 10권.
> 마이그레이션: `migrations/20260226_textbook.sql`, `migrations/20260303_textbook_improvements.sql`

<details>
<summary>📋 Textbook 엔드포인트 상세 (클릭)</summary>

#### 12-1 : `GET /textbook/catalog` (교재 카탈로그)

> 주문 가능한 교재 목록과 가격 정보를 반환.

**인증**: 불필요

**응답 (성공 200)**
```json
{
  "items": [
    {
      "language": "ja",
      "language_name_ko": "일본어",
      "language_name_en": "Japanese",
      "available_types": ["student", "teacher"],
      "unit_price": 25000,
      "available": true,
      "isbn_ready": true
    }
  ],
  "currency": "KRW",
  "min_total_quantity": 10
}
```

#### 12-2 : `POST /textbook/orders` (주문 생성)

> 교재 주문 접수. 로그인 필수 (2026-03-24 변경).

**인증**: Bearer JWT 필수 (`user_id`가 주문에 연결됨)

**요청**
```json
{
  "orderer_name": "홍길동",
  "orderer_email": "hong@example.com",
  "orderer_phone": "010-1234-5678",
  "org_name": "한국어학원",
  "org_type": "academy",
  "delivery_postal_code": "06234",
  "delivery_address": "서울특별시 강남구 ...",
  "delivery_detail": "3층",
  "payment_method": "bank_transfer",
  "depositor_name": "홍길동",
  "tax_invoice": false,
  "items": [
    { "language": "ja", "textbook_type": "student", "quantity": 10 }
  ],
  "notes": "빠른 배송 부탁드립니다"
}
```

**검증 규칙**:
- 총 수량 ≥ 10
- 각 항목 수량: 1~9999
- 중복 항목 거부 (같은 language + textbook_type 조합 불가)
- 비활성 언어 주문 차단 (카탈로그 `available=false`)
- `isbn_ready`: ISBN 발급 완료 여부. false인 언어는 ISBN 발급 후 인쇄 → 배송 약 1주 추가 소요. 발급 완료 9개: ja, zh_cn, vi, th, ne, ru, km, tl, id
- tax_invoice=true일 때 tax_biz_number + tax_email 필수
- IP 기반 Rate Limiting (Redis, 기본 5회/시간)

**프론트엔드 약관 동의**: 주문 제출 전 약관 동의 모달 표시 (6개 조항 — 주문 접수, 결제, 배송, 교환/반품, 개인정보, 기타). 동의 체크 후 제출 가능.

**응답 (성공 201)**: OrderRes (주문 상세 + 항목)

#### 12-2.5 : `GET /textbook/my` (내 주문 목록) — 2026-03-24 추가

> 로그인한 사용자의 교재 주문 목록 조회.

**인증**: Bearer JWT 필수

**응답 (성공 200)**
```json
{
  "orders": [
    { "order_id": 1, "order_code": "TB-260324-0001", "status": "pending", "items": [...], "total_quantity": 10, "total_amount": 250000, ... }
  ]
}
```

#### 12-3 : `GET /textbook/orders/{code}` (주문 조회)

> 주문번호(order_code)로 주문 상태 조회. 비회원도 조회 가능.

**인증**: 불필요

**응답 (성공 200)**: OrderRes

#### 12-4 : `GET /admin/textbook/orders` (관리자 주문 목록)

> 교재 주문 목록 조회. 상태 필터, 검색, 페이지네이션 지원.

**인증**: Admin (IP Guard + Role Guard)

**쿼리 파라미터**: `page`, `size`, `q` (주문번호/신청자/기관 검색), `status`

**응답 (성공 200)**
```json
{
  "items": [OrderRes],
  "meta": { "total_count": 42, "total_pages": 3, "current_page": 1, "per_page": 20 }
}
```

#### 12-5 : `GET /admin/textbook/orders/{id}` (관리자 주문 상세)

**인증**: Admin

**응답 (성공 200)**: OrderRes

#### 12-6 : `PATCH /admin/textbook/orders/{id}/status` (상태 변경)

> 주문 상태 변경 + admin_textbook_log에 변경 이력 기록 + 고객 이메일 알림 발송.
> 상태 전환은 State Machine 규칙을 따름 (유효하지 않은 전환 시 400 반환).
> Shipped 전환 시 tracking_number 필수 (없으면 400 반환).

**인증**: Admin

**요청**: `{ "status": "confirmed" }`

**상태 전환 규칙**:
```
pending → confirmed → paid → printing → shipped → delivered (정방향)
pending/confirmed/paid/printing/shipped → canceled (취소)
delivered/canceled → (전환 불가)
```

**응답 (성공 200)**: OrderRes

#### 12-7 : `PATCH /admin/textbook/orders/{id}/tracking` (배송 추적 정보 업데이트)

> 배송 추적번호/택배사 정보 업데이트 + admin_textbook_log 기록.

**인증**: Admin

**요청**: `{ "tracking_number": "1234567890", "tracking_provider": "CJ대한통운" }`

**응답 (성공 200)**: OrderRes

#### 12-8 : `DELETE /admin/textbook/orders/{id}` (주문 삭제)

> Soft Delete (is_deleted = true, deleted_at = NOW()). admin_textbook_log에 삭제 이력 기록.
> FK 제약조건: RESTRICT (감사 로그 보존을 위해 물리 삭제 불가).

**인증**: Admin

**응답**: `204 No Content`

</details>

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
