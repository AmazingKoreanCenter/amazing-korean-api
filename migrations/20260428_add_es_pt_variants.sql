-- =============================================================================
-- supported_language_enum 확장 — es_es / pt_pt 지역 variant 추가
-- =============================================================================
-- 정책 번복 (2026-04-27 사용자 결정):
--   2026-04-21 마이그레이션(20260421_expand_supported_languages.sql) 에서
--   "pt_pt 는 pt 로 병합" 으로 결정했으나, 스페인어/포르투갈어가
--   유럽 vs 중남미·브라질로 엄연히 구분되는 언어적 표현이라는 점에서 변경.
--   books `sentences.json` 35 언어 전부를 enum 으로 수용한다.
--
-- books seed SQL (gen_seed_sql.py) 의 "es_es + pt_pt skip" 로직은 별도
-- handoff 후 books 측에서 제거 필요 (Phase 4).
-- =============================================================================

ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'es_es';  -- Spanish (Spain/European)
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'pt_pt';  -- Portuguese (Portugal/European)
