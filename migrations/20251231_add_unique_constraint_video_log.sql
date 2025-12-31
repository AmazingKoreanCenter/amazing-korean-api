ALTER TABLE video_log
ADD CONSTRAINT uk_video_log_user_video UNIQUE (user_id, video_id);
