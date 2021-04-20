#![deny(rust_2018_idioms)]

mod config;
mod db;
mod fetch;
mod server;
mod service;
mod state;

pub use self::config::{Config, ConfigHandle};

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

    // Init state
    let state = state::State::new();

    // Start service
    if conf.service {
        service::start(state.clone(), db.clone());
    }

    // Start server
    server::init(state, conf, db).await
}

pub async fn start(conf: ConfigHandle) {
    let app = init(conf).await;
    warp::serve(app).run(([0, 0, 0, 0], 8081)).await;
}

async fn run_migrations(
    db: &db::Db,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = db.conn().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}
