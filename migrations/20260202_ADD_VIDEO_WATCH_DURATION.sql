-- Migration: Add video_watch_duration_sec to video_log
-- Date: 2026-02-02
-- Description: 실제 시청 시간(초)을 기록하는 컬럼 추가

-- 시청 누적 시간 (초 단위)
ALTER TABLE video_log
ADD COLUMN video_watch_duration_sec INT NOT NULL DEFAULT 0;

-- 인덱스 (필요시 통계 쿼리용)
CREATE INDEX idx_video_log_duration ON video_log(video_watch_duration_sec);

COMMENT ON COLUMN video_log.video_watch_duration_sec IS '누적 시청 시간 (초 단위)';
