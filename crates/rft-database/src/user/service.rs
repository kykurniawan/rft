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

    pub async fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.user_repository.find_all().await
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, UserError> {
        let user = self.user_repository.find_by_id(id).await;

        let user = match user {
            Ok(user) => match user {
                Some(user) => user,
                None => return Err(UserError::NotFound),
            },
            Err(_) => return Err(UserError::InternalError),
        };

        Ok(user)
    }

    // Uses early return (guard clause) to bail out early if email already exists,
    // avoiding unnecessary User construction and keeping the happy path unindented.
    pub async fn create_user(&self, name: String, email: String) -> Result<User, UserError> {
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        let user = User::new(name, email);

        self.user_repository.save(user.clone()).await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<User, UserError> {
        let user = self.user_repository.find_by_id(id).await?;

        let mut user = match user {
            Some(user) => user,
            None => return Err(UserError::NotFound),
        };

        user.name = name;
        user.email = email;

        self.user_repository.update(user.clone()).await?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        let user = self.user_repository.find_by_id(id).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(UserError::NotFound),
        };

        println!("User ID: {:?}", user.id());

        self.user_repository.delete(user.id()).await?;
        Ok(())
    }
}
