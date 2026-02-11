-- Phase 1A QA Fix: video vs video_tag 의미 분리
-- video = 비디오 제목/부제 번역, video_tag = 비디오 태그 번역
ALTER TYPE content_type_enum ADD VALUE IF NOT EXISTS 'video';
