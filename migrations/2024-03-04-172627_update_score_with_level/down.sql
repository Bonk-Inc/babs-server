ALTER TABLE "score"
    DROP COLUMN "level_id" CASCADE;

ALTER TABLE "score"
    ADD COLUMN "game_id" uuid;

ALTER TABLE "score"
    ADD CONSTRAINT "fk_game_score"
        FOREIGN KEY ("game_id")
            REFERENCES "game" ("id")
            ON DELETE CASCADE