use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::model::User;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, &'static str> {
    User::all(pool).await.map_err(|_| "Error retrieving users")
}

pub async fn add_user(pool: &SqlitePool, user: User) -> Result<Vec<User>, &'static str> {
    User::add_user(pool, user)
        .await
        .map_err(|_| "Error adding user")
}

pub async fn delete_user(pool: &SqlitePool, id: String) -> Result<(), &'static str> {
    User::delete_user(pool, id)
        .await
        .map_err(|_| "Error adding user")
}
