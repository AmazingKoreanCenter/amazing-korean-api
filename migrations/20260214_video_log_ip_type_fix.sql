-- video_log.video_last_ip_log: inet → TEXT (암호화된 IP 저장을 위해)
ALTER TABLE video_log ALTER COLUMN video_last_ip_log TYPE TEXT USING video_last_ip_log::TEXT;
