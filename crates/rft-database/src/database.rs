use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_connection(connection_string: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await?;

    Ok(pool)
}
