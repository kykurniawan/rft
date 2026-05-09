use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::user::{entity::User, error::UserError};

// In-memory user storage backed by a Vec<Mutex<T>>.
// Mutex is required because Axum serves requests on a multi-threaded runtime,
// and multiple handlers may read/write the shared user list concurrently.
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db: db }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, UserError> {
        let users = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.db)
            .await
            .map_err(|_| UserError::InternalError)?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1 LIMIT 1"#)
            .bind(id)
            .fetch_one(&self.db)
            .await;

        let user = match user {
            Ok(user) => Some(user),
            Err(sqlx::Error::RowNotFound) => None,
            Err(_) => return Err(UserError::InternalError),
        };

        Ok(user)
    }

    // Accepts &str to avoid requiring the caller to transfer ownership.
    // The comparison still works because String implements PartialEq<&str>.
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE email = $1 LIMIT 1"#)
            .bind(email)
            .fetch_one(&self.db)
            .await;

        let user = match user {
            Ok(user) => Some(user),
            Err(sqlx::Error::RowNotFound) => None,
            Err(_) => return Err(UserError::InternalError),
        };

        Ok(user)
    }

    pub async fn save(&self, user: User) -> Result<(), UserError> {
        let result =
            sqlx::query(r#"INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING id"#)
                .bind(user.id())
                .bind(user.name)
                .bind(user.email)
                .fetch_one(&self.db)
                .await;

        match result {
            Ok(_) => Ok(()),
            Err(er) => {
                println!("{:?}", er);
                Err(UserError::InternalError)
            }
        }
    }

    pub async fn update(&self, user: User) -> Result<(), UserError> {
        let id = user.id();
        let name = user.name;
        let email = user.email;
        let result =
            sqlx::query(r#"UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id"#)
                .bind(name)
                .bind(email)
                .bind(id)
                .fetch_one(&self.db)
                .await;

        match result {
            Ok(_) => Ok(()),
            Err(er) => {
                println!("{:?}", er);
                Err(UserError::InternalError)
            }
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        let result = sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(&self.db)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(er) => {
                println!("{:?}", er);
                Err(UserError::InternalError)
            }
        }
    }
}
