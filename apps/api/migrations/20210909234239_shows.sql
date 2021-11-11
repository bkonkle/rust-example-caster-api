CREATE TABLE "shows" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "title" TEXT NOT NULL,
    "summary" TEXT,
    "picture" TEXT,
    "content" JSONB,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    PRIMARY KEY ("id")
);
