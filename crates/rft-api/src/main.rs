use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::user::{UserState, repository::UserRepository, service::UserService};

mod user;

#[derive(Clone)]
pub struct AppState {
    pub user_state: Arc<UserState>,
}

impl axum::extract::FromRef<AppState> for Arc<UserState> {
    fn from_ref(state: &AppState) -> Self {
        state.user_state.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_repository = UserRepository::new();
    let user_service = UserService::new(user_repository);
    let user_state = UserState::new(user_service);

    let app_state = AppState {
        user_state: Arc::new(user_state),
    };

    let app = Router::new()
        .nest("/users", user::router())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
