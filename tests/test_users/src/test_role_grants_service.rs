use anyhow::Result;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};
use std::sync::Arc;
use pretty_assertions::assert_eq;

use crate::{role_grant_factory, user_factory};
use caster_users::{
    role_grant_model::CreateRoleGrantInput,
    role_grants_service::{DefaultRoleGrantsService, RoleGrantsService},
};

#[tokio::test]
async fn test_role_grants_service_get() -> Result<()> {
    let user = user_factory::create_user_with_username("test-username");
    let grant =
        role_grant_factory::create_role_grant_for_user("profiles", "profile-id", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]])
            .into_connection(),
    );

    let service = DefaultRoleGrantsService::new(db.clone());

    let result = service.get(&grant.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(grant));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "role_grants"."id", "role_grants"."created_at", "role_grants"."updated_at", "role_grants"."role_key", "role_grants"."user_id", "role_grants"."resource_table", "role_grants"."resource_id" FROM "role_grants" WHERE "role_grants"."id" = $1 LIMIT $2"#,
            vec![format!("{}-{}", user.id, "profile-id").into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_role_grants_service_create() -> Result<()> {
    let user = user_factory::create_user_with_username("test-username");
    let grant =
        role_grant_factory::create_role_grant_for_user("profiles", "profile-id", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]])
            .into_connection(),
    );

    let service = DefaultRoleGrantsService::new(db.clone());

    let result = service
        .create(&CreateRoleGrantInput {
            role_key: grant.role_key.clone(),
            user_id: user.id.clone(),
            resource_table: "profiles".to_string(),
            resource_id: "profile-id".to_string(),
        })
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, grant);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "role_grants" ("role_key", "user_id", "resource_table", "resource_id") VALUES ($1, $2, $3, $4) RETURNING "id", "created_at", "updated_at", "role_key", "user_id", "resource_table", "resource_id""#,
            vec![
                grant.role_key.into(),
                grant.user_id.into(),
                grant.resource_table.into(),
                grant.resource_id.into()
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_role_grants_service_delete() -> Result<()> {
    let user = user_factory::create_user_with_username("test-username");
    let grant =
        role_grant_factory::create_role_grant_for_user("profiles", "profile-id", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultRoleGrantsService::new(db.clone());

    service.delete(&grant.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "role_grants"."id", "role_grants"."created_at", "role_grants"."updated_at", "role_grants"."role_key", "role_grants"."user_id", "role_grants"."resource_table", "role_grants"."resource_id" FROM "role_grants" WHERE "role_grants"."id" = $1 LIMIT $2"#,
                vec![grant.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "role_grants" WHERE "role_grants"."id" = $1"#,
                vec![grant.id.into()]
            )
        ]
    );

    Ok(())
}
