use bytes::Bytes;
use market_api_server::{Config, ConfigHandle, Environment};
use warp::http::Response;

pub fn config() -> ConfigHandle {
    Config::new(":memory:", "1234", false, Environment::Test)
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
