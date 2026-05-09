use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Core domain entity representing a user.
// id is kept private with a getter because it is auto-generated and should
// never be set from outside. name and email are public for direct access
// (this is fine for a simple in-memory CRUD — larger apps would use setters/DTOs).
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User {
            id: Uuid::new_v4(),
            name,
            email,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
