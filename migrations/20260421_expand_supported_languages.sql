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
