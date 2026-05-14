use crate::domain::shared::error::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("user not found")]
    UserNotFound,

    #[error("name already exists")]
    NameAlreadyExists,

    #[error("user is inactive")]
    UserInactive,

    #[error("invalid filter by: {0}")]
    InvalidFilter(String),

    #[error("internal error")]
    Internal(String),
}

impl From<RepositoryError> for UserServiceError {
    fn from(e: RepositoryError) -> Self {
        match &e {
            RepositoryError::NotFound => UserServiceError::UserNotFound,
            RepositoryError::Conflict(msg) => UserServiceError::Internal(msg.clone()),
            RepositoryError::Database(db_err) => UserServiceError::Internal(format!("{db_err:?}")),
        }
    }
}
