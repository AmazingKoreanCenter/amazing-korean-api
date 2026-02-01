-- Gemini 코드 리뷰 반영 마이그레이션 (2026-02-01)
-- 1. study_task_status PRIMARY KEY - 이미 존재하므로 스킵
-- 2. study_task_log login_id FK 추가

-- study_task_log에 login_id FK 추가 (아직 없는 경우)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'fk_study_task_log_login'
    ) THEN
        ALTER TABLE study_task_log
        ADD CONSTRAINT fk_study_task_log_login FOREIGN KEY (login_id) REFERENCES login (login_id);
    END IF;
END $$;
