use actix_web::http::header::q;
use r2d2::Pool;
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
        let mut stmt = conn
            .prepare(r#"SELECT id, name, phone FROM users;"#)
            .expect("Error Preparing SQL Statement");
        let users = stmt.query_map([], |r| {
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

    pub async fn add_user(
        pool: &Pool<SqliteConnectionManager>,
        user: User,
    ) -> Result<Vec<User>, Error> {
        let conn = pool.get().expect("Error getting Connection From Pool");
        let mut stmt = conn
            .prepare(r#"INSERT INTO users (id, name, phone) VALUES (?1, ?2, ?3);"#)
            .expect("Error Preparing SQL Statement");
        match stmt.execute(&[&user.id, &user.name, &user.phone]) {
            Ok(_) => Ok(vec![user]),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_user(
        pool: &Pool<SqliteConnectionManager>,
        id: String,
    ) -> Result<(), Error> {
        let conn = pool.get().expect("Error getting Connection From Pool");
        let mut stmt = conn
            .prepare(r#"DELETE FROM users WHERE id = ?;"#)
            .expect("Error Preparing SQL Statement");

        match stmt.execute(&[&id]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
