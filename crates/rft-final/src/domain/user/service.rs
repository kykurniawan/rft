use uuid::Uuid;

use crate::domain::{
    shared::{
        db::{Paginated, Query},
        error::RepositoryError,
    },
    user::{User, UserRepository, UserServiceError},
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn get_users(&self, query: Query) -> Result<Paginated<User>, UserServiceError> {
        if let Some(sorts) = &query.sorts {
            for sort in sorts {
                match sort.by.as_str() {
                    "name" | "created_at" => {}
                    _ => return Err(UserServiceError::Internal("invalid sort by".to_string())),
                }
            }
        }

        if let Some(filters) = &query.filters {
            for filter in filters {
                match filter.by.as_str() {
                    "is_active" => {}
                    _ => return Err(UserServiceError::InvalidFilter(filter.by.clone())),
                }
            }
        }

        self.repository.find(query).await.map_err(Into::into)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, UserServiceError> {
        let user = self.repository.find_by_id(id).await;

        let user = match user {
            Ok(user) => user,
            Err(error) => match error {
                RepositoryError::NotFound => return Ok(None),
                _ => return Err(UserServiceError::Internal(error.to_string())),
            },
        };

        Ok(user)
    }

    pub async fn create_user(&self, name: &str) -> Result<User, UserServiceError> {
        let existing = self.repository.find_by_name(name).await;

        match existing {
            Ok(Some(_)) => return Err(UserServiceError::NameAlreadyExists),
            Ok(None) => {}
            Err(e) => return Err(e.into()),
        };

        let user = User::new(Uuid::now_v7(), name.to_string(), true);

        self.repository.save(&user).await.map_err(Into::into)
    }
}
