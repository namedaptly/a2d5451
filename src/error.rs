use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Failed to parse request: {0}")]
    FailedToParseRequest(String),
    #[error("Movie {0} not found")]
    MovieNotFound(String),

    // catch all error for developing new endpoints
    #[error("Unknown error")]
    #[allow(dead_code)]
    UnknownError,
}
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        ErrorResponse {
            error: value.to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::FailedToParseRequest(_) => StatusCode::BAD_REQUEST,
            Error::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MovieNotFound(_) => StatusCode::NOT_FOUND,
        };
        (status, Json(ErrorResponse::from(self))).into_response()
    }
}
