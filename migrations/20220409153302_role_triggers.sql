-- Create a trigger that validates the resource_table based on information_schema
create or replace function check_role_grant() returns trigger as $$
  begin
    if exists (
      select 1
        from information_schema.tables
      where table_schema='public'
        and table_type='BASE TABLE'
        and table_name=new.resource_table
    ) then
      return new;
    end if;

    raise exception 'resource_table must match an existing table_name';
  end;
$$ language plpgsql;

create trigger on_create_role_grant
before insert or update on role_grants
  for each row execute procedure check_role_grant();

-- Users

-- Delete any role_grants that match the user id
create or replace function on_delete_user() returns trigger as $$
  begin
    delete from role_grants where user_id = old.id;

    return old;
  end;
$$ language plpgsql;

-- Whenever a user is deleted, remove all the associated rolegrants as well
create trigger on_delete_user
before delete on users
  for each row execute procedure on_delete_user();

-- Profiles

-- Delete any RoleGrants that match the Profile id
create or replace function on_delete_profile() returns trigger as $$
  begin
    delete from role_grants
    where resource_id = old.id
      and resource_table = 'profiles';

    return old;
  end;
$$ language plpgsql;

-- Whenever a Show is deleted, remove all the associated RoleGrants as well
create trigger on_delete_profile
before delete on profiles
  for each row execute procedure on_delete_profile();
