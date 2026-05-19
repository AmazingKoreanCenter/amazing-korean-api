-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ② writing_practice_* → study_writing_practice_*
-- SSoT: AMK_API_MASTER §3.2.1. 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- 전부 메타데이터 RENAME (데이터 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 제약/인덱스 = 존재 가드(환경 독립). 약어 객체(idx_wps_*/fk_wps_*) = 토큰 미포함 미변경.

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE writing_practice_seed    RENAME TO study_writing_practice_seed;
ALTER TABLE writing_practice_session RENAME TO study_writing_practice_session;

-- ── 인덱스 (토큰 포함분만, IF EXISTS) ────────────────────────────────────
ALTER INDEX IF EXISTS idx_writing_practice_seed_level_type RENAME TO idx_study_writing_practice_seed_level_type;

-- ── 제약 (pkey/unique, 존재 확인 후만 RENAME) ──────────────────────────
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT * FROM (VALUES
    ('study_writing_practice_seed',    'writing_practice_seed_pkey',                'study_writing_practice_seed_pkey'),
    ('study_writing_practice_seed',    'uniq_writing_practice_seed_level_type_seq', 'uniq_study_writing_practice_seed_level_type_seq'),
    ('study_writing_practice_session', 'writing_practice_session_pkey',             'study_writing_practice_session_pkey')
  ) AS t(tbl, oldn, newn)
  LOOP
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = r.oldn AND conrelid = r.tbl::regclass) THEN
      EXECUTE format('ALTER TABLE %I RENAME CONSTRAINT %I TO %I', r.tbl, r.oldn, r.newn);
    END IF;
  END LOOP;
END $$;
