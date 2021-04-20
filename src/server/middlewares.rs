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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_authenticate_success() {
        warp::test::request()
            .header("x-auth-key", "1234")
            .filter(&authenticate("1234".to_string()))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_authenticate_error() {
        warp::test::request()
            .filter(&authenticate("1234".to_string()))
            .await
            .unwrap_err();

        warp::test::request()
            .header("x-auth-key", "xxxx")
            .filter(&authenticate("1234".to_string()))
            .await
            .unwrap_err();

        warp::test::request()
            .header("x-auth-key", "1234 ")
            .filter(&authenticate("1234".to_string()))
            .await
            .unwrap_err();

        warp::test::request()
            .header("x-auth-key", "ABC")
            .filter(&authenticate("abc".to_string()))
            .await
            .unwrap_err();
    }
}
