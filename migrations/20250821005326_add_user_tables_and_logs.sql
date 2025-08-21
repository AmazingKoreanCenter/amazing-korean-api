-- Add user_log, user_setting, user_language_pref, admin_user_action_log, user_export_job

--! up
CREATE TABLE IF NOT EXISTS user_log (
  user_log_id             BIGSERIAL PRIMARY KEY,
  action                  VARCHAR(20) NOT NULL,  -- 'create'|'update'|'deactivate'|'delete'...
  updated_by_user_id      BIGINT REFERENCES users(user_id),
  user_id                 BIGINT NOT NULL REFERENCES users(user_id),

  user_auth_log           VARCHAR(20),
  user_state_log          VARCHAR(10),
  user_email_log          VARCHAR(255),
  user_password_log       TEXT,
  user_nickname_log       VARCHAR(100),
  user_language_log       VARCHAR(50),
  user_country_log        VARCHAR(50),
  user_birthday_log       DATE,
  user_gender_log         VARCHAR(10),
  user_terms_service_log  BOOLEAN,
  user_terms_personal_log BOOLEAN,

  user_log_created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_user_log_user_id    ON user_log(user_id);
CREATE INDEX IF NOT EXISTS idx_user_log_updated_by ON user_log(updated_by_user_id);

CREATE TABLE IF NOT EXISTS user_setting (
  user_id              BIGINT PRIMARY KEY REFERENCES users(user_id),
  ui_language          VARCHAR(8) NOT NULL DEFAULT 'en',  -- ISO 639-1
  timezone             VARCHAR(64),
  notifications_email  BOOLEAN NOT NULL DEFAULT TRUE,
  notifications_push   BOOLEAN NOT NULL DEFAULT FALSE,
  created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_language_pref (
  user_id     BIGINT NOT NULL REFERENCES users(user_id),
  lang_code   VARCHAR(8) NOT NULL,   -- 'en','ko','ne','si','id','vi','th' 등
  priority    INT NOT NULL,
  is_primary  BOOLEAN NOT NULL DEFAULT FALSE,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(user_id, lang_code)
);
-- 조회/정렬 최적화 인덱스
CREATE INDEX IF NOT EXISTS idx_ulpref_user_priority ON user_language_pref(user_id, priority);
CREATE INDEX IF NOT EXISTS idx_ulpref_user_primary  ON user_language_pref(user_id) WHERE is_primary;

CREATE TABLE IF NOT EXISTS admin_user_action_log (
  id              BIGSERIAL PRIMARY KEY,
  actor_user_id   BIGINT NOT NULL REFERENCES users(user_id),
  target_user_id  BIGINT NOT NULL REFERENCES users(user_id),
  action          VARCHAR(32) NOT NULL,             -- 'admin_update','admin_block' 등
  before          JSONB,
  after           JSONB,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_admin_action_target ON admin_user_action_log(target_user_id);
CREATE INDEX IF NOT EXISTS idx_admin_action_actor  ON admin_user_action_log(actor_user_id);

CREATE TABLE IF NOT EXISTS user_export_job (
  job_id        BIGSERIAL PRIMARY KEY,
  user_id       BIGINT NOT NULL REFERENCES users(user_id),
  status        VARCHAR(16) NOT NULL DEFAULT 'queued', -- 'queued'|'processing'|'done'|'failed'
  signed_url    TEXT,
  expires_at    TIMESTAMPTZ,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- 사용자별 상태 조회/쿨다운 체크에 유용
CREATE INDEX IF NOT EXISTS idx_export_user_status ON user_export_job(user_id, status);
-- 같은 사용자의 'queued' 중복 생성 방지(선택)
CREATE UNIQUE INDEX IF NOT EXISTS ux_export_user_queued ON user_export_job(user_id) WHERE status = 'queued';

--! down
DROP TABLE IF EXISTS user_export_job;
DROP TABLE IF EXISTS admin_user_action_log;
DROP INDEX IF EXISTS idx_ulpref_user_primary;
DROP INDEX IF EXISTS idx_ulpref_user_priority;
DROP TABLE IF EXISTS user_language_pref;
DROP TABLE IF EXISTS user_setting;
DROP INDEX IF EXISTS idx_user_log_updated_by;
DROP INDEX IF EXISTS idx_user_log_user_id;
DROP TABLE IF EXISTS user_log;
