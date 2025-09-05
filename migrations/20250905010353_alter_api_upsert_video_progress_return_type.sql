--! up

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
) RETURNS SETOF video_log
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

  UPDATE video_log
  SET video_log_progress = LEAST(100, GREATEST(0, video_log_progress))
  WHERE video_id = p_video_id AND user_id = p_user_id;

  RETURN QUERY SELECT video_id, user_id, last_position_seconds, total_duration_seconds, video_log_progress, video_log_completed, watch_count, first_watched_at, last_watched_at, last_ip, last_device, last_user_agent, video_watched_at FROM video_log WHERE video_id = p_video_id AND user_id = p_user_id;
END
$fuvp$;

--! down

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

  UPDATE video_log
  SET video_log_progress = LEAST(100, GREATEST(0, video_log_progress))
  WHERE video_id = p_video_id AND user_id = p_user_id;
END
$fuvp$;