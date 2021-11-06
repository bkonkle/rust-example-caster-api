use sqlx::{Pool, Postgres};

/// The Shows entity servicea
#[derive(Default)]
pub struct ShowsService {
    pg_pool: Pool<Postgres>,
}

impl ShowsService {
    async fn get(&self, id: &str) -> Result<&str, sqlx::Error> {
        let countries = sqlx::query!(
            "
SELECT *
FROM \"User\"
WHERE id = $1
        ",
            id
        )
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok("Test")
    }
}
