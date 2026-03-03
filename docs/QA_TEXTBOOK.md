# QA Report — 교재 주문 시스템 (Textbook Order System)

**날짜**: 2026-03-03 (Phase 2 라이브 테스트 업데이트)
**범위**: 교재 주문 시스템 개선 (Rate Limiting, State Machine, Tracking, Soft Delete, Email, N+1 Fix)
**방법**: 로컬 서버 기동 + curl API 테스트 + DB 직접 검증

---

## 요약

| 카테고리 | 항목 수 | PASS | BUG (수정) | 비고 |
|----------|---------|------|------------|------|
| Public API — 정상 흐름 | 3 | 3 | 0 | catalog, create_order, order_by_code |
| Public API — Validation | 6 | 6 | 0 | min_qty, duplicate, tax, 404, email, tax_email |
| Rate Limiting | 2 | 2 | 0 | Redis INCR + EXPIRE, 429 반환 |
| Admin Auth | 1 | 1 | 0 | 401 Unauthorized |
| Admin List/Detail | 2 | 2 | 0 | 목록 + 상세 조회 |
| State Machine — 정방향 | 6 | 6 | 0 | pending→confirmed→paid→printing→shipped→delivered |
| State Machine — 차단 | 5 | 5 | 0 | 역방향, 터미널 상태 차단 |
| Tracking — 필수 검증 | 2 | 2 | 0 | tracking 없이 shipped 차단 + 업데이트 |
| Soft Delete | 4 | 4 | 0 | 삭제, Admin 404, Public 404, DB 확인 |
| Search + Filter | 5 | 5 | 0 | q=, status=, 조합, 특수문자, 빈 검색 |
| Pagination | 2 | 2 | 0 | size=1, page=2 |
| 404 Edge Cases | 3 | 3 | 0 | 없는 주문 status/delete/tracking |
| Audit Log | 1 | 1 | 0 | 12건 로그, before/after 정확 |
| **버그 발견 + 즉시 수정** | **1** | **1** | **0** | update_status 중복 컬럼 |
| **합계** | **43** | **43** | **0** | 버그 1건 발견 → 즉시 수정 후 PASS |

---

## 발견 + 즉시 수정된 버그

### BUG-FIX: `update_status` — PostgreSQL 중복 컬럼 할당 오류

| 항목 | 값 |
|------|-----|
| 심각도 | **CRITICAL** |
| 파일 | `src/api/textbook/repo.rs:349-361` |
| 증상 | `paid→printing` 전환 시 500 Internal Server Error |
| 원인 | `Printing`이 `_ =>` match arm으로 빠져 `col = "updated_at"` 설정 → SQL에 `updated_at` 컬럼이 2번 SET됨 → PostgreSQL "multiple assignments to same column" |
| 문제 SQL | `UPDATE textbook SET status=$1, updated_at=COALESCE($3,updated_at), updated_at=NOW() ...` |
| 수정 | `timestamp_col: Option<&str>` 패턴으로 분기 — 타임스탬프 있으면 3-bind 쿼리, 없으면 2-bind 쿼리 |
| 검증 | 수정 후 `paid→printing` 전환 200 OK 확인 |

---

## 테스트 상세

### 1. Public API — 정상 흐름

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 1 | GET /textbook/catalog | PASS | 20개 언어, available_types:[student,teacher], min_total_quantity:10 |
| 2 | POST /textbook/orders (정상 주문) | PASS | TB-260303-0001, language_name 포함, tracking 필드, updated_at |
| 3 | GET /textbook/orders/TB-260303-0001 | PASS | 주문 상세 정상 반환 |

### 2. Public API — Validation

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 4 | 총 수량 < 10 | PASS | 400 "Minimum total quantity is 10 copies" |
| 5 | 동일 language+type 중복 항목 | PASS | 400 "Duplicate item: Vi Student" |
| 6 | 세금계산서 + 사업자번호 누락 | PASS | 400 "Business registration number required" |
| 7 | 존재하지 않는 주문 코드 조회 | PASS | 404 |
| 8 | 세금계산서 + 이메일 누락 | PASS | 400 "Tax invoice email is required" |
| 9 | 잘못된 이메일 형식 | PASS | 400 validator error |

### 3. Rate Limiting

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 10 | 5회 초과 POST → 429 | PASS | "TEXTBOOK_429_TOO_MANY_ORDERS" |
| 11 | Redis key 확인 (rl:textbook_order:127.0.0.1) | PASS | INCR + TTL 동작 |

### 4. Admin API — Auth + CRUD

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 12 | JWT 없이 접근 → 401 | PASS | |
| 13 | 주문 목록 조회 | PASS | 3건, meta 정상 |
| 14 | 주문 상세 조회 | PASS | tracking, updated_at 포함 |

### 5. State Machine — 정방향 전이

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 15 | pending → confirmed | PASS | confirmed_at 설정됨 |
| 16 | confirmed → paid | PASS | paid_at 설정됨 |
| 17 | paid → printing | PASS | **버그 수정 후 PASS** |
| 18 | printing → shipped (tracking 있음) | PASS | shipped_at 설정됨 |
| 19 | shipped → delivered | PASS | delivered_at 설정됨 |
| 20 | pending → canceled | PASS | canceled_at 설정됨 |

### 6. State Machine — 차단

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 21 | delivered → canceled | PASS | 400 "Invalid status transition" |
| 22 | delivered → pending | PASS | 400 |
| 23 | canceled → pending | PASS | 400 |
| 24 | canceled → confirmed | PASS | 400 |
| 25 | confirmed → shipped (건너뛰기) | PASS | 400 (이전 세션) |

### 7. Tracking 필수 검증

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 26 | printing → shipped (tracking 없음) | PASS | 400 "Tracking number is required" |
| 27 | PATCH /tracking 업데이트 | PASS | tracking_number, tracking_provider 반영 |

### 8. Soft Delete

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 28 | DELETE /admin/textbook/orders/4 | PASS | 204 No Content |
| 29 | Admin 조회 삭제된 주문 → 404 | PASS | |
| 30 | Public 조회 삭제된 주문 → 404 | PASS | |
| 31 | DB: is_deleted=true, deleted_at IS NOT NULL | PASS | |

### 9. Search + Filter

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 32 | q=QA (orderer_name ILIKE) | PASS | 1건 매칭 |
| 33 | q=TB-260303-0001 (order_code ILIKE) | PASS | 1건 매칭 |
| 34 | q=테스트학원 (org_name ILIKE) | PASS | 1건 매칭 |
| 35 | status=delivered 필터 | PASS | delivered만 반환 |
| 36 | q=QA&status=delivered 조합 | PASS | 1건 매칭 |
| 37 | q=100% (ILIKE 특수문자 이스케이프) | PASS | 0건 (% 이스케이프 동작) |
| 38 | q=a_b (_ 이스케이프) | PASS | 0건 |

### 10. Pagination

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 39 | size=1, page=1 | PASS | items 1개, total_pages=3 |
| 40 | size=1, page=2 | PASS | 두 번째 주문 반환 |

### 11. 404 Edge Cases

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 41 | 없는 주문 상태 변경 (id=99999) | PASS | 404 |
| 42 | 없는 주문 삭제 (id=99999) | PASS | 404 |
| 43 | 없는 주문 tracking 업데이트 (id=99999) | PASS | 404 |

### 12. Audit Log

| # | 테스트 | 결과 | 비고 |
|---|--------|------|------|
| 44 | admin_textbook_log 12건 기록 확인 | PASS | 상태 변경(8), 추적 업데이트(2), 삭제(1) + 추가(1) |

---

## 테스트 환경

- **서버**: `cargo run` (localhost:3000)
- **DB**: Docker PostgreSQL (amk-pg)
- **Redis**: Docker Redis (amk-redis)
- **인증**: HS256 JWT (admin, user_id=20, exp=2026-03-03)
- **테스트 데이터**: 4개 주문 생성 (order_id 1~4)

## 테스트 데이터 상태 (최종)

| order_id | order_code | status | is_deleted |
|----------|------------|--------|------------|
| 1 | TB-260226-0001 | pending | false |
| 2 | TB-260303-0001 | delivered | false |
| 3 | TB-260303-0002 | canceled | false |
| 4 | TB-260303-0003 | printing | true (soft deleted) |

---

## 이전 QA 결과 (Phase 1 — 정적 코드 리뷰, 2026-02-26)

Phase 1에서 발견된 4개 DTO 불일치 버그는 모두 수정 완료:
- BUG-1: CatalogItem `types` → `available_types` (수정됨)
- BUG-2: OrderItemRes `language_name` 누락 (추가됨)
- BUG-3: CatalogRes `min_quantity` → `min_total_quantity` (수정됨)
- BUG-4: OrderRes `updated_at` 누락 (추가됨)
