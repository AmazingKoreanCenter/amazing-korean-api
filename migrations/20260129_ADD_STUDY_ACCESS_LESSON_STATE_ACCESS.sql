-- 2026-01-29: study_access, lesson_state, lesson_access 컬럼 추가
-- study 테이블에 access 컬럼 추가 (video와 동일한 패턴)
-- lesson 테이블에 state, access 컬럼 추가

-- 1. ENUM 타입 생성
DO $$ BEGIN
    CREATE TYPE study_access_enum AS ENUM ('public', 'paid', 'private', 'promote');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE lesson_state_enum AS ENUM ('ready', 'open', 'close');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE lesson_access_enum AS ENUM ('public', 'paid', 'private', 'promote');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 2. study 테이블에 study_access 컬럼 추가
ALTER TABLE study ADD COLUMN IF NOT EXISTS study_access study_access_enum NOT NULL DEFAULT 'public';

-- 3. lesson 테이블에 lesson_state, lesson_access 컬럼 추가
ALTER TABLE lesson ADD COLUMN IF NOT EXISTS lesson_state lesson_state_enum NOT NULL DEFAULT 'ready';
ALTER TABLE lesson ADD COLUMN IF NOT EXISTS lesson_access lesson_access_enum NOT NULL DEFAULT 'public';

-- 4. 코멘트 추가
COMMENT ON COLUMN study.study_access IS '학습 접근 권한: public(공개), paid(유료), private(비공개), promote(프로모션)';
COMMENT ON COLUMN lesson.lesson_state IS '수업 상태: ready(준비), open(공개), close(종료)';
COMMENT ON COLUMN lesson.lesson_access IS '수업 접근 권한: public(공개), paid(유료), private(비공개), promote(프로모션)';

-- 5. 인덱스 추가 (검색 성능 향상)
CREATE INDEX IF NOT EXISTS index_study_state_access ON study (study_state, study_access);
CREATE INDEX IF NOT EXISTS index_lesson_state_access ON lesson (lesson_state, lesson_access);
