use crate::common::config::Config;

mod app;
mod common;
mod domain;
mod infra;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let cfg = Config::new()?;

    let app = app::init(cfg).await?;

    app.run().await?;

    Ok(())
}
