use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, name: String, is_active: bool) -> Self {
        Self {
            id,
            name,
            is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
