use warp::http::StatusCode;

#[derive(thiserror::Error, Debug)]
#[error("Error {code}")]
pub struct Error {
    code: StatusCode,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    pub fn not_found() -> Self {
        StatusCode::NOT_FOUND.into()
    }

    pub fn server_error() -> Self {
        StatusCode::INTERNAL_SERVER_ERROR.into()
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl warp::reject::Reject for Error {}

impl From<StatusCode> for Error {
    fn from(code: StatusCode) -> Self {
        Error { code, source: None }
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
    E: std::error::Error + Send + Sync + 'static,
{
    type Ok = T;

    fn status_code_error(self, code: StatusCode) -> Result<T, Error> {
        self.map_err(|error| Error {
            code,
            source: Some(Box::new(error)),
        })
    }
}
