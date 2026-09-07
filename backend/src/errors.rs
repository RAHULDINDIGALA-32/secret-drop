use anyhow::Error as AnyError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::dto::response::ErrorResponse;


#[derive(thiserror::Error, Debug)]
pub enum SeceretDropError {
    #[error("Secret Not Found")]
    NotFound,

    #[error("Secret has expired")]
    Expired,

    #[error("Decryption failed")]
    DecryptionFailed,

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Internal Server Error")]
    InternalError(#[from] AnyError),
}

impl IntoResponse for SeceretDropError {
    fn into_response(self) -> Response {
        let error = self.to_string();

        let (status, details) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, None),
            Self::Expired => (StatusCode::GONE, None),
            Self::DecryptionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("CipherText could not be decrypted".to_owned()),
            ),
            Self::ValidationError(details) => (StatusCode::UNPROCESSABLE_ENTITY, Some(details)),
            Self::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS, 
                Some("Try again in a moment".to_owned()),
            ),
            Self::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
            Some("Unexpected server error".to_owned()),
            ),
        };

        let response_body = Json(ErrorResponse {
            error,
            details,
        });

        (status, response_body).into_response()
    }
}