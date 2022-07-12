use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
}

impl User {
    pub async fn all(connection: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users;
            "#
        )
        .fetch_all(connection)
        .await?;

        Ok(users)
    }

    pub async fn add_user(connection: &SqlitePool, user: User) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query!(
            r#"
            INSERT INTO users(id, name, phone)
            values(?, ?, ?);
            "#,
            user.id,
            user.name,
            user.phone
        )
        .fetch_all(connection)
        .await?;

        Ok(vec![user])
    }

    pub async fn delete_user(connection: &SqlitePool, id: String) -> Result<(), sqlx::Error> {
        let users = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = ?;
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }
}
