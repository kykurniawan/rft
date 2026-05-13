use axum::{Router, routing::get};

use crate::core::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
