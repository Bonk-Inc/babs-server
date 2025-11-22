CREATE TABLE IF NOT EXISTS "score"
(
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "username" VARCHAR(50) NOT NULL,
    "score" INTEGER NOT NULL,
    "is_hidden" BOOLEAN NOT NULL DEFAULT FALSE,
    "game_id" uuid NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP,
    CONSTRAINT "fk_game_score"
        FOREIGN KEY ("game_id")
            REFERENCES "game" ("id")
            ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('score');