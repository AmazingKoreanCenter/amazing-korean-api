-- =============================================================================
-- textbook_language_enum 확장 — 14개 언어 추가
-- =============================================================================
-- 목적:
--   books-api-bridge plan §3 Stage 1 #1. textbook 도메인의 언어 커버리지를
--   현행 21언어 → 35언어로 확장. ebook 도메인도 동일 enum 재활용 (20260311_ebook.sql L15).
--
--   Q13 Phase 1 (2026-04-28) 에서 supported_language_enum 만 35언어로 확장됐고
--   textbook_language_enum 은 21언어 그대로 남아 있어 books-api-bridge plan §1
--   진단 갭 #1 으로 도출됨.
--
-- 정책:
--   - enum ADD 만, 기존 값 변경 X — 이미 발행된 textbook_orders 행 안전.
--   - es_es / pt_pt 표기는 supported_language_enum 과 동일 (snake_case in DB,
--     BCP 47 'es-ES' / 'pt-PT' in API 응답은 serde rename 으로 처리).
--   - 알파벳순 추가 (ALTER TYPE ADD VALUE 는 enum 내부 순서를 보장하지 않으나
--     Postgres 상 ADD 순서 그대로 enum 끝에 부착됨).
-- =============================================================================

ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'am';     -- Amharic
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'ar';     -- Arabic
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'bn';     -- Bengali
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'es_es';  -- Spanish (Spain variant)
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'fa';     -- Persian / Farsi
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'it';     -- Italian
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'ky';     -- Kyrgyz
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'lo';     -- Lao
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'pl';     -- Polish
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'pt_pt';  -- Portuguese (Portugal variant)
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'sw';     -- Swahili
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'tr';     -- Turkish
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'uk';     -- Ukrainian
ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'ur';     -- Urdu
