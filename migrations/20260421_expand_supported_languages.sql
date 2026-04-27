-- =============================================================================
-- supported_language_enum 확장 — 13개 언어 추가
-- =============================================================================
-- 목적:
--   교재(amazing-korean-books/scripts/textbook/data/sentences.json) 에 번역은
--   존재하나 현행 supported_language_enum 에 없던 13개 언어를 추가하여,
--   500문장 시딩 시 content_translations 에 전체 번역을 저장 가능하게 한다.
--
-- 정책 결정:
--   - pt_pt (Portuguese-Portugal variant) 는 pt (Portuguese-Brazil) 로 병합.
--     이중 저장 오버헤드 대비 UX 이득이 불분명하므로 enum 확장 대상에서 제외.
--     [2026-04-28 번복] 본 정책은 후속 마이그레이션
--     `20260428_add_es_pt_variants.sql` 에서 번복됨.
--     스페인어/포르투갈어 유럽 variant 가 엄연히 구분되는 언어적 표현이라는
--     사용자 결정(2026-04-27)에 따라 es_es / pt_pt 별도 enum 으로 추가.
--   - user_language_enum (UI 언어) 은 이번 확장에서 제외. UI 언어 추가는
--     사용자 설정 정책이 별도로 필요함.
-- =============================================================================

ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'tl';  -- Tagalog/Filipino
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'tr';  -- Turkish
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'bn';  -- Bengali
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'ar';  -- Arabic
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'ur';  -- Urdu
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'fa';  -- Persian / Farsi
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'lo';  -- Lao
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'ky';  -- Kyrgyz
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'it';  -- Italian
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'sw';  -- Swahili
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'uk';  -- Ukrainian
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'am';  -- Amharic
ALTER TYPE supported_language_enum ADD VALUE IF NOT EXISTS 'pl';  -- Polish
