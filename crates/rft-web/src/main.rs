mod handler;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(handler::common::index))
        .route("/fruits", get(handler::common::fruits));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
