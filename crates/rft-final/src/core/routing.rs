use axum::{Router, routing::get};

use crate::{core::state::AppState, domain::user};

pub fn router() -> Router<AppState> {
    Router::new().route("/api/users", get(user::handler::index))
}
