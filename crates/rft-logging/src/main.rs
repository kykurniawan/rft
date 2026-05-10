use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
    serve,
};
use serde::{Deserialize, Serialize};
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
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Clone, Serialize, Deserialize)]
struct Company {
    id: u32,
    name: String,
}

#[derive(Clone)]
struct AppState {
    companies: Arc<Vec<Company>>,
}

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    let state = init_application_state();

    let service_builder = create_service_builder();

    let app = Router::new()
        .route("/", get(index))
        .route("/{id}", get(show))
        .with_state(state)
        .layer(service_builder);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port");

    info!("server listening on http://0.0.0.0:3000");

    serve(listener, app).await.expect("failed to serve");
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::never("log", "application.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let stdout_layer = fmt::layer()
        .with_target(false)
        .with_ansi(false)
        .with_thread_ids(false);

    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    guard
}

fn init_application_state() -> AppState {
    AppState {
        companies: Arc::new(vec![
            Company {
                id: 1,
                name: String::from("Apple"),
            },
            Company {
                id: 2,
                name: String::from("Google"),
            },
            Company {
                id: 3,
                name: String::from("Microsoft"),
            },
        ]),
    }
}

fn create_service_builder()
-> ServiceBuilder<Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, Identity>> {
    ServiceBuilder::new().layer(TraceLayer::new_for_http())
}

async fn index(State(state): State<AppState>) -> Json<Vec<Company>> {
    let companies = state.companies.clone().to_vec();

    Json(companies)
}

async fn show(State(state): State<AppState>, Path(id): Path<u32>) -> Json<Option<Company>> {
    let companies = state.companies.clone();

    let company = companies.iter().find(|c| c.id == id).cloned();

    if company.is_none() {
        return Json(None);
    }

    Json(company)
}
