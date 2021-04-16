use super::error;
use crate::state::StateHandle;
use tide::http::StatusCode;

type BoxFuture<'a, T> =
    std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = T> + Send + 'a>>;

// Custom 404 repsonse
// TODO replace middleware with something better
pub fn not_found(
    req: tide::Request<StateHandle>,
    next: tide::Next<'_, StateHandle>,
) -> BoxFuture<'_, tide::Response> {
    Box::pin(async move {
        let response = next.run(req).await;

        if response.status() == StatusCode::NOT_FOUND {
            error::error_response(StatusCode::NOT_FOUND, None)
        } else {
            response
        }
    })
}

pub fn authenticate(
    req: tide::Request<StateHandle>,
    next: tide::Next<'_, StateHandle>,
) -> BoxFuture<'_, tide::Response> {
    Box::pin(async move {
        let auth_key = std::env::var("AUTH_KEY").expect("Could not find env AUTH_KEY");

        let auth_key_header = req.header("x-auth-key");

        match auth_key_header {
            Some(key) if key == auth_key => next.run(req).await,
            _ => error::error_response(StatusCode::UNAUTHORIZED, None),
        }
    })
}
