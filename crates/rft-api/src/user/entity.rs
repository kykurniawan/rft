use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
