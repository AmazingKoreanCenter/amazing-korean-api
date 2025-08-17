-- USERS (Postgres 예약어 USER 대신 users)
CREATE TABLE IF NOT EXISTS users (
  user_id              BIGSERIAL PRIMARY KEY,
  user_auth            TEXT NOT NULL DEFAULT 'learner' CHECK (user_auth IN ('HYMN','admin','manager','learner')),
  user_state           TEXT NOT NULL DEFAULT 'on'      CHECK (user_state IN ('on','off')),
  user_email           VARCHAR(255) NOT NULL UNIQUE,
  user_password        TEXT NOT NULL,
  user_name            VARCHAR(100),
  user_nickname        VARCHAR(100),
  user_language        VARCHAR(50),
  user_country         VARCHAR(50),
  user_birthday        TIMESTAMPTZ,
  user_gender          TEXT NOT NULL DEFAULT 'none' CHECK (user_gender IN ('none','male','female','other')),
  user_terms_service   BOOLEAN NOT NULL DEFAULT FALSE,
  user_terms_personal  BOOLEAN NOT NULL DEFAULT FALSE,
  user_created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  user_quit_at         TIMESTAMPTZ
);

-- COURSE
CREATE TABLE IF NOT EXISTS course (
  course_id         BIGSERIAL PRIMARY KEY,
  course_type       TEXT NOT NULL CHECK (course_type IN ('video','study','live','package')),
  course_state      TEXT NOT NULL DEFAULT 'active' CHECK (course_state IN ('active','inactive','deleted')),
  course_title      VARCHAR(255) NOT NULL,
  course_subtitle   TEXT,
  course_price      INT NOT NULL DEFAULT 0,
  course_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  course_updated_at TIMESTAMPTZ,
  course_banned_at  TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_course_state ON course(course_state);
CREATE INDEX IF NOT EXISTS idx_course_type  ON course(course_type);

-- PAY
CREATE TABLE IF NOT EXISTS pay (
  pay_id           BIGSERIAL PRIMARY KEY,
  user_id          BIGINT NOT NULL REFERENCES users(user_id) ON DELETE RESTRICT,
  course_id        BIGINT NOT NULL REFERENCES course(course_id) ON DELETE RESTRICT,
  pay_token        TEXT,
  pay_state        TEXT NOT NULL DEFAULT 'ready' CHECK (pay_state IN ('ready','done','cancel')),
  pay_start_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
  pay_end_at       TIMESTAMPTZ,
  idempotency_key  VARCHAR(64),
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS ux_pay_idempotency ON pay(idempotency_key) WHERE idempotency_key IS NOT NULL;
CREATE INDEX        IF NOT EXISTS idx_pay_user_course ON pay(user_id, course_id);

/* (선택) 같은 유저가 같은 코스를 'done'으로 중복 결제 금지하고 싶으면 활성화
-- CREATE UNIQUE INDEX ux_pay_user_course_done ON pay(user_id, course_id) WHERE pay_state='done';
*/