CREATE TABLE "messages" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "text" TEXT NOT NULL,
    "profile_id" TEXT NOT NULL,
    "episode_id" TEXT NOT NULL,

    PRIMARY KEY ("id")
);

ALTER TABLE "messages" ADD FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "messages" ADD FOREIGN KEY ("episode_id") REFERENCES "episodes"("id") ON DELETE CASCADE ON UPDATE CASCADE;
