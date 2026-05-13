use crate::domain::{
    shared::db::{Paginated, Query},
    user::{User, UserRepository, UserServiceError},
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn find(&self, query: Query) -> Result<Paginated<User>, UserServiceError> {
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
                    _ => return Err(UserServiceError::Internal("invalid filter by".to_string())),
                }
            }
        }

        self.repository.find(query).await.map_err(Into::into)
    }
}
