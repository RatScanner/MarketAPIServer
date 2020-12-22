use tide::http::StatusCode;

#[derive(Debug)]
pub enum Error {
    StatusCode(StatusCode),
    // InternalServerError,
}

impl From<StatusCode> for Error {
    fn from(s: StatusCode) -> Self {
        Error::StatusCode(s)
    }
}

pub async fn catch<F, Fut>(f: F) -> tide::Response
where
    F: FnOnce() -> Fut,
    Fut: async_std::future::Future<Output = Result<tide::Response, Error>>,
{
    match f().await {
        Ok(v) => v,
        Err(e) => match e {
            Error::StatusCode(code) => error_response(code, None),
            // Error::InternalServerError => error_response(StatusCode::INTERNAL_SERVER_ERROR, None),
        },
    }
}

pub trait ResultExt: Sized {
    type Ok;

    fn status_code_error(self, code: StatusCode) -> Result<Self::Ok, Error>;

    fn server_error(self) -> Result<Self::Ok, Error> {
        self.status_code_error(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn client_error(self) -> Result<Self::Ok, Error> {
        self.status_code_error(StatusCode::BAD_REQUEST)
    }
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    fn status_code_error(self, code: StatusCode) -> Result<T, Error> {
        self.map_err(|e| {
            if code.is_server_error() {
                log::error!("catched: {:?} -> {}", e, code);
            }
            Error::StatusCode(code)
        })
    }
}

pub fn error_response(code: StatusCode, reason: Option<&str>) -> tide::Response {
    #[derive(Debug, serde::Serialize)]
    struct ResBody<'a> {
        code: u16,
        message: Option<&'a str>,
        reason: Option<&'a str>,
    }

    tide::Response::new(code.as_u16())
        .body_json(&ResBody {
            code: code.as_u16(),
            message: code.canonical_reason(),
            reason,
        })
        .unwrap()
}
