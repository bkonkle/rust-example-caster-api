CREATE TABLE "role_grants" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "role_key" TEXT NOT NULL,
    "profile_id" TEXT NOT NULL,
    "subject_table" TEXT NOT NULL,
    "subject_id" TEXT NOT NULL,

    PRIMARY KEY ("id")
);

ALTER TABLE "role_grants" ADD FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;
