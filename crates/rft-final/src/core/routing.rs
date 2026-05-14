use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{core::state::AppState, domain::user};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(user::handler::index))
        .route("/api/users", post(user::handler::store))
        .route("/api/users/{id}", get(user::handler::show))
        .route("/api/users/{id}", patch(user::handler::update))
        .route("/api/users/{id}", delete(user::handler::delete))
}
