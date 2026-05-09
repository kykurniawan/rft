use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::user::{UserState, repository::UserRepository, service::UserService};

mod database;
mod user;

// Root application state. Holds shared dependencies for all routes.
// FromRef lets Axum extract a sub-state (Arc<UserState>) from AppState
// without the handler knowing about other parts of the application.
#[derive(Clone)]
pub struct AppState {
    pub user_state: Arc<UserState>,
}

impl axum::extract::FromRef<AppState> for Arc<UserState> {
    fn from_ref(state: &AppState) -> Self {
        state.user_state.clone()
    }
}

// Composition root — wires dependencies manually: Pool → Repository → Service → State.
// No DI framework needed for a crate this size.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env into the process environment so std::env::var() can find DATABASE_URL.
    // The `_` return value is a Result — we ignore failure because .env is optional
    // (e.g. in production env vars are set directly by the container/deploy system).
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/rft_database".to_string());

    let db = database::create_connection(&database_url).await?;

    let user_repository = UserRepository::new(db);
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
