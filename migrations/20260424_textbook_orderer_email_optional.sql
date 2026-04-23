-- 2026-04-23: textbook 주문 orderer_email 필수 해제 (#73 후속)
--
-- 파일명 주의: 날짜는 2026-04-23 이지만 파일명 prefix 는 `20260424` 로
-- 설정. 같은 날 이미 `20260423_textbook_order_discount.sql` 이 프로덕션에
-- 적용된 상태에서, 동일 `20260423` 버전으로 두 번째 파일을 추가하면 sqlx
-- migrator 가 체크섬 불일치로 crash loop. 교훈 → 하루에 2개 이상
-- 마이그레이션이 필요하면 두 번째부터는 다음 날짜 or 버전 suffix 사용.
--
-- 배경: 오프라인·전화 주문 대리 입력 시 이메일 수집이 어려운 경우가 있음.
-- 관리자 대리 주문 생성 UX 개선 과정에서 이메일 필수 요구가 현실에 맞지 않음
-- 확인. `NOT NULL` → NULL 허용으로 완화.
--
-- 관련:
--   - 사용자 일반 주문 (POST /textbook/orders) 는 UI 에서 여전히 필수 유지
--     (확인 메일 발송 필요)
--   - 관리자 대리 주문 (POST /admin/textbook/orders) 은 optional
--   - service 의 이메일 발송 로직은 `if let Some(email)` 로 분기하여
--     이메일이 없을 경우 발송 스킵

BEGIN;

-- idx_textbook_email / idx_textbook_orderer_email 은 partial 로 재생성 필요
-- 없으나, 일반 인덱스도 NULL 을 제외하고 인덱싱하므로 그대로 유지 가능.
ALTER TABLE textbook ALTER COLUMN orderer_email DROP NOT NULL;

COMMIT;
