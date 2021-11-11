CREATE TABLE "users" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "username" TEXT NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT true,

    PRIMARY KEY ("id")
);

CREATE TABLE "profiles" (
    "id" TEXT NOT NULL DEFAULT ulid_generate(),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "email" TEXT NOT NULL,
    "display_name" TEXT,
    "picture" TEXT,
    "content" JSONB,
    "city" TEXT,
    "state_province" TEXT,
    "user_id" TEXT,

    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "users.users__username__key" ON "users"("username");

CREATE UNIQUE INDEX "profiles.profiles__user_id__key" ON "profiles"("user_id");

ALTER TABLE "profiles" ADD FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE SET NULL ON UPDATE CASCADE;
