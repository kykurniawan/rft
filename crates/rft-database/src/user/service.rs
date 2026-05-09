use uuid::Uuid;

use crate::user::{entity::User, error::UserError, repository::UserRepository};

// Business-logic layer between handlers and the repository.
// Orchestrates multi-step operations (e.g. uniqueness checks) that go beyond
// simple CRUD, keeping the repository focused purely on data access.
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.user_repository.find_all().await
    }

    // ok_or converts Option::None into UserError::NotFound — the repository
    // returns Option (data-layer concern), the service translates it to a
    // domain error the handler can convert to an HTTP response.
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, UserError> {
        self.user_repository
            .find_by_id(id)
            .await?
            .ok_or(UserError::NotFound)
    }

    // Guard clause bails out early if email already exists, keeping the
    // happy path (construct → save → return) unindented.
    pub async fn create_user(&self, name: String, email: String) -> Result<User, UserError> {
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        let user = User::new(name, email);
        self.user_repository.save(user.clone()).await?;
        Ok(user)
    }

    // Repository's update() already checks rows_affected and returns NotFound
    // if the id doesn't match. No separate existence check needed here.
    pub async fn update_user(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<User, UserError> {
        self.user_repository.update(id, &name, &email).await?;
        Ok(User::with_id(id, name, email))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        self.user_repository.delete(id).await?;
        Ok(())
    }
}
