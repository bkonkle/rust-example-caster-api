CREATE TABLE "role_grants" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "role_key" TEXT NOT NULL,
    "profile_id" TEXT NOT NULL,
    "resource_table" TEXT NOT NULL,
    "resource_id" TEXT NOT NULL,

    PRIMARY KEY ("id")
);

ALTER TABLE "role_grants" ADD FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;

CREATE TRIGGER sync_role_grants_updated_at BEFORE UPDATE ON "role_grants" FOR EACH ROW EXECUTE PROCEDURE sync_updated_at();
