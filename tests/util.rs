use bytes::Bytes;
use market_api_server::{Config, ConfigHandle, Environment};
use std::env;
use warp::http::Response;

pub fn config() -> ConfigHandle {
    // Load env
    dotenv::dotenv().ok();

    Config::new(
        8081,
        env::var("DATABASE_URL").expect("Could not find env DATABASE_URL"),
        "1234",
        "",
        "",
        "",
        Environment::Test,
    )
}

pub trait ResponseExt {
    fn body_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned;
}

impl ResponseExt for Response<Bytes> {
    fn body_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(&self.into_body()).unwrap()
    }
}
