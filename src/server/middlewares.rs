use super::error::Error;
use warp::{
    http::{HeaderValue, StatusCode},
    Filter, Rejection,
};

pub fn authenticate(auth_key: String) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::value("x-auth-key")
        .recover(|_| async { Err(Rejection::from(Error::from(StatusCode::UNAUTHORIZED))) })
        .unify()
        .and(warp::any().map(move || auth_key.clone()))
        .and_then(|header_value: HeaderValue, auth_key: String| async move {
            match header_value.to_str() {
                Ok(header_value) if header_value == auth_key => Ok(()),
                _ => Err(Rejection::from(Error::from(StatusCode::UNAUTHORIZED))),
            }
        })
        .untuple_one()
}
