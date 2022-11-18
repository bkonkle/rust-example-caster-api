use anyhow::Result;
use async_graphql::MaybeUndefined::Undefined;
use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};
use std::sync::Arc;

use crate::{
    profiles::{
        model::{Model, ProfileList},
        mutations::{CreateProfileInput, UpdateProfileInput},
        queries::{ProfileCondition, ProfilesOrderBy},
        service::{DefaultProfilesService, ProfilesService},
    },
    users::model::User,
};
use caster_utils::pagination::ManyResponse;

#[tokio::test]
async fn test_profiles_service_get() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service.get(&profile.id, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(profile.clone().into()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
            vec![profile.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_with_related() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(profile.clone(), user.clone())]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service.get(&profile.id, &true).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(profile.clone().into_profile_with_user(user)));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."id" = $1 LIMIT $2"#,
            vec![profile.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let mut other_user: User = Faker.fake();
    other_user.roles = vec![];
    other_user.username = "test-user-2".to_string();

    let mut other_profile: Model = Faker.fake();
    other_profile.user_id = Some(user.id.clone());
    other_profile.email = "test+2@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone(), other_profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .get_many(
            Some(ProfileCondition {
                email: Some("test@profile.com".to_string()),
                display_name: None,
                city: None,
                state_province: None,
                user_id: None,
                ids_in: None,
            }),
            None,
            None,
            None,
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: vec![profile.into(), other_profile.into()],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."email" = $1"#,
            vec!["test@profile.com".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_with_related() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let mut other_user: User = Faker.fake();
    other_user.roles = vec![];
    other_user.username = "test-user-2".to_string();

    let mut other_profile: Model = Faker.fake();
    other_profile.user_id = Some(other_user.id.clone());
    other_profile.email = "test+2@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![
                (profile.clone(), user.clone()),
                (other_profile.clone(), other_user.clone()),
            ]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .get_many(
            Some(ProfileCondition {
                email: Some("test@profile.com".to_string()),
                display_name: None,
                city: None,
                state_province: None,
                user_id: None,
                ids_in: None,
            }),
            None,
            None,
            None,
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: vec![
                profile.into_profile_with_user(user),
                other_profile.into_profile_with_user(other_user)
            ],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."email" = $1"#,
            vec!["test@profile.com".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_pagination() -> Result<()> {
    let mut user1: User = Faker.fake();
    user1.roles = vec![];
    user1.username = "test-user-1".to_string();

    let mut user2: User = Faker.fake();
    user2.roles = vec![];
    user2.username = "test-user-2".to_string();

    let mut user3: User = Faker.fake();
    user3.roles = vec![];
    user3.username = "test-user-3".to_string();

    let mut user4: User = Faker.fake();
    user4.roles = vec![];
    user4.username = "test-user-4".to_string();

    let mut user5: User = Faker.fake();
    user5.roles = vec![];
    user5.username = "test-user-5".to_string();

    let users = vec![user1, user2, user3, user4, user5];

    let profiles: Vec<Model> = users
        .into_iter()
        .enumerate()
        .map(|(i, user)| {
            let mut profile: Model = Faker.fake();
            profile.email = format!("test+{}@profile.com", i);
            profile.user_id = Some(user.id);

            profile
        })
        .collect();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<sea_orm::Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                profiles.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .get_many(
            None,
            Some(vec![ProfilesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    let data: ProfileList = profiles.into();

    assert_eq!(
        result,
        ManyResponse {
            data: data.into(),
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" ORDER BY "profiles"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" ORDER BY "profiles"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_pagination_with_related() -> Result<()> {
    let mut user1: User = Faker.fake();
    user1.roles = vec![];
    user1.username = "test-user-1".to_string();

    let mut user2: User = Faker.fake();
    user2.roles = vec![];
    user2.username = "test-user-2".to_string();

    let mut user3: User = Faker.fake();
    user3.roles = vec![];
    user3.username = "test-user-3".to_string();

    let mut user4: User = Faker.fake();
    user4.roles = vec![];
    user4.username = "test-user-4".to_string();

    let mut user5: User = Faker.fake();
    user5.roles = vec![];
    user5.username = "test-user-5".to_string();

    let users = vec![user1, user2, user3, user4, user5];

    let profiles: Vec<(Model, User)> = users
        .into_iter()
        .enumerate()
        .map(|(i, user)| {
            let mut profile: Model = Faker.fake();
            profile.email = format!("test+{}@profile.com", i);
            profile.user_id = Some(user.id.clone());

            (profile, user)
        })
        .collect();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<sea_orm::Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                profiles.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .get_many(
            None,
            Some(vec![ProfilesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: profiles
                .into_iter()
                .map(|(profile, user)| profile.into_profile_with_user(user))
                .collect(),
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" ORDER BY "profiles"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" ORDER BY "profiles"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_create() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .create(
            &CreateProfileInput {
                email: profile.email.clone(),
                display_name: profile.display_name.clone(),
                picture: profile.picture.clone(),
                city: profile.city.clone(),
                state_province: profile.state_province.clone(),
                user_id: user.id.clone(),
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, profile.clone().into());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "profiles" ("email", "display_name", "picture", "city", "state_province", "user_id") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "city", "state_province", "user_id""#,
            vec![
                profile.email.into(),
                profile.display_name.into(),
                profile.picture.into(),
                profile.city.into(),
                profile.state_province.into(),
                profile.user_id.into(),
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_create_with_related() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .append_query_results(vec![vec![user.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .create(
            &CreateProfileInput {
                email: profile.email.clone(),
                display_name: profile.display_name.clone(),
                picture: profile.picture.clone(),
                city: profile.city.clone(),
                state_province: profile.state_province.clone(),
                user_id: user.id.clone(),
            },
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, profile.clone().into_profile_with_user(user.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "profiles" ("email", "display_name", "picture", "city", "state_province", "user_id") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "city", "state_province", "user_id""#,
                vec![
                    profile.email.into(),
                    profile.display_name.into(),
                    profile.picture.into(),
                    profile.city.into(),
                    profile.state_province.into(),
                    profile.user_id.into(),
                ]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![user.id.into(), 1u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_update() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let updated = Model {
        email: "test+updated@profile.com".to_string(),
        ..profile.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()], vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .update(
            &user.id,
            &UpdateProfileInput {
                email: Some(updated.email.clone()),
                display_name: Undefined,
                picture: Undefined,
                city: Undefined,
                state_province: Undefined,
                user_id: Some(user.id.clone()),
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone().into());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "profiles" SET "email" = $1, "user_id" = $2 WHERE "profiles"."id" = $3 RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "city", "state_province", "user_id""#,
                vec![updated.email.into(), user.id.into(), profile.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_update_with_related() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let updated = Model {
        email: "test+updated@profile.com".to_string(),
        ..profile.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(profile.clone(), user.clone())]])
            .append_query_results(vec![vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    let result = service
        .update(
            &user.id,
            &UpdateProfileInput {
                email: Some(updated.email.clone()),
                display_name: Undefined,
                picture: Undefined,
                city: Undefined,
                state_province: Undefined,
                user_id: Some(user.id.clone()),
            },
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone().into_profile_with_user(user.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "profiles" SET "email" = $1, "user_id" = $2 WHERE "profiles"."id" = $3 RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "city", "state_province", "user_id""#,
                vec![updated.email.into(), user.id.into(), profile.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_delete() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut profile: Model = Faker.fake();
    profile.user_id = Some(user.id.clone());
    profile.email = "test@profile.com".to_string();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(&db);

    service.delete(&profile.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
                vec![profile.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "profiles" WHERE "profiles"."id" = $1"#,
                vec![profile.id.into()]
            )
        ]
    );

    Ok(())
}
