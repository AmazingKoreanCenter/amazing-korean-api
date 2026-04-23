-- 2026-04-23: textbook 주문에 할인 필드 추가 (#73 후속)
--
-- 관리자가 대리 주문 생성 시 임의 할인을 적용할 수 있도록 세 컬럼 추가.
-- 세법 정확 표기 위해 gross_amount (할인 전) / discount_amount (할인) /
-- total_amount (할인 후) 분리 저장. 영수증에서 공급가액·VAT 계산 시
-- total_amount 기준 (할인 후 과세표준).
--
-- 관계: total_amount = gross_amount - discount_amount
-- 기존 주문 백필: discount_amount = 0 이므로 gross_amount = total_amount.
--
-- 결정 이력:
--   - 1 (B): 세 필드 저장 (gross + discount + total) — 영수증 세법 표기 + 감사 추적성
--   - 2 (B): 주문 상세 페이지에서도 편집 가능 — 별도 UPDATE 경로 service 에 구현
--   - 3 (A): 세법 정확 — 할인 반영 후 과세표준, VAT 재계산

BEGIN;

ALTER TABLE textbook
    ADD COLUMN gross_amount INT,
    ADD COLUMN discount_amount INT NOT NULL DEFAULT 0,
    ADD COLUMN discount_reason TEXT;

-- 기존 주문은 할인이 없었으므로 할인 전 금액 = 현재 total_amount
UPDATE textbook SET gross_amount = total_amount WHERE gross_amount IS NULL;

ALTER TABLE textbook ALTER COLUMN gross_amount SET NOT NULL;

-- discount_amount 는 0 이상이고 gross_amount 를 초과할 수 없음 (DB 레벨 가드).
-- service 레이어에서도 중복 검증.
ALTER TABLE textbook
    ADD CONSTRAINT textbook_discount_amount_nonneg CHECK (discount_amount >= 0),
    ADD CONSTRAINT textbook_discount_not_exceed_gross CHECK (discount_amount <= gross_amount),
    ADD CONSTRAINT textbook_total_equals_gross_minus_discount CHECK (total_amount = gross_amount - discount_amount);

COMMENT ON COLUMN textbook.gross_amount IS '할인 전 총액 (수량 × 단가, VAT 포함). 영수증 "품목 합계" 표시용';
COMMENT ON COLUMN textbook.discount_amount IS '할인 금액 (VAT 포함). 관리자 임의 입력. 0 ≤ discount ≤ gross';
COMMENT ON COLUMN textbook.discount_reason IS '할인 사유 (관리자 메모). 영수증에는 선택적 표시';

COMMIT;
