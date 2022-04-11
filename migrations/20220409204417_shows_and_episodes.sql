-- Shows

create table shows
(
    id         text         default gen_random_ulid() not null
        primary key,
    created_at timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at timestamp(3) default CURRENT_TIMESTAMP not null,

    title      text                                   not null,
    summary    text,
    picture    text,
    content    jsonb
);

-- Delete any RoleGrants that match the Show id
create or replace function on_delete_show() returns trigger as $$
  begin
    delete from role_grants
    where resource_id = old.id
      and resource_table = 'shows';

    return old;
  end;
$$ language plpgsql;

-- Whenever a Show is deleted, remove all the associated RoleGrants as well
create trigger on_delete_show
before delete on shows
  for each row execute procedure on_delete_show();

create trigger sync_shows_updated_at before update on shows for each row execute procedure sync_updated_at();

-- Episodes

create table episodes
(
    id          text         default gen_random_ulid() not null
        primary key,
    created_at  timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at  timestamp(3) default CURRENT_TIMESTAMP not null,

    title       text                                   not null,
    summary     text,
    picture     text,
    content     jsonb,

    show_id text                                       not null
        references shows
            on update cascade on delete cascade
);

create trigger sync_episodes_updated_at before update on episodes for each row execute procedure sync_updated_at();

-- Delete any RoleGrants that match the Episode id
create or replace function on_delete_episode() returns trigger as $$
  begin
    delete from role_grants
    where resource_id = old.id
      and resource_table = 'episodes';

    return old;
  end;
$$ language plpgsql;

-- Whenever a Episode is deleted, remove all the associated RoleGrants as well
create trigger on_delete_episode
before delete on episodes
  for each row execute procedure on_delete_episode();
