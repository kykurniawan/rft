use std::sync::Arc;

use axum::serve;
use tokio::net::TcpListener;
use tower::{
    ServiceBuilder,
    layer::util::{Identity, Stack},
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    core::{config::Config, routing, state::AppState},
    domain::{self},
    infra,
};

pub async fn init(config: Config) -> Result<App, Box<dyn std::error::Error>> {
    let _tracing = infra::tracing::init(&config.tracing);

    info!("initializing application");

    let db = infra::db::connect(&config.database).await?;

    let user_repository = domain::user::UserRepository::new(db.clone());

    let user_service = domain::user::UserService::new(user_repository.clone());

    let state = AppState {
        user_service: Arc::new(user_service),
    };

    info!("application initialized");

    Ok(App { config, state })
}

pub struct App {
    config: Config,
    state: AppState,
}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("preparing http server");

        let router = routing::router()
            .with_state(self.state.clone())
            .layer(self.create_service_builder());

        let address = format!("{}:{}", self.config.server.host, self.config.server.port);

        let listener = TcpListener::bind(address).await?;

        info!("http server is running on {}", listener.local_addr()?);

        serve(listener, router).await?;

        Ok(())
    }

    fn create_service_builder(
        &self,
    ) -> ServiceBuilder<Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, Identity>> {
        ServiceBuilder::new().layer(TraceLayer::new_for_http())
    }
}
