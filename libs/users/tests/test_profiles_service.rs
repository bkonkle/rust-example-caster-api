use anyhow::Result;
use caster_utils::pagination::ManyResponse;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction, Value};
use std::sync::Arc;

use caster_users::{
    profile_model::{Model as ProfileModel, ProfileList},
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
    profile_queries::{ProfileCondition, ProfilesOrderBy},
    profiles_service::{DefaultProfilesService, ProfilesService},
};

mod users_factory;

#[tokio::test]
async fn test_profiles_service_get() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service.get(&profile.id, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(profile.into()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
            vec!["test-profile-com".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_with_related() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(profile.clone(), user.clone())]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service.get(&profile.id, &true).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(profile.into_profile_with_user(user)));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."content" AS "A_content", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."id" = $1 LIMIT $2"#,
            vec!["test-profile-com".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user);

    let other_user = users_factory::create_user("test-user-2");
    let other_profile = users_factory::create_profile_for_user("test+2@profile.com", other_user);

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone(), other_profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .get_many(
            Some(ProfileCondition {
                email: Some("test@profile.com".to_string()),
                display_name: None,
                city: None,
                state_province: None,
                user_id: None,
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
            r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."email" = $1"#,
            vec!["test@profile.com".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_with_related() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let other_user = users_factory::create_user("test-user-2");
    let other_profile =
        users_factory::create_profile_for_user("test+2@profile.com", other_user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![
                (profile.clone(), user.clone()),
                (other_profile.clone(), other_user.clone()),
            ]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .get_many(
            Some(ProfileCondition {
                email: Some("test@profile.com".to_string()),
                display_name: None,
                city: None,
                state_province: None,
                user_id: None,
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
            r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."content" AS "A_content", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."email" = $1"#,
            vec!["test@profile.com".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_pagination() -> Result<()> {
    let users = vec![
        users_factory::create_user("test-user-1"),
        users_factory::create_user("test-user-2"),
        users_factory::create_user("test-user-3"),
        users_factory::create_user("test-user-4"),
        users_factory::create_user("test-user-5"),
    ];

    let profiles: Vec<ProfileModel> = users
        .into_iter()
        .enumerate()
        .map(|(i, user)| {
            users_factory::create_profile_for_user(&format!("test+{}@profile.com", i), user)
        })
        .collect();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                profiles.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

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
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" ORDER BY "profiles"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" ORDER BY "profiles"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_get_many_pagination_with_related() -> Result<()> {
    let users = vec![
        users_factory::create_user("test-user-1"),
        users_factory::create_user("test-user-2"),
        users_factory::create_user("test-user-3"),
        users_factory::create_user("test-user-4"),
        users_factory::create_user("test-user-5"),
    ];

    let profiles = vec![
        (
            users_factory::create_profile_for_user("test+1@profile.com", users[0].clone()),
            users[0].clone(),
        ),
        (
            users_factory::create_profile_for_user("test+2@profile.com", users[1].clone()),
            users[1].clone(),
        ),
        (
            users_factory::create_profile_for_user("test+3@profile.com", users[2].clone()),
            users[2].clone(),
        ),
        (
            users_factory::create_profile_for_user("test+4@profile.com", users[3].clone()),
            users[3].clone(),
        ),
        (
            users_factory::create_profile_for_user("test+5@profile.com", users[4].clone()),
            users[4].clone(),
        ),
    ];

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                profiles.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

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
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."content" AS "A_content", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" ORDER BY "profiles"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."content" AS "A_content", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" ORDER BY "profiles"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_create() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .create(
            &CreateProfileInput {
                email: profile.email.clone(),
                display_name: profile.display_name.clone(),
                picture: profile.picture.clone(),
                content: profile.content.clone(),
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
            r#"INSERT INTO "profiles" ("email", "display_name", "picture", "content", "city", "state_province", "user_id") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "content", "city", "state_province", "user_id""#,
            vec![
                profile.email.into(),
                profile.display_name.into(),
                profile.picture.into(),
                profile.content.into(),
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
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .append_query_results(vec![vec![user.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .create(
            &CreateProfileInput {
                email: profile.email.clone(),
                display_name: profile.display_name.clone(),
                picture: profile.picture.clone(),
                content: profile.content.clone(),
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

    assert_eq!(result, profile.clone().into_profile_with_user(user));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "profiles" ("email", "display_name", "picture", "content", "city", "state_province", "user_id") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "content", "city", "state_province", "user_id""#,
                vec![
                    profile.email.into(),
                    profile.display_name.into(),
                    profile.picture.into(),
                    profile.content.into(),
                    profile.city.into(),
                    profile.state_province.into(),
                    profile.user_id.into(),
                ]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec!["test-username".into(), 1u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_update() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let updated = ProfileModel {
        email: "test+updated@profile.com".to_string(),
        ..profile.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()], vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .update(
            &user.id,
            &UpdateProfileInput {
                email: Some(updated.email.clone()),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
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
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "profiles" SET "email" = $1, "user_id" = $2 WHERE "profiles"."id" = $3 RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "content", "city", "state_province", "user_id""#,
                vec![updated.email.into(), user.id.into(), profile.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_update_with_related() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let updated = ProfileModel {
        email: "test+updated@profile.com".to_string(),
        ..profile.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(profile.clone(), user.clone())]])
            .append_query_results(vec![vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

    let result = service
        .update(
            &user.id,
            &UpdateProfileInput {
                email: Some(updated.email.clone()),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
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
                r#"SELECT "profiles"."id" AS "A_id", "profiles"."created_at" AS "A_created_at", "profiles"."updated_at" AS "A_updated_at", "profiles"."email" AS "A_email", "profiles"."display_name" AS "A_display_name", "profiles"."picture" AS "A_picture", "profiles"."content" AS "A_content", "profiles"."city" AS "A_city", "profiles"."state_province" AS "A_state_province", "profiles"."user_id" AS "A_user_id", "users"."id" AS "B_id", "users"."created_at" AS "B_created_at", "users"."updated_at" AS "B_updated_at", "users"."username" AS "B_username", "users"."is_active" AS "B_is_active" FROM "profiles" LEFT JOIN "users" ON "profiles"."user_id" = "users"."id" WHERE "profiles"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "profiles" SET "email" = $1, "user_id" = $2 WHERE "profiles"."id" = $3 RETURNING "id", "created_at", "updated_at", "email", "display_name", "picture", "content", "city", "state_province", "user_id""#,
                vec![updated.email.into(), user.id.into(), profile.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_profiles_service_delete() -> Result<()> {
    let user = users_factory::create_user("test-username");
    let profile = users_factory::create_profile_for_user("test@profile.com", user.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![profile.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultProfilesService::new(db.clone());

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
                r#"SELECT "profiles"."id", "profiles"."created_at", "profiles"."updated_at", "profiles"."email", "profiles"."display_name", "profiles"."picture", "profiles"."content", "profiles"."city", "profiles"."state_province", "profiles"."user_id" FROM "profiles" WHERE "profiles"."id" = $1 LIMIT $2"#,
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
