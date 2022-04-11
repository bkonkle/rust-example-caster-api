-- Messages

create table "messages"
(
    id         text         default gen_random_ulid() not null
        primary key,
    created_at timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at timestamp(3) default CURRENT_TIMESTAMP not null,

    text       text                                   not null,

    profile_id text                                   not null
        references profiles
            on update cascade on delete cascade,
    episode_id text                                   not null
        references episodes
            on update cascade on delete cascade
);

create trigger sync_messages_updated_at before update on messages for each row execute procedure sync_updated_at();
