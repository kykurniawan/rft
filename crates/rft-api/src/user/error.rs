use axum::{Json, http::StatusCode};
use serde_json::json;
use thiserror::Error;

// Centralizes all user-domain errors with their HTTP status code mappings.
// Each handler simply calls error.to_response() instead of manually matching variants.
#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Unexpected internal error")]
    InternalError,
}

impl UserError {
    // Converts the error into an HTTP response tuple, so every handler
    // branches with the same concrete return type: (StatusCode, Json<Value>).
    pub fn to_response(&self) -> (StatusCode, Json<serde_json::Value>) {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::EmailAlreadyExists => StatusCode::BAD_REQUEST,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.to_string() })))
    }
}
