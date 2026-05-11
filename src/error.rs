use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unsupported language: {0}")]
    UnsupportedLanguage(String),
    #[error("sandbox error: {0}")]
    Sandbox(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::Config(_) | Self::Sandbox(_) | Self::Io(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
            }
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            Self::UnsupportedLanguage(_) => (StatusCode::BAD_REQUEST, "unsupported_language"),
        };

        let body = Json(ErrorResponse {
            error: ErrorBody {
                code,
                message: self.to_string(),
            },
        });

        (status, body).into_response()
    }
}
