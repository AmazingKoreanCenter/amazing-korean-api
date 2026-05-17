-- =============================================================================
-- 설명(해설) 콘텐츠 — content_type_enum 값 추가
-- =============================================================================
-- 목적: books→api 해설 콘텐츠 인계 (AMK_API_LEARNING.md §5.10).
--   explanation_unit / explanation_block 번역을 기존 content_translations
--   공유 표에 (content_type, content_id, field_name, lang) 튜플로 적재(B안).
-- 단독 마이그레이션: ALTER TYPE ADD VALUE 안전성 (선례 20260212).
-- =============================================================================

ALTER TYPE content_type_enum ADD VALUE IF NOT EXISTS 'explanation_unit';
ALTER TYPE content_type_enum ADD VALUE IF NOT EXISTS 'explanation_block';
