-- 모바일 IAP 지원: payment_provider_enum, ebook_payment_method_enum 확장 + ebook_purchase IAP 컬럼

-- ALTER TYPE ADD VALUE는 트랜잭션 내 실행 불가 — sqlx는 파일 단위 실행이므로 문제 없음
ALTER TYPE payment_provider_enum ADD VALUE IF NOT EXISTS 'apple';
ALTER TYPE payment_provider_enum ADD VALUE IF NOT EXISTS 'google';
ALTER TYPE payment_provider_enum ADD VALUE IF NOT EXISTS 'revenuecat';

ALTER TYPE ebook_payment_method_enum ADD VALUE IF NOT EXISTS 'apple_iap';
ALTER TYPE ebook_payment_method_enum ADD VALUE IF NOT EXISTS 'google_iap';

-- IAP 구매 추적 컬럼
ALTER TABLE ebook_purchase
  ADD COLUMN IF NOT EXISTS iap_platform VARCHAR(10),
  ADD COLUMN IF NOT EXISTS iap_transaction_id VARCHAR(255),
  ADD COLUMN IF NOT EXISTS iap_product_id VARCHAR(255);
