-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ① user_* → users_*
-- SSoT: AMK_API_MASTER §3.2.1. 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- 전부 메타데이터 RENAME (데이터 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 제약/인덱스 리네임 = 존재 가드(환경 독립): clean CI / prod 증분 / 손패치 로컬 모두 안전.
--   (사유: 손패치 로컬 amk-pg 에만 있던 중복 FK user_export_data_user_id_fkey1 등
--    환경별 제약 집합 차이 → pg_constraint 존재 확인 후만 RENAME)
-- AAD 암호문 문자열은 테이블명 아님 → 코드에서 불변(본 마이그 무관).

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE user_oauth       RENAME TO users_oauth;
ALTER TABLE user_export_data RENAME TO users_export_data;
ALTER TABLE user_course      RENAME TO users_course;

-- ── 인덱스 (토큰 포함분만, IF EXISTS = 환경 독립) ────────────────────────
ALTER INDEX IF EXISTS idx_user_oauth_user_id  RENAME TO idx_users_oauth_user_id;
ALTER INDEX IF EXISTS idx_user_course_active  RENAME TO idx_users_course_active;
ALTER INDEX IF EXISTS idx_user_course_course_id RENAME TO idx_users_course_course_id;
ALTER INDEX IF EXISTS idx_user_course_expire  RENAME TO idx_users_course_expire;
ALTER INDEX IF EXISTS idx_user_course_user_id RENAME TO idx_users_course_user_id;
ALTER INDEX IF EXISTS unique_user_course      RENAME TO unique_users_course;
ALTER INDEX IF EXISTS idx_admin_course_log_user_course RENAME TO idx_admin_course_log_users_course;

-- ── 제약 (pkey/fk, 존재 확인 후만 RENAME) ───────────────────────────────
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT * FROM (VALUES
    ('users_oauth',       'user_oauth_pkey',                                'users_oauth_pkey'),
    ('users_oauth',       'fk_user_oauth_user',                             'fk_users_oauth_user'),
    ('users_export_data', 'user_export_data_pkey',                          'users_export_data_pkey'),
    ('users_export_data', 'user_export_data_user_id_fkey',                  'users_export_data_user_id_fkey'),
    ('users_export_data', 'user_export_data_user_id_fkey1',                 'users_export_data_user_id_fkey1'),
    ('users_course',      'user_course_pkey',                               'users_course_pkey'),
    ('users_course',      'user_course_course_id_fkey',                     'users_course_course_id_fkey'),
    ('users_course',      'user_course_user_id_fkey',                       'users_course_user_id_fkey'),
    ('users_course',      'user_course_user_course_granted_by_user_id_fkey','users_course_user_course_granted_by_user_id_fkey'),
    ('users_course',      'user_course_user_course_last_lesson_id_fkey',    'users_course_user_course_last_lesson_id_fkey'),
    ('admin_course_log',  'admin_course_log_admin_pick_user_course_id_fkey','admin_course_log_admin_pick_users_course_id_fkey')
  ) AS t(tbl, oldn, newn)
  LOOP
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = r.oldn AND conrelid = r.tbl::regclass) THEN
      EXECUTE format('ALTER TABLE %I RENAME CONSTRAINT %I TO %I', r.tbl, r.oldn, r.newn);
    END IF;
  END LOOP;
END $$;
