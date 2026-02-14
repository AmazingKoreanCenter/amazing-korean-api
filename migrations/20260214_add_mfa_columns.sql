-- MFA (Multi-Factor Authentication) 컬럼 추가
-- Admin/HYMN 역할 사용자의 TOTP MFA 지원

ALTER TABLE users ADD COLUMN user_mfa_secret TEXT;
ALTER TABLE users ADD COLUMN user_mfa_enabled BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN user_mfa_backup_codes TEXT;
ALTER TABLE users ADD COLUMN user_mfa_enabled_at TIMESTAMPTZ;
