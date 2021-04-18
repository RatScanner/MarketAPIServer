use super::error::Error;
use warp::{
    http::{HeaderValue, StatusCode},
    Filter, Rejection,
};

pub fn authenticate() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::value("x-auth-key")
        .recover(|_| async { Err(Rejection::from(Error::from(StatusCode::UNAUTHORIZED))) })
        .unify()
        .and_then(|header_value: HeaderValue| async move {
            let auth_key = std::env::var("AUTH_KEY").expect("Could not find env AUTH_KEY");

            match header_value.to_str() {
                Ok(header_value) if header_value == auth_key => Ok(()),
                _ => Err(Rejection::from(Error::from(StatusCode::UNAUTHORIZED))),
            }
        })
        .untuple_one()
}
