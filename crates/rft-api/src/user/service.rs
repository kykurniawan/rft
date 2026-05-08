use uuid::Uuid;

use crate::user::{entity::User, error::UserError, repository::UserRepository};

// Thin business-logic layer between handlers and the repository.
// In a real app this would hold validation rules, cross-cutting concerns, etc.
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.user_repository.find_all()
    }

    pub fn get_user_by_id(&self, id: Uuid) -> Result<User, UserError> {
        self.user_repository
            .find_by_id(id)?
            .ok_or(UserError::NotFound)
    }

    // Uses early return (guard clause) to bail out early if email already exists,
    // avoiding unnecessary User construction and keeping the happy path unindented.
    pub fn create_user(&self, name: String, email: String) -> Result<User, UserError> {
        if self.user_repository.find_by_email(&email)?.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        let user = User::new(name, email);
        self.user_repository.save(user.clone())?;
        Ok(user)
    }
}
