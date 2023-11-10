-- This file should undo anything in `up.sql`

ALTER TABLE endpoints_setting ALTER COLUMN options TYPE JSON;