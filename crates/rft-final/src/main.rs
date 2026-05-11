mod app;
mod common;
mod domain;
mod infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::init().await?;

    app.run().await?;

    Ok(())
}
