-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ③ payment 무접두사+복수형 정정
-- SSoT: AMK_API_MASTER §3.2.1 (도메인 접두사 필수 + 단수형). 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- subscriptions/transactions/webhook_events = payment 도메인 무접두사 + 복수형 (이중 위반).
-- 전부 메타데이터 RENAME (데이터 이동 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 풀 정합: 토큰(subscriptions/transactions/webhook_events) 포함 객체만.
-- unique_provider_subscription / unique_provider_event = 토큰('subscriptions'/'webhook_events')
--   미포함(단수 'subscription'/'provider_event')이라 미변경(그룹 ①② 동일 mechanical 규칙).

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE subscriptions  RENAME TO payment_subscription;
ALTER TABLE transactions   RENAME TO payment_transaction;
ALTER TABLE webhook_events RENAME TO payment_webhook_event;

-- ── 인덱스 (토큰 포함분만) ───────────────────────────────────────────────
ALTER INDEX idx_subscriptions_status        RENAME TO idx_payment_subscription_status;
ALTER INDEX idx_subscriptions_user_id       RENAME TO idx_payment_subscription_user_id;
-- 20260504 복합 인덱스: prod 적용·로컬 비-head 미적용 → IF EXISTS (마이그 상태 분기 안전)
ALTER INDEX IF EXISTS idx_subscriptions_user_status RENAME TO idx_payment_subscription_user_status;
ALTER INDEX idx_transactions_provider_txn_id RENAME TO idx_payment_transaction_provider_txn_id;
ALTER INDEX idx_transactions_subscription_id RENAME TO idx_payment_transaction_subscription_id;
ALTER INDEX idx_transactions_user_id         RENAME TO idx_payment_transaction_user_id;
ALTER INDEX idx_webhook_events_type          RENAME TO idx_payment_webhook_event_type;

-- ── 제약 (pkey/fk, 토큰 포함분만. pkey 리네임 = 백킹 인덱스 동반) ─────────
ALTER TABLE payment_subscription  RENAME CONSTRAINT subscriptions_pkey                 TO payment_subscription_pkey;
ALTER TABLE payment_subscription  RENAME CONSTRAINT subscriptions_user_id_fkey         TO payment_subscription_user_id_fkey;
ALTER TABLE payment_transaction   RENAME CONSTRAINT transactions_pkey                  TO payment_transaction_pkey;
ALTER TABLE payment_transaction   RENAME CONSTRAINT transactions_user_id_fkey          TO payment_transaction_user_id_fkey;
ALTER TABLE payment_transaction   RENAME CONSTRAINT transactions_subscription_id_fkey  TO payment_transaction_subscription_id_fkey;
ALTER TABLE payment_webhook_event RENAME CONSTRAINT webhook_events_pkey                TO payment_webhook_event_pkey;
