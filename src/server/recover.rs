use super::error::Error;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(serde::Serialize)]
struct ErrorMessage<'a> {
    code: u16,
    message: Option<&'a str>,
}

pub async fn recover(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
    } else if let Some(err) = err.find::<Error>() {
        if err.code().is_server_error() {
            log::error!("catched: {:?} -> {}", err.source(), err.code());
        }
        code = err.code();
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
        || err.find::<warp::reject::InvalidQuery>().is_some()
        || err.find::<warp::reject::MissingHeader>().is_some()
    {
        code = StatusCode::BAD_REQUEST;
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        log::error!("unexpected error: {:?} -> {}", err, code);
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: code.canonical_reason(),
    });

    Ok(warp::reply::with_status(json, code))
}
