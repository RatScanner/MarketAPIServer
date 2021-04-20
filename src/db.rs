use crate::Config;
use sqlx::{Connection, Result, SqliteConnection};

pub async fn get_connection(conf: &Config) -> Result<SqliteConnection> {
    Ok(SqliteConnection::connect(&conf.database_url).await?)
}
