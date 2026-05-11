use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub tracing: Tracing,
    pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(Environment::default().ignore_empty(true).separator("__"))
            .set_default("tracing.directory", "log")?
            .set_default("tracing.filename_suffix", "application.log")?
            .build()?;

        cfg.try_deserialize()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Tracing {
    pub directory: String,
    pub filename_suffix: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub connection_string: String,
}
