use crate::{common::config, domain, infra};

pub struct App {}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub async fn init() -> Result<App, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let cfg = config::Config::new()?;

    let _tracing = infra::tracing::init(&cfg.tracing);

    let db = infra::db::connect(&cfg.database).await?;

    let _user_repository = domain::user::UserRepository::new(db);

    Ok(App {})
}
