create table role_grants
(
    id             text         default gen_random_ulid() not null
        primary key,
    created_at     timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at     timestamp(3) default CURRENT_TIMESTAMP not null,

    role_key       text                                   not null,
    resource_table text                                   not null,
    resource_id    text                                   not null,

    user_id        text                                   not null
        references users
            on update cascade on delete cascade
);

create trigger sync_role_grants_updated_at before update on role_grants for each row execute procedure sync_updated_at();
