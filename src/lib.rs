#![deny(rust_2018_idioms)]

mod db;
mod fetch;
mod server;
mod service;
mod state;

pub async fn start() {
    // Load env
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    // Enable logger
    env_logger::init();

    // Run migrations
    run_migrations().await.unwrap();

    // Init state
    let state = state::StateHandle::new();

    // Start service
    service::start(state.clone());

    // Start server
    server::start(state).await;
}

async fn run_migrations() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = db::get_db_connection().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}
