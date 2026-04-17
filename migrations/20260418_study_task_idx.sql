-- =============================================================================
-- study_task_idx — 외부 공개용 안정 참조 키
-- =============================================================================
-- 목적:
--   1. 교재/해설집 시딩의 재시딩 멱등성 확보 (study_task_idx 기준 upsert)
--   2. 해설집 시딩 시 문장별 안정 참조 키 (예: 'amk500-sent-001')
--   3. content_translations.content_id 연결 안정성 (PK 변동에도 논리 연결 유지)
-- =============================================================================

-- 1. 컬럼 추가 (NULL 허용으로 시작하여 기존 데이터 백필 가능하게 함)
ALTER TABLE study_task ADD COLUMN study_task_idx varchar(100);

-- 2. 기존 레거시 행 백필
--    후속 마이그레이션(20260419_reset_test_studies.sql) 에서 어차피 삭제되지만,
--    NOT NULL 제약을 걸기 위해 임시 유니크 값으로 채운다.
UPDATE study_task
   SET study_task_idx = 'legacy-' || study_task_id
 WHERE study_task_idx IS NULL;

-- 3. NOT NULL 제약
ALTER TABLE study_task ALTER COLUMN study_task_idx SET NOT NULL;

-- 4. UNIQUE 인덱스
CREATE UNIQUE INDEX uq_study_task_idx ON study_task (study_task_idx);

-- 5. 컬럼 주석
COMMENT ON COLUMN study_task.study_task_idx
    IS '외부 공개용 안정 참조 키 (해설집 참조·재시딩 멱등성용). 예: amk500-sent-001';
