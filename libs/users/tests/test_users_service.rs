use anyhow::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};
use std::sync::Arc;

use caster_users::{
    user_model::User,
    user_mutations::UpdateUserInput,
    users_service::{DefaultUsersService, UsersService},
};

mod users_factory;

#[tokio::test]
async fn test_users_service_get() -> Result<()> {
    let user = users_factory::create_user("test-username");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .into_connection(),
    );

    let service = DefaultUsersService::new(db.clone());

    let result = service.get(&user.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(user));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
            vec!["test-username".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_users_service_get_by_username() -> Result<()> {
    let user = users_factory::create_user("test-username");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .into_connection(),
    );

    let service = DefaultUsersService::new(db.clone());

    let result = service.get_by_username(&user.username, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(user));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."username" = $1 LIMIT $2"#,
            vec!["test-username".into(), 1u64.into()]
        )]
    );

    Ok(())
}

// TODO: test_users_service_get_by_username_with_roles

#[tokio::test]
async fn test_users_service_create() -> Result<()> {
    let user = users_factory::create_user("test-username");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .into_connection(),
    );

    let service = DefaultUsersService::new(db.clone());

    let result = service.create(&user.username).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, user);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "users" ("username") VALUES ($1) RETURNING "id", "created_at", "updated_at", "username", "is_active""#,
            vec![user.username.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_users_service_update() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let updated = User {
        username: "updated-username".to_string(),
        ..user.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()], vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultUsersService::new(db.clone());

    let result = service
        .update(
            &user.id,
            &UpdateUserInput {
                username: Some(updated.username.clone()),
                is_active: None,
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "users" SET "username" = $1 WHERE "users"."id" = $2 RETURNING "id", "created_at", "updated_at", "username", "is_active""#,
                vec![updated.username.into(), user.id.into()]
            )
        ]
    );

    Ok(())
}

// TODO: test_users_service_update_with_roles

#[tokio::test]
async fn test_users_service_delete() -> Result<()> {
    let user = users_factory::create_user("test-username");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultUsersService::new(db.clone());

    service.delete(&user.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "users" WHERE "users"."id" = $1"#,
                vec![user.id.into()]
            )
        ]
    );

    Ok(())
}