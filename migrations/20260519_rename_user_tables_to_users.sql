-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ① user_* → users_*
-- SSoT: AMK_API_MASTER §3.2.1 (user 도메인 토큰 = users, PG 예약어 USER 기인 복수형)
-- 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- 전부 메타데이터 RENAME (데이터 이동 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 풀 정합: 테이블 + 인덱스 + 제약. user_oauth/user_export_data/user_course 토큰 포함 객체만.
-- AAD 암호화 문자열("user_oauth.oauth_email" 등)은 테이블명이 아니라 코드에서 불변 유지.

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE user_oauth       RENAME TO users_oauth;
ALTER TABLE user_export_data RENAME TO users_export_data;
ALTER TABLE user_course      RENAME TO users_course;

-- ── 인덱스 (토큰 포함분만) ───────────────────────────────────────────────
ALTER INDEX idx_user_oauth_user_id  RENAME TO idx_users_oauth_user_id;
ALTER INDEX idx_user_course_active  RENAME TO idx_users_course_active;
ALTER INDEX idx_user_course_course_id RENAME TO idx_users_course_course_id;
ALTER INDEX idx_user_course_expire  RENAME TO idx_users_course_expire;
ALTER INDEX idx_user_course_user_id RENAME TO idx_users_course_user_id;
ALTER INDEX unique_user_course      RENAME TO unique_users_course;
ALTER INDEX idx_admin_course_log_user_course RENAME TO idx_admin_course_log_users_course;

-- ── 제약 (pkey/fk, 토큰 포함분만. pkey 리네임 = 백킹 인덱스 동반 리네임) ──
ALTER TABLE users_oauth RENAME CONSTRAINT user_oauth_pkey      TO users_oauth_pkey;
ALTER TABLE users_oauth RENAME CONSTRAINT fk_user_oauth_user   TO fk_users_oauth_user;

ALTER TABLE users_export_data RENAME CONSTRAINT user_export_data_pkey            TO users_export_data_pkey;
ALTER TABLE users_export_data RENAME CONSTRAINT user_export_data_user_id_fkey   TO users_export_data_user_id_fkey;
ALTER TABLE users_export_data RENAME CONSTRAINT user_export_data_user_id_fkey1  TO users_export_data_user_id_fkey1;

ALTER TABLE users_course RENAME CONSTRAINT user_course_pkey                                 TO users_course_pkey;
ALTER TABLE users_course RENAME CONSTRAINT user_course_course_id_fkey                       TO users_course_course_id_fkey;
ALTER TABLE users_course RENAME CONSTRAINT user_course_user_id_fkey                         TO users_course_user_id_fkey;
ALTER TABLE users_course RENAME CONSTRAINT user_course_user_course_granted_by_user_id_fkey  TO users_course_user_course_granted_by_user_id_fkey;
ALTER TABLE users_course RENAME CONSTRAINT user_course_user_course_last_lesson_id_fkey      TO users_course_user_course_last_lesson_id_fkey;

ALTER TABLE admin_course_log RENAME CONSTRAINT admin_course_log_admin_pick_user_course_id_fkey TO admin_course_log_admin_pick_users_course_id_fkey;
