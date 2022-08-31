use sqlx::{
    pool::{PoolConnection, PoolOptions},
    PgPool, Postgres, Result,
};

#[derive(Clone)]
pub struct Db(PgPool);

impl Db {
    pub async fn connect(database_url: &str) -> Result<Self> {
        Ok(Self(
            PoolOptions::new()
                .min_connections(1)
                .max_connections(10)
                .connect(database_url)
                .await?,
        ))
    }

    pub async fn conn(&self) -> Result<PoolConnection<Postgres>> {
        self.0.acquire().await
    }
}
