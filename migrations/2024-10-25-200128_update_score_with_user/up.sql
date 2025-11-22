-- Your SQL goes here
ALTER TABLE "score"
    ADD COLUMN "user_id" uuid;

ALTER TABLE "score"
    ALTER COLUMN "username" DROP NOT NULL;

ALTER TABLE "score"
    ADD CONSTRAINT "fk_user_score"
        FOREIGN KEY ("user_id")
            REFERENCES "user" ("id")
            ON DELETE CASCADE