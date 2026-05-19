-- 스키마 명명 SSoT 정리 트랙 2단계 — 그룹 ② writing_practice_* → study_writing_practice_*
-- SSoT: AMK_API_MASTER §3.2.1 (도메인 접두사 필수. writing_practice = study 영역 자유연습,
--       study_task_writing 과 별개 기능). 감사 SoT: docs/AMK_SCHEMA_NAMING_AUDIT.md §9
--
-- 전부 메타데이터 RENAME (데이터 이동 0, 무중단). 신규 forward 마이그(INC-004 차단).
-- 풀 정합: writing_practice_seed/writing_practice_session 토큰 포함 객체만.
-- 약어 객체(idx_wps_*, fk_wps_*)는 토큰 미포함 → 미변경(그룹 ① 동일 규칙).

-- ── 테이블 ───────────────────────────────────────────────────────────────
ALTER TABLE writing_practice_seed    RENAME TO study_writing_practice_seed;
ALTER TABLE writing_practice_session RENAME TO study_writing_practice_session;

-- ── 인덱스 (토큰 포함분만) ───────────────────────────────────────────────
ALTER INDEX idx_writing_practice_seed_level_type RENAME TO idx_study_writing_practice_seed_level_type;

-- ── 제약 (pkey/unique, 토큰 포함분만. 백킹 인덱스 동반 리네임) ───────────
ALTER TABLE study_writing_practice_seed    RENAME CONSTRAINT writing_practice_seed_pkey                 TO study_writing_practice_seed_pkey;
ALTER TABLE study_writing_practice_seed    RENAME CONSTRAINT uniq_writing_practice_seed_level_type_seq TO uniq_study_writing_practice_seed_level_type_seq;
ALTER TABLE study_writing_practice_session RENAME CONSTRAINT writing_practice_session_pkey              TO study_writing_practice_session_pkey;
