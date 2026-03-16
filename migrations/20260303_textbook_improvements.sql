-- Textbook Order System Improvements (2026-03-03)
-- CRITICAL: Soft Delete, CASCADE → RESTRICT, 배송 추적 필드, 인덱스 추가

-- =============================================================================
-- 1. Soft Delete 컬럼 추가 (한국 세법 5년 보관 의무 대응)
-- =============================================================================
ALTER TABLE textbook ADD COLUMN is_deleted BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE textbook ADD COLUMN deleted_at TIMESTAMPTZ;

-- =============================================================================
-- 2. 배송 추적 필드 추가
-- =============================================================================
ALTER TABLE textbook ADD COLUMN tracking_number VARCHAR(100);
ALTER TABLE textbook ADD COLUMN tracking_provider VARCHAR(50);

-- =============================================================================
-- 3. admin_textbook_log: CASCADE → RESTRICT 변경
--    (주문 삭제 시 감사 로그 보존)
-- =============================================================================
ALTER TABLE admin_textbook_log
    DROP CONSTRAINT admin_textbook_log_order_id_fkey,
    ADD CONSTRAINT admin_textbook_log_order_id_fkey
        FOREIGN KEY (order_id) REFERENCES textbook(order_id) ON DELETE RESTRICT;

-- =============================================================================
-- 4. 세금계산서 필드 무결성 (DB 레벨 CHECK)
-- =============================================================================
ALTER TABLE textbook ADD CONSTRAINT check_tax_fields
    CHECK (NOT tax_invoice OR (tax_biz_number IS NOT NULL AND tax_email IS NOT NULL));

-- =============================================================================
-- 5. 추가 인덱스
-- =============================================================================

-- 관리자 목록: status + created_at 복합 인덱스
CREATE INDEX idx_textbook_status_created ON textbook(status, created_at DESC);

-- 이메일 검색용 인덱스
CREATE INDEX idx_textbook_orderer_email ON textbook(orderer_email);

-- Soft delete 필터링
CREATE INDEX idx_textbook_not_deleted ON textbook(is_deleted) WHERE is_deleted = false;
