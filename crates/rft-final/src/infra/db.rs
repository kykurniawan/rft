use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{error, info};

use crate::core::config::Database;

pub async fn connect(config: &Database) -> Result<Pool<Postgres>, sqlx::Error> {
    info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.connection_string.as_str())
        .await;

    match pool {
        Ok(pool) => {
            info!("database connection established");
            Ok(pool)
        }
        Err(err) => {
            error!("database connection failed: {}", err);
            Err(err)
        }
    }
}
