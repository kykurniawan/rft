use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("not found error")]
    NotFound,

    #[error("conflict error")]
    Conflict(String),

    #[error("database error: {0}")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,

            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::Conflict(db_err.message().to_string())
            }

            _ => RepositoryError::Database(e),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found error: {0:?}")]
    NotFound(String),

    #[error("conflict error: {0:?}")]
    Conflict(String),

    #[error("internal error: {0:?}")]
    Internal(String),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error(transparent)]
    PathExtractorError(#[from] PathRejection),

    #[error(transparent)]
    JsonExtractorError(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error) = match &self {
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::Conflict(e) => (StatusCode::CONFLICT, e.to_string()),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::PathExtractorError(e) => (e.status(), e.body_text()),
            AppError::JsonExtractorError(e) => (e.status(), e.body_text()),
        };

        let body = json!({ "error": error });

        (status, Json(body)).into_response()
    }
}
