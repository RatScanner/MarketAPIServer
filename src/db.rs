use sqlx::{Connection, Result, SqliteConnection};
use std::env;

pub async fn get_db_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL").expect("Could not find env DATABASE_URL");
    Ok(SqliteConnection::connect(&database_url).await?)
}
