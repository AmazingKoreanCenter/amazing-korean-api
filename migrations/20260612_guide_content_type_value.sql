-- =============================================================================
-- guide 도메인 — content_type_enum 값 추가
-- =============================================================================
-- 목적: 온라인 콘텐츠(해설집) guide 도메인 번역을 기존 content_translations
--   공유 표에 (content_type, content_id, field_name, lang) 튜플로 적재.
--   설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md §3.
-- 번역 대상은 guide_block.text 단일 (단원 title/subtitle 은 TITLE/PARAGRAPH
--   블록의 비정규화 사본이라 'guide' content_type 불요 — 미사용 enum 값 미추가).
-- 단독 마이그레이션: ALTER TYPE ADD VALUE 안전성 (선례 20260212/20260517).
-- =============================================================================

ALTER TYPE content_type_enum ADD VALUE IF NOT EXISTS 'guide_block';
