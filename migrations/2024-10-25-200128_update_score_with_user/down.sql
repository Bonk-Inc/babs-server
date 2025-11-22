-- This file should undo anything in `up.sql`
ALTER TABLE "score"
    DROP COLUMN "user_id" CASCADE;

ALTER TABLE "score"
    ALTER COLUMN "username" SET NOT NULL;