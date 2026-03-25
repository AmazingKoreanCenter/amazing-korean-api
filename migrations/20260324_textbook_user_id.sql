-- 교재 주문에 사용자 연결 (로그인 필수화)
-- NULLABLE: 기존 비회원 주문 데이터는 user_id = NULL 유지
ALTER TABLE textbook ADD COLUMN user_id BIGINT REFERENCES users(user_id);
CREATE INDEX idx_textbook_user_id ON textbook(user_id);
