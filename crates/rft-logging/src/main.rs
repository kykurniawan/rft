// mod simple_logger;
use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get, serve};
use serde::Serialize;
use tokio::net::TcpListener;
use tower::{
    ServiceBuilder,
    layer::util::{Identity, Stack},
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};

#[derive(Clone, Serialize)]
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
    log4rs::init_file("log4rs.yaml", Default::default()).expect("failed to initialize logging");

    let state = init_application_state();

    let service_builder = create_service_builder();

    let app = Router::new()
        .route("/", get(index))
        .with_state(state)
        .layer(service_builder);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port");

    log::info!("server listening on http://0.0.0.0:3000");

    serve(listener, app).await.expect("failed to serve");
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
