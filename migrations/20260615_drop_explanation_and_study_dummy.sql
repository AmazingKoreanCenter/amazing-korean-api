-- =============================================================================
-- PR-4b — 구 explanation 도메인 DROP + study/study_task 더미 DELETE
-- =============================================================================
-- 배경: explanation 도메인은 guide(§5.12)로 전면 대체, 코드는 PR-4a(#331)에서 제거.
--   study67/task500 더미(amk500-*, 숨김 시딩)도 guide로 대체되어 정리.
-- 설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md §5. STATUS #166.
-- ⚠️ prod 데이터 영구 삭제 (되돌리기 불가). 서버 부팅 시 자동 적용.
-- 보존: study/study_task 테이블·HYMN 계정(user_id=8)·study_task explain 기능(study 도메인).
-- =============================================================================

-- ── Part 1: explanation 도메인 제거 ──────────────────────────────────────────
-- 번역 행 먼저 (content_translations 공유 표). prod 4,362행(en).
DELETE FROM content_translations
 WHERE content_type IN ('explanation_unit', 'explanation_block');

-- 테이블 (block → unit, block 이 unit 을 ON DELETE CASCADE 참조).
DROP TABLE IF EXISTS explanation_block;
DROP TABLE IF EXISTS explanation_unit;

-- enum 타입 (explanation 테이블 전용 — 다른 참조 없음 확인됨).
DROP TYPE IF EXISTS explanation_block_type_enum;
DROP TYPE IF EXISTS explanation_unit_kind_enum;
DROP TYPE IF EXISTS explanation_source_enum;

-- 주: content_type_enum 의 'explanation_unit'/'explanation_block' 값은
--    PostgreSQL 제약상 enum 값 DROP 불가(타입 재생성 필요) → 미사용 휴면 잔존.

-- ── Part 2: study/study_task 더미(amk500-*) 제거 (테이블 유지) ────────────────
-- 더미 task 식별 = amk500-* study 에 속한 study_task.
-- FK 순서: 무-cascade 자식(choice/typing/voice/writing/explain, lesson_item) 명시 삭제
--   → study_task (status/log 는 ON DELETE CASCADE) → study.
-- 전부 amk500-* 로 엄격 스코프. 더미 = typing 만 실재(나머지 자식은 방어적 0행 삭제).
-- 서브쿼리 인라인: CI(psql autocommit) + sqlx(tx) 양쪽에서 동작 (TEMP TABLE 회피).
DELETE FROM study_task_choice  WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');
DELETE FROM study_task_typing  WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');
DELETE FROM study_task_voice   WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');
DELETE FROM study_task_writing WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');
DELETE FROM study_explain      WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');
DELETE FROM lesson_item        WHERE study_task_id IN
  (SELECT st.study_task_id FROM study_task st JOIN study s ON s.study_id=st.study_id WHERE s.study_idx LIKE 'amk500-%');

-- study_task (status/log ON DELETE CASCADE 자동 정리)
DELETE FROM study_task WHERE study_id IN (SELECT study_id FROM study WHERE study_idx LIKE 'amk500-%');

-- study (HYMN 계정은 users 테이블이라 무관 — 유지)
DELETE FROM study WHERE study_idx LIKE 'amk500-%';
