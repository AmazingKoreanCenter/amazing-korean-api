# QA Report — 교재 주문 시스템 (Textbook Order System)

**날짜**: 2026-02-26
**범위**: 교재 주문 시스템 전체 (DB 마이그레이션, Backend 7개 API, Frontend 5개 페이지)
**방법**: 정적 코드 리뷰 + 빌드 검증

---

## 요약

| 카테고리 | 항목 수 | PASS | BUG | 비고 |
|----------|---------|------|-----|------|
| DB 마이그레이션 | 8 | 8 | 0 | |
| Backend Enum (types.rs) | 5 | 5 | 0 | |
| Backend Public API (3 endpoints) | 12 | 12 | 0 | |
| Backend Admin API (4 endpoints) | 10 | 10 | 0 | |
| Backend 라우터 마운팅 | 4 | 4 | 0 | |
| Frontend/Backend DTO 동기화 | 6 | 2 | **4** | **CRITICAL 2, HIGH 1, MEDIUM 1** |
| Frontend Public 페이지 (2 pages) | 10 | 10 | 0 | |
| Frontend Admin 페이지 (3 pages) | 12 | 12 | 0 | |
| Frontend Hooks | 5 | 5 | 0 | |
| Frontend Admin API | 4 | 4 | 0 | |
| 라우팅 (routes.tsx) | 6 | 6 | 0 | |
| i18n (ko/en) | 8 | 8 | 0 | |
| 빌드 검증 | 2 | 2 | 0 | |
| **합계** | **92** | **88** | **4** | |

---

## CRITICAL BUGS (2)

### BUG-1: CatalogItem 필드명 불일치 — `types` vs `available_types`

| 항목 | 값 |
|------|-----|
| 심각도 | **CRITICAL** |
| 백엔드 | `src/api/textbook/dto.rs:19` → `pub types: Vec<TextbookType>` |
| 프론트엔드 | `frontend/src/category/textbook/types.ts:32` → `available_types: z.array(textbookTypeSchema)` |
| 영향 | 프론트엔드가 `available_types` 필드를 읽지만 백엔드는 `types`로 반환 → `undefined` |
| 증상 | 주문 폼에서 교재 유형(학생용/교사용)을 catalog에서 가져올 때 문제 발생 가능 |
| 수정 방안 | 백엔드 `types` → `available_types` 로 rename (프론트엔드 기준 통일) |

### BUG-2: OrderItemRes에 `language_name` 필드 누락

| 항목 | 값 |
|------|-----|
| 심각도 | **CRITICAL** |
| 백엔드 | `src/api/textbook/dto.rs:98-104` → OrderItemRes에 `language_name` 없음 |
| 프론트엔드 | `frontend/src/category/textbook/types.ts:79` → `language_name: z.string()` (필수) |
| 영향 범위 | 3개 페이지에서 `item.language_name` 사용:<br>- `textbook_order_status_page.tsx:228`<br>- `admin_textbook_order_detail.tsx:202`<br>- `admin_textbook_order_print.tsx:120` |
| 증상 | 주문 상세/조회/인쇄 페이지에서 언어 이름이 `undefined`로 표시 |
| 수정 방안 | 백엔드 OrderItemRes에 `language_name: String` 추가 + `build_order_res_from`에서 `TextbookLanguage::to_string()` 매핑 |

---

## HIGH BUGS (1)

### BUG-3: CatalogRes 필드명 불일치 — `min_quantity` vs `min_total_quantity`

| 항목 | 값 |
|------|-----|
| 심각도 | **HIGH** |
| 백엔드 | `src/api/textbook/dto.rs:29` → `pub min_quantity: i32` |
| 프론트엔드 | `frontend/src/category/textbook/types.ts:41` → `min_total_quantity: z.number().int()` |
| 영향 | 프론트엔드에서 최소 수량 기준을 catalog에서 읽을 때 `undefined` |
| 완화 요인 | `textbook_order_page.tsx:88`에서 `MIN_TOTAL_QUANTITY = 10` 하드코딩되어 폼 자체는 동작 |
| 수정 방안 | 백엔드 `min_quantity` → `min_total_quantity` 로 rename |

---

## MEDIUM BUGS (1)

### BUG-4: OrderRes에 `updated_at` 필드 누락

| 항목 | 값 |
|------|-----|
| 심각도 | **MEDIUM** |
| 백엔드 | `src/api/textbook/dto.rs:108-145` → OrderRes에 `updated_at` 없음 |
| 프론트엔드 | `frontend/src/category/textbook/types.ts:116` → `updated_at: z.string()` (필수) |
| 영향 | DB에는 `updated_at` 있고 repo Row에도 있으나, `build_order_res_from`에서 DTO로 매핑하지 않음 |
| 완화 요인 | 현재 어떤 페이지에서도 `updated_at`을 UI에 표시하지 않아 가시적 오류 없음 |
| 수정 방안 | 백엔드 OrderRes에 `updated_at: String` 추가 + `build_order_res_from`에서 매핑 |

---

## PASS 상세

### 1. DB 마이그레이션 (`migrations/20260226_textbook.sql`)

| # | 항목 | 결과 |
|---|------|------|
| 1 | `textbook_language_enum` 20개 값 정의 | PASS |
| 2 | `textbook_type_enum` 2개 값 (student, teacher) | PASS |
| 3 | `textbook_order_status_enum` 7개 값 (pending~canceled) | PASS |
| 4 | `textbook_payment_method_enum` 1개 값 (bank_transfer) | PASS |
| 5 | `textbook` 테이블: 24개 컬럼, PK, DEFAULT, FK 없음 (독립 테이블) | PASS |
| 6 | `textbook_item` 테이블: FK ON DELETE CASCADE, idx_textbook_item_order_id | PASS |
| 7 | `admin_textbook_log` 테이블: FK → textbook, idx 2개 | PASS |
| 8 | 인덱스 5개: status, order_code, created_at, item_order_id, log_order_id | PASS |

### 2. Backend Enum (`src/types.rs`)

| # | 항목 | 결과 |
|---|------|------|
| 1 | TextbookLanguage: 20개 variant, snake_case serde, zh_cn/zh_tw 명시 rename | PASS |
| 2 | TextbookType: Student/Teacher, snake_case serde | PASS |
| 3 | TextbookOrderStatus: 7개 variant, snake_case serde | PASS |
| 4 | TextbookPaymentMethod: BankTransfer → "bank_transfer", snake_case serde | PASS |
| 5 | Display impl for TextbookLanguage (20개 한국어 이름) | PASS |

### 3. Backend Public API

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | GET /textbook/catalog — 인증 불필요 | PASS | handler에 AuthUser 없음 |
| 2 | catalog: 20개 언어 × (Student, Teacher) 반환 | PASS | UNIT_PRICE=25000, KRW |
| 3 | POST /textbook/orders — 비회원 주문 가능 | PASS | handler에 AuthUser 없음 |
| 4 | orders: 최소 수량 검증 (< 10 → BadRequest) | PASS | |
| 5 | orders: 세금계산서 + 사업자번호 누락 → BadRequest | PASS | |
| 6 | orders: 트랜잭션 (insert_order + insert_items) | PASS | tx.commit() |
| 7 | orders: 주문번호 생성 (TB-YYMMDD-NNNN) | PASS | |
| 8 | orders: Validate derive (email, length) | PASS | |
| 9 | GET /textbook/orders/:code — 주문 조회 | PASS | |
| 10 | orders/:code: NotFound 처리 | PASS | .ok_or(AppError::NotFound) |
| 11 | utoipa 3개 핸들러 등록 | PASS | |
| 12 | router: GET /catalog, POST /orders, GET /orders/:code | PASS | |

### 4. Backend Admin API

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | GET /admin/textbook/orders — AuthUser 추출기 | PASS | `_auth: AuthUser` |
| 2 | list_orders: 페이지네이션 (page, size) + clamp(1, 100) | PASS | |
| 3 | list_orders: status + search(ILIKE) 필터 | PASS | |
| 4 | GET /admin/textbook/orders/:id — 상세 조회 | PASS | |
| 5 | PATCH /admin/textbook/orders/:id/status — 상태 변경 | PASS | |
| 6 | status: 상태별 시각 자동 업데이트 (confirmed_at, paid_at 등) | PASS | |
| 7 | status: admin_textbook_log 감사 로그 기록 | PASS | before/after JSON |
| 8 | DELETE /admin/textbook/orders/:id — 삭제 | PASS | StatusCode::NO_CONTENT |
| 9 | delete: 로그 먼저 기록 → CASCADE 삭제 | PASS | |
| 10 | utoipa 4개 핸들러 등록 | PASS | |

### 5. Backend 라우터 마운팅

| # | 항목 | 결과 |
|---|------|------|
| 1 | `src/api/mod.rs` — `pub mod textbook` + `.nest("/textbook", textbook_router())` | PASS |
| 2 | `src/api/admin/mod.rs` — `pub mod textbook` | PASS |
| 3 | `src/api/admin/router.rs` — `.nest("/textbook", admin_textbook_router())` | PASS |
| 4 | Public: 인증 미들웨어 없음, Admin: role_guard 미들웨어 적용 | PASS |

### 6. Frontend Hooks

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | `use_catalog.ts` — useQuery, staleTime 10분 | PASS | |
| 2 | `use_create_order.ts` — useMutation | PASS | |
| 3 | `use_order_by_code.ts` — useQuery, enabled=!!code | PASS | |
| 4 | `use_admin_textbook.ts` — 4개 hooks (list, detail, updateStatus, delete) | PASS | |
| 5 | invalidateQueries on mutation success | PASS | orders key 무효화 |

### 7. Frontend Admin API (`admin_api.ts`)

| # | 항목 | 결과 |
|---|------|------|
| 1 | `getAdminTextbookOrders` — GET /admin/textbook/orders | PASS |
| 2 | `getAdminTextbookOrder` — GET /admin/textbook/orders/:id | PASS |
| 3 | `updateAdminTextbookOrderStatus` — PATCH /admin/textbook/orders/:id/status | PASS |
| 4 | `deleteAdminTextbookOrder` — DELETE /admin/textbook/orders/:id | PASS |

### 8. Frontend Public 페이지

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | `TextbookOrderPage` — PageMeta SEO | PASS | |
| 2 | 7단계 폼 구성 (교재/신청자/기관/배송/결제/세금/비고) | PASS | |
| 3 | useFieldArray 교재 항목 동적 추가/삭제 | PASS | |
| 4 | 최소 수량 프론트엔드 검증 (< 10 → 제출 비활성화) | PASS | |
| 5 | 주문 완료 후 orderResult 상태 전환 | PASS | |
| 6 | 주문번호 클립보드 복사 | PASS | |
| 7 | `TextbookOrderStatusPage` — URL param 코드 조회 | PASS | |
| 8 | Progress Steps 시각화 (6단계, 취소 시 숨김) | PASS | |
| 9 | STATUS_CONFIG: 7개 상태별 아이콘/색상 매핑 | PASS | |
| 10 | 검색 전 초기 상태 UI | PASS | |

### 9. Frontend Admin 페이지

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | `AdminTextbookOrdersPage` — 주문 목록 테이블 | PASS | |
| 2 | 검색 + 상태 필터 SelectUI | PASS | |
| 3 | Pagination 컴포넌트 | PASS | |
| 4 | statusBadgeVariant 7개 상태 매핑 | PASS | |
| 5 | `AdminTextbookOrderDetail` — 상태 변경 Select | PASS | |
| 6 | 삭제 확인 Dialog | PASS | |
| 7 | 견적서/주문확인서 인쇄 링크 (target="_blank") | PASS | |
| 8 | Row/TimeRow 헬퍼 컴포넌트 | PASS | |
| 9 | `AdminTextbookOrderPrint` — print:hidden / @media print | PASS | |
| 10 | 견적서/주문확인서 분기 (searchParams type) | PASS | |
| 11 | window.print() 호출 | PASS | |
| 12 | AdminTextbookMeta 타입 (total_count, total_pages, current_page, per_page) | PASS | |

### 10. 라우팅 (`routes.tsx`)

| # | 항목 | 결과 |
|---|------|------|
| 1 | `/textbook` → TextbookOrderPage (Public) | PASS |
| 2 | `/textbook/order/:code` → TextbookOrderStatusPage (Public) | PASS |
| 3 | `/admin/textbook` → Navigate to `textbook/orders` | PASS |
| 4 | `/admin/textbook/orders` → AdminTextbookOrdersPage | PASS |
| 5 | `/admin/textbook/orders/:orderId` → AdminTextbookOrderDetail | PASS |
| 6 | `/admin/textbook/orders/:orderId/print` → AdminTextbookOrderPrint | PASS |

### 11. i18n (ko.json / en.json)

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | `admin.textbook.*` — 관리자 페이지 키 (50+개) | PASS | ko/en 대칭 |
| 2 | `admin.textbook.status.*` — 7개 상태 라벨 | PASS | |
| 3 | `admin.textbook.print.*` — 인쇄 관련 키 (20+개) | PASS | |
| 4 | `textbook.order.*` — 주문 폼 키 (40+개) | PASS | ko/en 대칭 |
| 5 | `textbook.status.*` — 주문 조회 키 (20+개) | PASS | |
| 6 | `textbook.status.label.*` — 7개 상태 라벨 | PASS | |
| 7 | `textbook.status.step.*` — 6개 진행 단계 | PASS | |
| 8 | `seo.textbook`, `seo.textbookStatus` — SEO 메타 | PASS | |

### 12. 빌드 검증

| # | 항목 | 결과 | 비고 |
|---|------|------|------|
| 1 | `cargo check` | PASS | 0.31s |
| 2 | `npm run build` (tsc + vite) | PASS | 7.97s, chunk size 경고만 (기존) |

---

## 수정 가이드

### BUG-1 수정 (CatalogItem `types` → `available_types`)

```rust
// src/api/textbook/dto.rs
pub struct CatalogItem {
    // ...
    pub available_types: Vec<TextbookType>,  // types → available_types
    // ...
}
```

```rust
// src/api/textbook/service.rs (get_catalog)
CatalogItem {
    // ...
    available_types: vec![TextbookType::Student, TextbookType::Teacher],  // types → available_types
    // ...
}
```

### BUG-2 수정 (OrderItemRes `language_name` 추가)

```rust
// src/api/textbook/dto.rs
pub struct OrderItemRes {
    pub language: TextbookLanguage,
    pub language_name: String,  // 추가
    pub textbook_type: TextbookType,
    pub quantity: i32,
    pub unit_price: i32,
    pub subtotal: i32,
}
```

```rust
// src/api/textbook/service.rs (build_order_res_from)
.map(|i| OrderItemRes {
    language: i.textbook_language,
    language_name: i.textbook_language.to_string(),  // Display impl 활용
    textbook_type: i.textbook_type,
    // ...
})
```

### BUG-3 수정 (CatalogRes `min_quantity` → `min_total_quantity`)

```rust
// src/api/textbook/dto.rs
pub struct CatalogRes {
    pub items: Vec<CatalogItem>,
    pub min_total_quantity: i32,  // min_quantity → min_total_quantity
    pub currency: String,
}
```

```rust
// src/api/textbook/service.rs (get_catalog)
Ok(CatalogRes {
    items,
    min_total_quantity: MIN_TOTAL_QUANTITY,  // min_quantity → min_total_quantity
    currency: "KRW".to_string(),
})
```

### BUG-4 수정 (OrderRes `updated_at` 추가)

```rust
// src/api/textbook/dto.rs (OrderRes에 추가)
pub updated_at: String,

// src/api/textbook/service.rs (build_order_res_from)
updated_at: order.updated_at.to_rfc3339(),
```

---

## 아키텍처 분석

### 잘 된 점
- 트랜잭션 기반 주문 생성 (order + items 원자적)
- 관리자 감사 로그 (admin_textbook_log) — 상태 변경/삭제 전 before/after 기록
- CASCADE 삭제 전 로그 먼저 기록
- 비회원 주문 가능 (Public API에 AuthUser 없음)
- 주문번호 TB-YYMMDD-NNNN 형식 — 일자별 순번
- 상태 변경 시 해당 시각 자동 업데이트 (confirmed_at, paid_at 등)
- i18n 완전 대응 (ko/en, admin/public, status/step/print 모두)
- Print 페이지 `@media print` CSS 적용

### 주의사항
- `generate_order_code` 동시 요청 시 중복 가능성 (COUNT 기반) — 현재 트래픽에서는 무시 가능
- `frontend/src/category/textbook/types.ts`에 Zod 스키마 정의되어 있으나 런타임 파싱에는 미사용 (TypeScript 타입 추론용)
- Badge variant `"warning"`, `"blue"`, `"purple"`, `"success"` — shadcn/ui 커스텀 variant 필요 (기존 프로젝트에 이미 정의된 것으로 추정)
