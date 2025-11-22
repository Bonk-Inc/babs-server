ALTER TABLE "score"
    ADD COLUMN "level_id" uuid;

ALTER TABLE "score"
    DROP COLUMN "game_id" CASCADE;

ALTER TABLE "score"
    ADD CONSTRAINT "fk_level_score"
        FOREIGN KEY ("level_id")
            REFERENCES "level" ("id")
            ON DELETE CASCADE