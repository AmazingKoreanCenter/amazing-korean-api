-- Add 'delete' value to admin_action_enum
-- This is needed for lesson_item deletion logging

ALTER TYPE admin_action_enum ADD VALUE IF NOT EXISTS 'delete';
