use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::user::UserState;

// Typed request body for POST /users/create.
// Using a concrete struct with #[derive(Deserialize)] instead of serde_json::Value
// gives compile-time field validation — a missing field produces a clear 422 error
// from Axum itself, no manual check needed.
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
}

pub async fn index(State(state): State<Arc<UserState>>) -> impl IntoResponse {
    let result = state.user_service.get_users().await;

    match result {
        Ok(users) => (StatusCode::OK, Json(json!({ "users": users }))).into_response(),
        Err(error) => error.to_response().into_response(),
    }
}

// Axum's Path<Uuid> parses the {id} segment into a Uuid automatically.
// If parsing fails (malformed UUID), Axum rejects the request with a 404.
pub async fn find(State(state): State<Arc<UserState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = state.user_service.get_user_by_id(id).await;

    match result {
        Ok(user) => (StatusCode::OK, Json(json!({ "user": user }))).into_response(),
        Err(error) => error.to_response().into_response(),
    }
}

pub async fn create(
    State(state): State<Arc<UserState>>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let result = state.user_service.create_user(req.name, req.email).await;

    match result {
        Ok(user) => (StatusCode::OK, Json(json!({ "user": user }))).into_response(),
        Err(error) => error.to_response().into_response(),
    }
}

pub async fn update(
    State(state): State<Arc<UserState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let result = state
        .user_service
        .update_user(id, req.name, req.email)
        .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(json!({ "user": user }))).into_response(),
        Err(error) => error.to_response().into_response(),
    }
}

pub async fn delete(
    State(state): State<Arc<UserState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = state.user_service.delete_user(id).await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "User deleted" }))).into_response(),
        Err(error) => error.to_response().into_response(),
    }
}
