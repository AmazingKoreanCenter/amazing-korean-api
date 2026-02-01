-- 2026-01-28: Vimeo API 연동을 위한 video 테이블 컬럼 추가
-- video_duration: 영상 길이 (초 단위)
-- video_thumbnail: 영상 썸네일 URL

ALTER TABLE video ADD COLUMN IF NOT EXISTS video_duration INT;
ALTER TABLE video ADD COLUMN IF NOT EXISTS video_thumbnail TEXT;

-- 코멘트 추가
COMMENT ON COLUMN video.video_duration IS '영상 길이 (초 단위, Vimeo API에서 가져옴)';
COMMENT ON COLUMN video.video_thumbnail IS '영상 썸네일 URL (Vimeo API에서 가져옴)';
