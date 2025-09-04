--! up

DO $enum$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'video_state_enum') THEN
    CREATE TYPE video_state_enum AS ENUM ('ready','open','close');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'video_access_enum') THEN
    CREATE TYPE video_access_enum AS ENUM ('public','paid','private');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'caption_storage_enum') THEN
    CREATE TYPE caption_storage_enum AS ENUM ('vimeo','s3','url');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'caption_kind_enum') THEN
    CREATE TYPE caption_kind_enum AS ENUM ('subtitles','captions');
  END IF;
END
$enum$;

CREATE OR REPLACE FUNCTION set_video_updated_at() RETURNS TRIGGER
LANGUAGE plpgsql
AS $fv$
BEGIN
  NEW.video_updated_at := now();
  RETURN NEW;
END
$fv$;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER
LANGUAGE plpgsql
AS $fu$
BEGIN
  NEW.updated_at := now();
  RETURN NEW;
END
$fu$;

CREATE TABLE IF NOT EXISTS video (
  video_id               BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  updated_by_user_id     BIGINT NOT NULL REFERENCES users(user_id),

  video_idx              VARCHAR NOT NULL UNIQUE,
  video_state            video_state_enum NOT NULL DEFAULT 'ready',
  video_access           video_access_enum NOT NULL DEFAULT 'paid',

  video_title            VARCHAR(100),
  video_subtitle         TEXT,

  vimeo_video_id         VARCHAR(64),
  video_duration_seconds INTEGER,
  video_thumbnail_url    TEXT,
  video_language         VARCHAR(10),

  video_link             TEXT,

  video_created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  video_updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  video_banned_at        TIMESTAMPTZ,
  video_banned_reason    TEXT,
  deleted_at             TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_video_state_access ON video (video_state, video_access);
CREATE INDEX IF NOT EXISTS idx_video_lang          ON video (video_language);
CREATE INDEX IF NOT EXISTS idx_video_created       ON video (video_created_at);

-- updated_at 자동 갱신
DROP TRIGGER IF EXISTS trg_video_set_updated_at ON video;
CREATE TRIGGER trg_video_set_updated_at
BEFORE UPDATE ON video
FOR EACH ROW EXECUTE FUNCTION set_video_updated_at();

CREATE TABLE IF NOT EXISTS video_log (
  video_log_id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  video_id                BIGINT NOT NULL REFERENCES video(video_id),
  user_id                 BIGINT NOT NULL REFERENCES users(user_id),

  video_log_progress      INTEGER,                  -- 0~100
  last_position_seconds   INTEGER,                  -- 마지막 위치(초)
  total_duration_seconds  INTEGER,                  -- 당시 총 길이 스냅샷
  video_log_completed     BOOLEAN NOT NULL DEFAULT FALSE,
  watch_count             INTEGER NOT NULL DEFAULT 0,

  first_watched_at        TIMESTAMPTZ DEFAULT now(),
  last_watched_at         TIMESTAMPTZ,
  last_ip                 INET,
  last_device             login_device_enum,
  last_user_agent         TEXT,
  -- 과거 호환
  video_watched_at        TIMESTAMPTZ,

  CONSTRAINT chk_video_log_progress
    CHECK (video_log_progress IS NULL OR (video_log_progress BETWEEN 0 AND 100))
);

-- 유저별 1행 보장
CREATE UNIQUE INDEX IF NOT EXISTS uq_video_user
  ON video_log (video_id, user_id);

-- 조회 최적화
CREATE INDEX IF NOT EXISTS idx_vlog_user_last
  ON video_log (user_id, last_watched_at);

CREATE INDEX IF NOT EXISTS idx_vlog_video_last
  ON video_log (video_id, last_watched_at);

CREATE TABLE IF NOT EXISTS video_caption (
  caption_id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  video_id           BIGINT NOT NULL REFERENCES video(video_id),
  lang_code          VARCHAR(10),         -- BCP-47
  label              VARCHAR(50),
  kind               caption_kind_enum NOT NULL DEFAULT 'subtitles',

  storage_provider   caption_storage_enum NOT NULL DEFAULT 'vimeo',
  vimeo_texttrack_id VARCHAR(64),         -- storage=vimeo 일 때
  caption_url        TEXT,                -- storage=s3/url 일 때

  is_default         BOOLEAN NOT NULL DEFAULT FALSE,
  is_active          BOOLEAN NOT NULL DEFAULT TRUE,

  created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at         TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 고유성/조회 인덱스
CREATE UNIQUE INDEX IF NOT EXISTS uq_vcaption_lang_kind
  ON video_caption (video_id, lang_code, kind);

CREATE INDEX IF NOT EXISTS idx_vcaption_video
  ON video_caption (video_id);

CREATE INDEX IF NOT EXISTS idx_vcaption_active
  ON video_caption (is_active);

-- 영상당 default 1개(활성만) 보장: 부분 유니크 인덱스
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1 FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE c.relkind = 'i' AND c.relname = 'uq_vcaption_one_default_per_video'
  ) THEN
    EXECUTE 'CREATE UNIQUE INDEX uq_vcaption_one_default_per_video
             ON video_caption (video_id)
             WHERE is_default = TRUE AND is_active = TRUE';
  END IF;
END$$;

-- updated_at 자동 갱신
DROP TRIGGER IF EXISTS trg_vcaption_set_updated_at ON video_caption;
CREATE TRIGGER trg_vcaption_set_updated_at
BEFORE UPDATE ON video_caption
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TABLE IF NOT EXISTS video_tag (
  video_tag_id  BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  tag_key       VARCHAR(30) NOT NULL UNIQUE,  -- eps, topik, pron, grammar ...
  tag_name      VARCHAR(50) NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS video_tag_map (
  video_id      BIGINT NOT NULL REFERENCES video(video_id),
  video_tag_id  BIGINT NOT NULL REFERENCES video_tag(video_tag_id),
  PRIMARY KEY (video_id, video_tag_id)
);

CREATE INDEX IF NOT EXISTS idx_vtag_tag ON video_tag_map (video_tag_id);

CREATE TABLE IF NOT EXISTS video_stat_daily (
  stat_date     DATE NOT NULL,
  video_id      BIGINT NOT NULL REFERENCES video(video_id),
  views         INTEGER NOT NULL DEFAULT 0,
  completes     INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT uq_vstat_day UNIQUE (stat_date, video_id)
);

--! down

-- 드롭 순서: 의존(자막/태그/로그/통계) → 본문 → 함수 → 타입

DROP TABLE IF EXISTS video_stat_daily;
DROP TABLE IF EXISTS video_tag_map;
DROP TABLE IF EXISTS video_tag;
DROP TRIGGER IF EXISTS trg_vcaption_set_updated_at ON video_caption;
DROP TABLE IF EXISTS video_caption;

DROP INDEX IF EXISTS idx_vlog_video_last;
DROP INDEX IF EXISTS idx_vlog_user_last;
DROP INDEX IF EXISTS uq_video_user;
DROP TABLE IF EXISTS video_log;

DROP TRIGGER IF EXISTS trg_video_set_updated_at ON video;
DROP INDEX IF EXISTS idx_video_created;
DROP INDEX IF EXISTS idx_video_lang;
DROP INDEX IF EXISTS idx_video_state_access;
DROP TABLE IF EXISTS video;

-- helper functions
DROP FUNCTION IF EXISTS set_updated_at();
DROP FUNCTION IF EXISTS set_video_updated_at();

-- 부분 유니크 인덱스는 테이블 드롭과 함께 제거되지만, 방어적으로도 처리 가능
-- DROP INDEX IF EXISTS uq_vcaption_one_default_per_video;

-- enums (login_device_enum 은 기존 마이그레이션 소유이므로 삭제하지 않음)
DO $$
BEGIN
  IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'caption_kind_enum') THEN
    DROP TYPE caption_kind_enum;
  END IF;
  IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'caption_storage_enum') THEN
    DROP TYPE caption_storage_enum;
  END IF;
  IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'video_access_enum') THEN
    DROP TYPE video_access_enum;
  END IF;
  IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'video_state_enum') THEN
    DROP TYPE video_state_enum;
  END IF;
END$$;
