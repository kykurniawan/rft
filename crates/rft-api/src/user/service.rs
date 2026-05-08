use uuid::Uuid;

use crate::user::{entity::User, error::UserError, repository::UserRepository};

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
        let user = self.user_repository.find_by_id(id)?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound),
        }
    }

    pub fn create_user(&self, name: String, email: String) -> Result<User, UserError> {
        let user = User::new(name, email);

        let existsing_user = self.user_repository.find_by_email(user.email.clone())?;

        match existsing_user {
            Some(_) => Err(UserError::EmailAlreadyExists),
            None => {
                self.user_repository.save(user.clone())?;

                Ok(user)
            }
        }
    }
}
