use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::user::{entity::User, error::UserError};

// Database-backed user repository. Holds a reference-counted connection pool
// (Pool<Postgres> is internally Arc-backed — cheap to clone, thread-safe).
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, UserError> {
        // query_as::<_, User> tells sqlx to deserialize each row into a User.
        // The `_` lets Rust infer the database type (Postgres) automatically.
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.db)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        // fetch_optional returns Ok(None) when no row matches — no RowNotFound
        // error to catch, which makes the calling code cleaner.
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn save(&self, user: User) -> Result<(), UserError> {
        // execute() runs a statement that doesn't return rows. No need for
        // RETURNING here since the caller already has the full User object.
        sqlx::query("INSERT INTO users (id, name, email) VALUES ($1, $2, $3)")
            .bind(user.id())
            .bind(user.name)
            .bind(user.email)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    // Takes individual fields instead of a User struct to avoid the caller
    // constructing a partial entity just for the update payload.
    pub async fn update(&self, id: Uuid, name: &str, email: &str) -> Result<(), UserError> {
        let rows_affected = sqlx::query("UPDATE users SET name = $1, email = $2 WHERE id = $3")
            .bind(name)
            .bind(email)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected();

        // rows_affected == 0 means the WHERE clause matched nothing —
        // the caller passed an id that doesn't exist.
        if rows_affected == 0 {
            return Err(UserError::NotFound);
        }

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        let rows_affected = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(UserError::NotFound);
        }

        Ok(())
    }
}
