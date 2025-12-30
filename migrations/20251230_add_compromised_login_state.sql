ALTER TYPE login_state_enum ADD VALUE IF NOT EXISTS 'compromised';
ALTER TYPE login_event_enum ADD VALUE IF NOT EXISTS 'reuse_detected';
