-- ============================================================
-- E-book 구매코드 형식 변경 — 2026-03-12
-- EB-YYMMDD-NNNN (20자) → {LANG}-{ED}-{YYYYMMDD}-{PAY}-{NNNN} (30자)
-- 예: VN-ST-20260310-CA-0001
-- ============================================================

ALTER TABLE ebook_purchase ALTER COLUMN purchase_code TYPE VARCHAR(30);
