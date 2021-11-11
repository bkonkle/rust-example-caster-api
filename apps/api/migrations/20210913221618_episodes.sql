CREATE TABLE "episodes" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "title" TEXT NOT NULL,
    "summary" TEXT,
    "picture" TEXT,
    "content" JSONB,
    "show_id" TEXT NOT NULL,

    PRIMARY KEY ("id")
);

ALTER TABLE "episodes" ADD FOREIGN KEY ("show_id") REFERENCES "shows"("id") ON DELETE CASCADE ON UPDATE CASCADE;
