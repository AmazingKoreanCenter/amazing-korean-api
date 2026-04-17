-- =============================================================================
-- test-* 레거시 study 데이터 완전 리셋
-- =============================================================================
-- 목적:
--   seeds/20260208_AMK_V1_SEED.sql 로 주입된 test-1~test-9 더미 study 및
--   연관 데이터 전량 삭제. IDENTITY 시퀀스를 1부터 재시작하여 후속 시딩
--   (M05~M08) 시점에 study_id=1 부터 할당되도록 한다.
--
-- 가정 (사전 확인 필수):
--   - study 테이블에 test-* 외의 실 콘텐츠 없음 (SELECT COUNT(*) FROM study
--     WHERE study_idx NOT LIKE 'test-%' → 0)
--   - study_task_status/study_task_log 의 실 사용자 기록은 test 더미 대상
--     풀이이므로 의미 없음 (시드 콘텐츠 부재 상태였음)
--
-- 참고:
--   - 테이블명은 원본 `20260208_AMK_V1.sql` 기준 `study_explain`. content_type
--     enum 값 'study_task_explain' (20260212 에서 추가) 과 혼동하지 말 것 —
--     enum 값은 그대로이되 물리 테이블명은 study_explain.
--   - sqlx migrator 가 파일 단위 트랜잭션 처리. 명시적 BEGIN/COMMIT 생략.
--
-- INC-002 (2026-04-18): 초판에서 테이블명을 `study_task_explain` 으로 썼다가
--   프로덕션에서 `relation "study_task_explain" does not exist` 실패 → api
--   crash loop → 약 20분 다운. 로컬 DB 의 이상 rename 상태를 실 DB 로 오인한
--   실수. feedback_migration_safety.md 에 재발 방지 규칙 추가 예정.
-- =============================================================================

-- 1. 자식 테이블 먼저 삭제 (일부 FK 는 ON DELETE CASCADE 없음)
DELETE FROM content_translations
 WHERE (content_type = 'study'
        AND content_id IN (SELECT study_id FROM study))
    OR (content_type IN (
            'study_task_choice',
            'study_task_typing',
            'study_task_voice',
            'study_task_writing',
            'study_task_explain'
        )
        AND content_id IN (SELECT study_task_id FROM study_task));

-- study_task_status / study_task_log 는 CASCADE 되지만 명시적으로 선삭제
-- (트랜잭션 중 FK 순서 의존 최소화)
DELETE FROM study_task_log    WHERE study_task_id IN (SELECT study_task_id FROM study_task);
DELETE FROM study_task_status WHERE study_task_id IN (SELECT study_task_id FROM study_task);
DELETE FROM study_explain WHERE study_task_id IN (SELECT study_task_id FROM study_task);

-- 수업 연결 (task 참조) 제거 — 현재 존재하는 study_task 에 연결된 lesson_item 만
DELETE FROM lesson_item
 WHERE study_task_id IN (SELECT study_task_id FROM study_task);

-- 관리자 작업 로그 — 삭제 대상 study_id/study_task_id 에 해당하는 로그만
-- (전량 DELETE 는 위험 — 실 관리자 로그 누적 가능성)
DELETE FROM admin_study_log
 WHERE admin_pick_study_id IN (SELECT study_id FROM study)
    OR admin_pick_task_id  IN (SELECT study_task_id FROM study_task);

-- study_task 서브테이블 삭제
DELETE FROM study_task_choice  WHERE study_task_id IN (SELECT study_task_id FROM study_task);
DELETE FROM study_task_typing  WHERE study_task_id IN (SELECT study_task_id FROM study_task);
DELETE FROM study_task_voice   WHERE study_task_id IN (SELECT study_task_id FROM study_task);
DELETE FROM study_task_writing WHERE study_task_id IN (SELECT study_task_id FROM study_task);

-- writing_practice_session 중 현재 study_task 에 연결된 task 기반 세션만 제거.
-- 자유 연습 세션(study_task_id IS NULL) 은 보존.
DELETE FROM writing_practice_session
 WHERE study_task_id IN (SELECT study_task_id FROM study_task);

-- 부모 삭제
DELETE FROM study_task;
DELETE FROM study;

-- 2. IDENTITY 시퀀스 1 부터 재시작
ALTER TABLE study      ALTER COLUMN study_id      RESTART WITH 1;
ALTER TABLE study_task ALTER COLUMN study_task_id RESTART WITH 1;
