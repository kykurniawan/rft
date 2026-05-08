use std::sync::Mutex;

use uuid::Uuid;

use crate::user::{entity::User, error::UserError};

// In-memory user storage backed by a Vec<Mutex<T>>.
// Mutex is required because Axum serves requests on a multi-threaded runtime,
// and multiple handlers may read/write the shared user list concurrently.
pub struct UserRepository {
    users: Mutex<Vec<User>>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }

    pub fn find_all(&self) -> Result<Vec<User>, UserError> {
        let users = self.users.lock().map_err(|_| UserError::InternalError)?;
        Ok(users.clone())
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        let users = self.users.lock().map_err(|_| UserError::InternalError)?;
        Ok(users.iter().find(|user| user.id() == id).cloned())
    }

    // Accepts &str to avoid requiring the caller to transfer ownership.
    // The comparison still works because String implements PartialEq<&str>.
    pub fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let users = self.users.lock().map_err(|_| UserError::InternalError)?;
        Ok(users.iter().find(|user| user.email == email).cloned())
    }

    pub fn save(&self, user: User) -> Result<(), UserError> {
        let mut users = self.users.lock().map_err(|_| UserError::InternalError)?;
        users.push(user);
        Ok(())
    }
}
