mod core;
mod domain;
mod infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let cfg = core::config::Config::new()?;

    let app = core::app::init(cfg).await?;

    app.run().await?;

    Ok(())
}
