#![deny(rust_2018_idioms)]

mod config;
mod db;
mod fetch;
mod server;
mod service;
mod state;

pub use self::config::{Config, ConfigHandle};

pub async fn start(conf: ConfigHandle) {
    // Enable logger
    match conf.env {
        config::Environment::Production => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Warn)
                .init();
        }
        config::Environment::Development | config::Environment::Test => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .filter_module("sqlx", log::LevelFilter::Error)
                .init();
        }
    }

    // Run migrations
    run_migrations(&conf).await.unwrap();

    // Init state
    let state = state::State::new();

    // Start service
    service::start(state.clone(), conf.clone());

    // Start server
    server::start(state, conf).await;
}

async fn run_migrations(
    conf: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = db::get_connection(conf).await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}
