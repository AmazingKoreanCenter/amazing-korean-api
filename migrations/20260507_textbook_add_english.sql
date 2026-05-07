-- =============================================================================
-- textbook_language_enum 확장 — 영어 ('en') 추가
-- =============================================================================
-- 목적:
--   2026-05-07 사용자 보고: 관리자 textbook 주문 생성 UI 에서 "영어 학생용/교사용"
--   선택지 부재. 원인 = 본 enum 에 'en' 미등록 (initial 21 + 20260310 tl + 20260503
--   14 expand = 36 언어 모두 영어 누락).
--
--   Amazing Korean = 한국어 학습 서비스, 외국어 화자 대상.
--   영어 화자 학습자 = 가장 흔한 글로벌 사용자 → enum 누락은 명백한 부채.
--
-- 정책:
--   - enum ADD 만, 기존 값 변경 X — 이미 발행된 textbook_orders / ebook 행 안전
--     (textbook_language_enum 은 ebook 도메인 재활용, 20260311_ebook.sql L15)
--   - Postgres ALTER TYPE ADD VALUE 는 enum 끝에 부착됨
--   - Rust 측 TextbookLanguage::En + catalog_languages 영어 항목 추가는 별도 commit 동기
-- =============================================================================

ALTER TYPE textbook_language_enum ADD VALUE IF NOT EXISTS 'en';     -- English
