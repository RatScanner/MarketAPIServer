#[tokio::main]
async fn main() {
    // Load env
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    // Load conf and start
    let conf = market_api_server::Config::from_env();
    market_api_server::start(conf, [0, 0, 0, 0]).await;
}
