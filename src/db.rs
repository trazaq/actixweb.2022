use crate::model::User;
use r2d2::Pool;
use r2d2_sqlite::rusqlite::Error;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn init_pool(database_url: &str) -> Result<Pool<SqliteConnectionManager>, Error> {
    let manager = SqliteConnectionManager::file(database_url);
    let pool = r2d2::Pool::builder().max_size(10).build(manager).unwrap();
    Ok(pool)
}

pub async fn get_all_users(
    pool: &Pool<SqliteConnectionManager>,
) -> Result<Vec<User>, &'static str> {
    User::all(pool).await.map_err(|_| "Error retrieving users")
}

pub async fn add_user(
    pool: &Pool<SqliteConnectionManager>,
    user: User,
) -> Result<Vec<User>, &'static str> {
    User::add_user(pool, user)
        .await
        .map_err(|_| "Error adding user")
}

pub async fn delete_user(
    pool: &Pool<SqliteConnectionManager>,
    id: &str,
) -> Result<(), &'static str> {
    User::delete_user(pool, id)
        .await
        .map_err(|_| "Error deleting user")
}
