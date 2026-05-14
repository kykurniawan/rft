use axum::{
    Router,
    routing::{get, post},
};

use crate::{core::state::AppState, domain::user};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(user::handler::index))
        .route("/api/users", post(user::handler::store))
        .route("/api/users/{id}", get(user::handler::show))
}
