#[async_std::main]
async fn main() {
    market_api_server::start().await;
}
