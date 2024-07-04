use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("csv parsing error: `{0}`")]
    Multipart(#[from] MultipartError),

    #[error("content type error")]
    ContentType,

    #[error("multipart field error")]
    ZeroFieldsProvided,

    #[error("task join error: `{0}`")]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("database error: `{0}`")]
    Sqlx(#[from] sqlx::Error),

    #[error("csv error: `{0}`")]
    Csv(#[from]csv_async::Error)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error_handler = match self {
            ApiError::Multipart(err) => (err.status(), err.body_text()),
            ApiError::ContentType => (StatusCode::BAD_REQUEST, "Use a csv file".to_string()),
            ApiError::ZeroFieldsProvided => (
                StatusCode::BAD_REQUEST,
                "Don't forget to include a csv file".to_string(),
            ),
            ApiError::TaskJoin(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            ApiError::Sqlx(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            ApiError::Csv(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        };

        error_handler.into_response()
    }
}
