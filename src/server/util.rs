use super::error::Error;
use std::convert::AsRef;
use std::str::FromStr;
use warp::http::StatusCode;

#[derive(Clone, Debug)]
pub struct PercentDecoded(String);

impl AsRef<str> for PercentDecoded {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for PercentDecoded {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = percent_encoding::percent_decode_str(s)
            .decode_utf8()
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .to_string();
        Ok(PercentDecoded(s))
    }
}
