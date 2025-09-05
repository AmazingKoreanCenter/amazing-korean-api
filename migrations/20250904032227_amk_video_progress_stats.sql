--! up

-- ===============================
-- 진척 업서트용 함수(선택 사용)
-- 앱 레이어에서 직접 UPSERT 할 수도 있지만,
-- 공용 함수로 두면 서비스 계층이 간단해집니다.
-- ===============================
CREATE OR REPLACE FUNCTION api_upsert_video_progress(
  p_video_id              BIGINT,
  p_user_id               BIGINT,
  p_position_seconds      INTEGER,
  p_duration_seconds      INTEGER,
  p_progress              INTEGER,
  p_completed             BOOLEAN,
  p_ip                    INET,
  p_device                login_device_enum,
  p_ua                    TEXT
) RETURNS VOID
LANGUAGE plpgsql
AS $fuvp$
BEGIN
  INSERT INTO video_log (
    video_id, user_id,
    last_position_seconds, total_duration_seconds, video_log_progress, video_log_completed,
    watch_count, first_watched_at, last_watched_at,
    last_ip, last_device, last_user_agent
  )
  VALUES (
    p_video_id, p_user_id,
    p_position_seconds, p_duration_seconds, p_progress, COALESCE(p_completed, FALSE),
    1, now(), now(),
    p_ip, p_device, p_ua
  )
  ON CONFLICT (video_id, user_id) DO UPDATE SET
    last_position_seconds   = GREATEST(EXCLUDED.last_position_seconds, video_log.last_position_seconds),
    total_duration_seconds  = COALESCE(EXCLUDED.total_duration_seconds, video_log.total_duration_seconds),
    video_log_progress      = GREATEST(COALESCE(EXCLUDED.video_log_progress, 0), COALESCE(video_log.video_log_progress, 0)),
    video_log_completed     = video_log.video_log_completed OR EXCLUDED.video_log_completed,
    watch_count             = CASE
                                WHEN now() - COALESCE(video_log.last_watched_at, now() - interval '11 minutes') > interval '10 minutes'
                                  THEN video_log.watch_count + 1
                                ELSE video_log.watch_count
                              END,
    last_watched_at         = now(),
    last_ip                 = EXCLUDED.last_ip,
    last_device             = EXCLUDED.last_device,
    last_user_agent         = EXCLUDED.last_user_agent;

  -- 값 범위 가드(에러로 튕기지 않도록 최소한의 클램프)
  UPDATE video_log
  SET video_log_progress = LEAST(100, GREATEST(0, video_log_progress))
  WHERE video_id = p_video_id AND user_id = p_user_id;
END
$fuvp$;

-- ===============================
-- 일별 통계 집계를 위한 트리거 함수들
-- ===============================
-- INSERT 시: 조회수 +1, (completed=true라면 완료수 +1)
CREATE OR REPLACE FUNCTION tg_video_log_on_insert() RETURNS TRIGGER
LANGUAGE plpgsql
AS $tglins$
BEGIN
  INSERT INTO video_stat_daily (stat_date, video_id, views, completes)
  VALUES (CURRENT_DATE, NEW.video_id, 1, CASE WHEN NEW.video_log_completed THEN 1 ELSE 0 END)
  ON CONFLICT (stat_date, video_id) DO UPDATE
  SET views = video_stat_daily.views + 1,
      completes = video_stat_daily.completes + EXCLUDED.completes;
  RETURN NEW;
END
$tglins$;

-- UPDATE 시: completed가 false→true로 바뀌었을 때만 완료수 +1
CREATE OR REPLACE FUNCTION tg_video_log_on_complete() RETURNS TRIGGER
LANGUAGE plpgsql
AS $tglupd$
BEGIN
  INSERT INTO video_stat_daily (stat_date, video_id, views, completes)
  VALUES (CURRENT_DATE, NEW.video_id, 0, 1)
  ON CONFLICT (stat_date, video_id) DO UPDATE
  SET completes = video_stat_daily.completes + 1;
  RETURN NEW;
END
$tglupd$;

-- ===============================
-- 트리거 부착
-- ===============================
DROP TRIGGER IF EXISTS trg_vlog_stat_ins ON video_log;
CREATE TRIGGER trg_vlog_stat_ins
AFTER INSERT ON video_log
FOR EACH ROW EXECUTE FUNCTION tg_video_log_on_insert();

DROP TRIGGER IF EXISTS trg_vlog_stat_complete ON video_log;
CREATE TRIGGER trg_vlog_stat_complete
AFTER UPDATE OF video_log_completed ON video_log
FOR EACH ROW
WHEN (NEW.video_log_completed IS TRUE AND (OLD.video_log_completed IS DISTINCT FROM TRUE))
EXECUTE FUNCTION tg_video_log_on_complete();

--! down

-- 트리거/함수 제거
DROP TRIGGER IF EXISTS trg_vlog_stat_complete ON video_log;
DROP TRIGGER IF EXISTS trg_vlog_stat_ins ON video_log;

DROP FUNCTION IF EXISTS tg_video_log_on_complete();
DROP FUNCTION IF EXISTS tg_video_log_on_insert();
DROP FUNCTION IF EXISTS api_upsert_video_progress(
  BIGINT, BIGINT, INTEGER, INTEGER, INTEGER, BOOLEAN, INET, login_device_enum, TEXT
);

-- ===============================
-- 일별 통계 집계를 위한 트리거 함수들
-- ===============================
-- INSERT 시: 조회수 +1, (completed=true라면 완료수 +1)
CREATE OR REPLACE FUNCTION tg_video_log_on_insert() RETURNS TRIGGER
LANGUAGE plpgsql
AS $tglins$
BEGIN
  INSERT INTO video_stat_daily (stat_date, video_id, views, completes)
  VALUES (CURRENT_DATE, NEW.video_id, 1, CASE WHEN NEW.video_log_completed THEN 1 ELSE 0 END)
  ON CONFLICT (stat_date, video_id) DO UPDATE
  SET views = video_stat_daily.views + 1,
      completes = video_stat_daily.completes + EXCLUDED.completes;
  RETURN NEW;
END
$tglins$;

-- UPDATE 시: completed가 false→true로 바뀌었을 때만 완료수 +1
CREATE OR REPLACE FUNCTION tg_video_log_on_complete() RETURNS TRIGGER
LANGUAGE plpgsql
AS $tglupd$
BEGIN
  INSERT INTO video_stat_daily (stat_date, video_id, views, completes)
  VALUES (CURRENT_DATE, NEW.video_id, 0, 1)
  ON CONFLICT (stat_date, video_id) DO UPDATE
  SET completes = video_stat_daily.completes + 1;
  RETURN NEW;
END
$tglupd$;

-- ===============================
-- 트리거 부착
-- ===============================
DROP TRIGGER IF EXISTS trg_vlog_stat_ins ON video_log;
CREATE TRIGGER trg_vlog_stat_ins
AFTER INSERT ON video_log
FOR EACH ROW EXECUTE FUNCTION tg_video_log_on_insert();

DROP TRIGGER IF EXISTS trg_vlog_stat_complete ON video_log;
CREATE TRIGGER trg_vlog_stat_complete
AFTER UPDATE OF video_log_completed ON video_log
FOR EACH ROW
WHEN (NEW.video_log_completed IS TRUE AND (OLD.video_log_completed IS DISTINCT FROM TRUE))
EXECUTE FUNCTION tg_video_log_on_complete();

--! down

-- 트리거/함수 제거
DROP TRIGGER IF EXISTS trg_vlog_stat_complete ON video_log;
DROP TRIGGER IF EXISTS trg_vlog_stat_ins ON video_log;

DROP FUNCTION IF EXISTS tg_video_log_on_complete();
DROP FUNCTION IF EXISTS tg_video_log_on_insert();
DROP FUNCTION IF EXISTS api_upsert_video_progress(
  BIGINT, BIGINT, INTEGER, INTEGER, INTEGER, BOOLEAN, INET, login_device_enum, TEXT
);
