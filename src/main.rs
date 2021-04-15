#![deny(rust_2018_idioms)]
#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;

mod db;
mod fetch;
mod server;

mod schema;

#[async_std::main]
async fn main() {
    // Enable logger
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();
    env_logger::init();

    // Start server
    server::start().await;
}
