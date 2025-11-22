-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "user" 
(
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(50) NOT NULL,
    "game_id" uuid NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP,
    CONSTRAINT "fk_game_user"
        FOREIGN KEY ("game_id")
            REFERENCES "game" ("id")
            ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('user');