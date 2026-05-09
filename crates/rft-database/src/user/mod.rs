// User module — groups all user-related logic: entity, repository, service, handlers.
// UserState wraps UserService in an Arc so a single shared instance can be
// extracted by every handler via Axum's State extractor (multi-thread safe).

use std::sync::Arc;

use axum::{
    Router,
    extract::FromRef,
    routing::{delete, get, post, put},
};

use crate::user::service::UserService;

pub mod entity;
pub mod error;
pub mod handler;
pub mod repository;
pub mod service;

#[derive(Clone)]
pub struct UserState {
    pub user_service: Arc<UserService>,
}

impl UserState {
    pub fn new(user_service: UserService) -> Self {
        Self {
            user_service: Arc::new(user_service),
        }
    }
}

pub fn router<S>() -> Router<S>
where
    Arc<UserState>: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(handler::index))
        .route("/{id}", get(handler::find))
        .route("/{id}", put(handler::update))
        .route("/{id}", delete(handler::delete))
        .route("/create", post(handler::create))
}
