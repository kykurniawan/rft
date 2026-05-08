use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Unexpected internal error")]
    InternalError,
}
