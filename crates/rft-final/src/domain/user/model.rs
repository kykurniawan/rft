use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
