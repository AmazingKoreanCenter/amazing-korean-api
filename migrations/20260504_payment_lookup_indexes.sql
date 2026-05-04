-- N-28/N-29/N-30: 결제 조회 성능 인덱스 추가 (AMK_AUDIT_2026-05-04 §N-28~30)
-- 작성일: 2026-05-04
-- 영향: 데이터 누적 시 점진적 성능 개선. 기존 데이터 영향 X (인덱스만 추가).
-- 멱등성: IF NOT EXISTS 사용.

-- N-28: ebook_purchase(paddle_txn_id) 단일 인덱스
-- 사용 위치: src/api/ebook/repo.rs (refund_by_paddle_txn / complete_with_paddle_txn 내 UPDATE WHERE paddle_txn_id)
CREATE INDEX IF NOT EXISTS idx_ebook_purchase_paddle_txn_id
  ON ebook_purchase(paddle_txn_id);

-- N-29: subscriptions(user_id, status) 복합 인덱스
-- 사용 위치: src/api/payment/repo.rs (활성 구독 조회 WHERE user_id = $1 AND status = 'active')
-- 기존 분리 인덱스 (idx_subscriptions_user_id + idx_subscriptions_status) 결합보다 효율적
CREATE INDEX IF NOT EXISTS idx_subscriptions_user_status
  ON subscriptions(user_id, status);

-- N-30: ebook_purchase(user_id, language, edition) 복합 인덱스
-- 사용 위치: src/api/ebook/repo.rs (중복 구매 확인 WHERE user_id = $1 AND language = $2 AND edition = $3 AND status IN (...))
CREATE INDEX IF NOT EXISTS idx_ebook_purchase_user_lang_edition
  ON ebook_purchase(user_id, language, edition);
