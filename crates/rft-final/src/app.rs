use std::sync::Arc;

use axum::{Router, routing::get, serve};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    common::config::{self, Config},
    domain::{
        self,
        shared::db::{Filter, FilterValue, Pagination, Query, Sort, SortOrder},
    },
    infra,
    state::AppState,
};

pub struct App {
    state: AppState,
}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("preparing http server");

        let router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .with_state(self.state.clone());

        let listener = TcpListener::bind("0.0.0.0:3000").await?;

        info!("http server is running on {}", listener.local_addr()?);

        serve(listener, router).await?;

        Ok(())
    }
}

pub async fn init(cfg: Config) -> Result<App, Box<dyn std::error::Error>> {
    let _tracing = infra::tracing::init(&cfg.tracing);

    let db = infra::db::connect(&cfg.database).await?;

    let user_repository = domain::user::UserRepository::new(db.clone());

    let user_service = domain::user::UserService::new(user_repository.clone());

    let state = AppState {
        user_service: Arc::new(user_service),
    };

    Ok(App { state })
}
