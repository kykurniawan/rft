use std::sync::Arc;

use axum::serve;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    core,
    domain::{self},
    infra,
};

pub async fn init(config: core::config::Config) -> Result<App, Box<dyn std::error::Error>> {
    let tracing = infra::tracing::init(&config.tracing);

    info!("initializing application");

    let db = infra::db::connect(&config.database).await?;

    let user_repository = domain::user::UserRepository::new(db.clone());

    let user_service = domain::user::UserService::new(user_repository.clone());

    let state = core::state::AppState {
        user_service: Arc::new(user_service),
    };

    info!("application initialized");

    Ok(App {
        tracing,
        config,
        state,
    })
}

pub struct App {
    #[allow(unused)]
    tracing: WorkerGuard,
    config: core::config::Config,
    state: core::state::AppState,
}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("preparing http server");

        let router = core::routing::router()
            .with_state(self.state.clone())
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

        let address = format!("{}:{}", self.config.server.host, self.config.server.port);

        let listener = TcpListener::bind(address).await?;

        info!("http server is running on {}", listener.local_addr()?);

        serve(listener, router).await?;

        Ok(())
    }
}
