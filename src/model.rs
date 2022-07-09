use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone: String,
}

impl User {
    pub async fn all(connection: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            "#
        )
        .fetch_all(connection)
        .await?;

        Ok(users)
    }
}
