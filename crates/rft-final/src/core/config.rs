use config::{ConfigBuilder, ConfigError, Environment, builder::DefaultState};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Tracing {
    // TRACING__DIRECTORY
    pub directory: String,

    // TRACING__FILENAME_SUFFIX
    pub filename_suffix: String,

    // TRACING__ENV_FILTER
    pub env_filter: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    // DATABASE__CONNECTION_STRING
    pub connection_string: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    // SERVER__HOST
    pub host: String,

    // SERVER__PORT
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub tracing: Tracing,
    pub database: Database,
    pub server: Server,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = Self::with_defaults(
            config::Config::builder()
                .add_source(Environment::default().ignore_empty(true).separator("__")),
        )?;

        cfg.build()?.try_deserialize()
    }

    fn with_defaults(
        mut cfg: ConfigBuilder<DefaultState>,
    ) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        cfg = cfg
            .set_default("tracing.directory", "logs")?
            .set_default("tracing.filename_suffix", "application.log")?
            .set_default("tracing.env_filter", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?;

        Ok(cfg)
    }
}
