-- 세금계산서 발행에 필요한 홈택스 필수/권장 항목 추가
-- 기존: tax_biz_number, tax_email 만 있음
-- 추가: tax_company_name(상호), tax_rep_name(대표자명), tax_address(사업장주소), tax_biz_type(업태), tax_biz_item(종목)

ALTER TABLE textbook
  ADD COLUMN tax_company_name VARCHAR(200),
  ADD COLUMN tax_rep_name     VARCHAR(100),
  ADD COLUMN tax_address      TEXT,
  ADD COLUMN tax_biz_type     VARCHAR(100),
  ADD COLUMN tax_biz_item     VARCHAR(100);

-- 기존 데이터 보정: tax_invoice=true인데 새 필수 필드가 NULL인 행 업데이트
UPDATE textbook SET tax_company_name = '-' WHERE tax_invoice = true AND tax_company_name IS NULL;
UPDATE textbook SET tax_rep_name = '-' WHERE tax_invoice = true AND tax_rep_name IS NULL;

-- 세금계산서 요청 시 상호 + 대표자명 필수 (홈택스 필수 항목)
ALTER TABLE textbook
  ADD CONSTRAINT chk_tax_company_name CHECK (tax_invoice = false OR tax_company_name IS NOT NULL),
  ADD CONSTRAINT chk_tax_rep_name     CHECK (tax_invoice = false OR tax_rep_name IS NOT NULL);
