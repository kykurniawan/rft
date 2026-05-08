use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::user::{UserState, error::UserError};

pub async fn index(State(state): State<Arc<UserState>>) -> impl IntoResponse {
    let result = state.user_service.get_users();

    match result {
        Ok(users) => (
            StatusCode::OK,
            Json(json!({ "users": users })).into_response(),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error.to_string() })).into_response(),
        ),
    }
}

pub async fn find(
    State(state): State<Arc<UserState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let id = Uuid::parse_str(id.as_str());
    let id = match id {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid id" })).into_response(),
            );
        }
    };

    let result = state.user_service.get_user_by_id(id);

    match result {
        Ok(user) => (
            StatusCode::OK,
            Json(json!({ "user": user })).into_response(),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error.to_string() })).into_response(),
        ),
    }
}

pub async fn create(
    State(state): State<Arc<UserState>>,
    Json(user): Json<serde_json::Value>,
) -> impl IntoResponse {
    let name = user["name"].as_str();
    let name = match name {
        Some(name) => name.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Missing name" })).into_response(),
            );
        }
    };

    let email = user["email"].as_str();
    let email = match email {
        Some(email) => email.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Missing email" })).into_response(),
            );
        }
    };

    let result = state.user_service.create_user(name, email);

    match result {
        Ok(user) => (
            StatusCode::OK,
            Json(json!({ "user": user })).into_response(),
        ),
        Err(error) => match error {
            UserError::EmailAlreadyExists => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": error.to_string() })).into_response(),
            ),
            UserError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": error.to_string() })).into_response(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": error.to_string() })).into_response(),
            ),
        },
    }
}
