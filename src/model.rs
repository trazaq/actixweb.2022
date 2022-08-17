use r2d2::{Pool};
use r2d2_sqlite::rusqlite::Error;
use r2d2_sqlite::SqliteConnectionManager;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
}

impl User {
    pub async fn all(pool: &Pool<SqliteConnectionManager>) -> Result<Vec<User>, Error> {
        let conn = pool.get().expect("Error getting Connection From Pool");
        let mut stmt = conn.prepare(r#"SELECT id, name, phone FROM users;"#).expect("Error Preparing SQL Statement");
        let users = stmt
            .query_map([], |r| {
                Ok(User {
                    id: r.get_unwrap(0),
                    name: r.get_unwrap(1),
                    phone: r.get_unwrap(2),
                })
            })?;

        if let Some(size) = users.size_hint().1 {
            log::info!("Size hint: {}", size);
            let mut result = Vec::with_capacity(size);
            for user in users {
                result.push(user?);
            }
            Ok(result)
        } else {
            log::info!("No size Hint");
            let mut result = Vec::new();
            for user in users {
                result.push(user?);
            }
            Ok(result)
        }

    }

    /*    pub async fn add_user(connection: &SqlitePool, user: User) -> Result<Vec<User>, sqlx::Error> {
        let _users = sqlx::query!(
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
        let _users = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = ?;
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }*/
}
