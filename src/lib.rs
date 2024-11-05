#![deny(rust_2018_idioms)]

mod config;
mod db;
pub mod server;

pub use self::config::{Config, ConfigHandle, Environment};

pub async fn init(
    conf: ConfigHandle,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    // Enable logger
    match conf.env {
        config::Environment::Production => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Warn)
                .init();
        }
        config::Environment::Development => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .filter_module("sqlx", log::LevelFilter::Error)
                .init();
        }
        config::Environment::Test => (),
    }

    // Connect to database
    let db = db::Db::connect(&conf.database_url).await.unwrap();

    // Run migrations
    run_migrations(&db).await.unwrap();

    // Start server
    server::init(conf, db).await
}

pub async fn start(conf: ConfigHandle, addr: impl Into<std::net::IpAddr>) {
    let port = conf.port;
    let app = init(conf).await;
    warp::serve(app).run((addr.into(), port)).await;
}

async fn run_migrations(
    db: &db::Db,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = db.conn().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}
