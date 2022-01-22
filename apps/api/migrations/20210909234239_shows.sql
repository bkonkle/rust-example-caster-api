CREATE TABLE "shows" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "title" TEXT NOT NULL,
    "summary" TEXT,
    "picture" TEXT,
    "content" JSONB,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY ("id")
);

CREATE TRIGGER sync_shows_updated_at BEFORE UPDATE ON "shows" FOR EACH ROW EXECUTE PROCEDURE sync_updated_at();
