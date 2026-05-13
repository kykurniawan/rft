use std::sync::Arc;

use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    core::{config::Config, state::AppState},
    domain::{self},
    infra,
};

pub async fn init(config: Config) -> Result<App, Box<dyn std::error::Error>> {
    let _tracing = infra::tracing::init(&config.tracing);

    let db = infra::db::connect(&config.database).await?;

    let user_repository = domain::user::UserRepository::new(db.clone());

    let user_service = domain::user::UserService::new(user_repository.clone());

    let state = AppState {
        user_service: Arc::new(user_service),
    };

    Ok(App { config, state })
}

pub struct App {
    config: Config,
    state: AppState,
}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("preparing http server");

        let router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .with_state(self.state.clone());

        let address = format!("{}:{}", self.config.server.host, self.config.server.port);

        let listener = TcpListener::bind(address).await?;

        info!("http server is running on {}", listener.local_addr()?);

        serve(listener, router).await?;

        Ok(())
    }
}
