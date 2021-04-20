use sqlx::{
    pool::{PoolConnection, PoolOptions},
    Result, Sqlite, SqlitePool,
};

#[derive(Clone)]
pub struct Db(SqlitePool);

impl Db {
    pub async fn connect(database_url: &str) -> Result<Self> {
        Ok(Self(
            PoolOptions::new()
                .min_connections(1)
                .max_connections(1)
                .connect(database_url)
                .await?,
        ))
    }

    pub async fn conn(&self) -> Result<PoolConnection<Sqlite>> {
        self.0.acquire().await
    }
}
