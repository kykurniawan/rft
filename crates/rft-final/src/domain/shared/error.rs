use axum::{Json, http::StatusCode, response::IntoResponse};
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

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self),
            AppError::Conflict(_) => (StatusCode::CONFLICT, self),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, self),
            AppError::Internal(_) => {
                tracing::error!(?self, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, self)
            }
        };

        let body = json!({ "error": error_message.to_string() });
        (status, Json(body)).into_response()
    }
}
