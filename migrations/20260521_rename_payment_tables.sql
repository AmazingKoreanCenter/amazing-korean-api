-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ③ payment 무접두사+복수형 정정
-- SSoT: AMK_API_MASTER §3.2.1 (도메인 접두사 필수 + 단수형). 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- subscriptions/transactions/webhook_events = payment 도메인 무접두사 + 복수형 (이중 위반).
-- 전부 메타데이터 RENAME (데이터 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 제약/인덱스 = 존재 가드(환경 독립). unique_provider_subscription / unique_provider_event
--   = 토큰('subscriptions'/'webhook_events') 미포함이라 미변경.

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE subscriptions  RENAME TO payment_subscription;
ALTER TABLE transactions   RENAME TO payment_transaction;
ALTER TABLE webhook_events RENAME TO payment_webhook_event;

-- ── 인덱스 (토큰 포함분만, IF EXISTS = 환경 독립; user_status=20260504 prod만) ──
ALTER INDEX IF EXISTS idx_subscriptions_status         RENAME TO idx_payment_subscription_status;
ALTER INDEX IF EXISTS idx_subscriptions_user_id        RENAME TO idx_payment_subscription_user_id;
ALTER INDEX IF EXISTS idx_subscriptions_user_status    RENAME TO idx_payment_subscription_user_status;
ALTER INDEX IF EXISTS idx_transactions_provider_txn_id RENAME TO idx_payment_transaction_provider_txn_id;
ALTER INDEX IF EXISTS idx_transactions_subscription_id RENAME TO idx_payment_transaction_subscription_id;
ALTER INDEX IF EXISTS idx_transactions_user_id         RENAME TO idx_payment_transaction_user_id;
ALTER INDEX IF EXISTS idx_webhook_events_type          RENAME TO idx_payment_webhook_event_type;

-- ── 제약 (pkey/fk, 존재 확인 후만 RENAME) ───────────────────────────────
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT * FROM (VALUES
    ('payment_subscription',  'subscriptions_pkey',                'payment_subscription_pkey'),
    ('payment_subscription',  'subscriptions_user_id_fkey',        'payment_subscription_user_id_fkey'),
    ('payment_transaction',   'transactions_pkey',                 'payment_transaction_pkey'),
    ('payment_transaction',   'transactions_user_id_fkey',         'payment_transaction_user_id_fkey'),
    ('payment_transaction',   'transactions_subscription_id_fkey', 'payment_transaction_subscription_id_fkey'),
    ('payment_webhook_event', 'webhook_events_pkey',               'payment_webhook_event_pkey')
  ) AS t(tbl, oldn, newn)
  LOOP
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = r.oldn AND conrelid = r.tbl::regclass) THEN
      EXECUTE format('ALTER TABLE %I RENAME CONSTRAINT %I TO %I', r.tbl, r.oldn, r.newn);
    END IF;
  END LOOP;
END $$;
