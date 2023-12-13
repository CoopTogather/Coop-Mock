-- This file should undo anything in `up.sql`
ALTER TABLE endpoints_setting DROP COLUMN method;
ALTER TABLE endpoints_setting DROP COLUMN enabled;