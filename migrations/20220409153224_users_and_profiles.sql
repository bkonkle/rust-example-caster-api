-- Users

create table users
(
    id         text         default gen_random_ulid() not null
        primary key,
    created_at timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at timestamp(3) default CURRENT_TIMESTAMP not null,

    username   text                                   not null,
    is_active  boolean      default true              not null
);

create unique index users__username__unique
    on users (username);

create trigger sync_users_updated_at before update on users for each row execute procedure sync_updated_at();

-- Profiles

create table profiles
(
    id             text         default gen_random_ulid() not null
        primary key,
    created_at     timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at     timestamp(3) default CURRENT_TIMESTAMP not null,

    email          text                                   not null,
    display_name   text,
    picture        text,
    content        jsonb,
    city           text,
    state_province text,

    user_id        text
        references users
            on update cascade on delete set null
);

create unique index profiles__user_id__unique
    on profiles (user_id);

create trigger sync_profiles_updated_at before update on profiles for each row execute procedure sync_updated_at();
