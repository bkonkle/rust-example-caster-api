-- Create a trigger that validates the resource_table based on information_schema
CREATE OR REPLACE FUNCTION check_role_grant() RETURNS trigger as $$
  BEGIN
    IF EXISTS (
      SELECT 1
        FROM information_schema.tables
      WHERE table_schema='public'
        AND table_type='BASE TABLE'
        AND table_name=NEW."resource_table"
    ) THEN
      RETURN NEW;
    END IF;

    RAISE EXCEPTION 'resource_table must match an existing table_name';
  END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_create_role_grant
BEFORE INSERT OR UPDATE ON "role_grants"
  FOR EACH ROW EXECUTE PROCEDURE check_role_grant();

-- Users

-- Delete any role_grants that match the User id
CREATE OR REPLACE FUNCTION on_delete_user() RETURNS trigger AS $$
  BEGIN
    DELETE FROM "role_grants" WHERE "user_id" = OLD.id;

    RETURN OLD;
  END;
$$ LANGUAGE plpgsql;

-- Whenever a User is deleted, remove all the associated RoleGrants as well
CREATE TRIGGER on_delete_user
BEFORE DELETE ON "users"
  FOR EACH ROW EXECUTE PROCEDURE on_delete_user();

-- Shows

-- Delete any RoleGrants that match the Show id
CREATE OR REPLACE FUNCTION on_delete_show() RETURNS trigger AS $$
  BEGIN
    DELETE FROM "role_grants"
    WHERE "resource_id" = OLD.id
      AND "resource_table" = 'shows';

    RETURN OLD;
  END;
$$ LANGUAGE plpgsql;

-- Whenever a Show is deleted, remove all the associated RoleGrants as well
CREATE TRIGGER on_delete_show
BEFORE DELETE ON "shows"
  FOR EACH ROW EXECUTE PROCEDURE on_delete_show();

-- Episodes

-- Delete any RoleGrants that match the Episode id
CREATE OR REPLACE FUNCTION on_delete_episode() RETURNS trigger AS $$
  BEGIN
    DELETE FROM "role_grants"
    WHERE "resource_id" = OLD.id
      AND "resource_table" = 'episodes';

    RETURN OLD;
  END;
$$ LANGUAGE plpgsql;

-- Whenever an Episode is deleted, remove all the associated RoleGrants as well
CREATE TRIGGER on_delete_episode
BEFORE DELETE ON "episodes"
  FOR EACH ROW EXECUTE PROCEDURE on_delete_episode();

