use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Serialize, Debug, Deserialize)]
pub enum BackendError {
    DatabaseError(String),
    DateError(String),
    UncaughtError(String),
}

// Helper function to convert `sqlx::Error` to `DbError`.
fn sql_error(err: Error) -> BackendError {
    BackendError::DatabaseError(err.to_string())
}

// Define a trait to add the custom method.
pub trait MapErrorExt<T> {
    fn map_error(self) -> Result<T, BackendError>;
}

// Implement the trait for `Result<T, sqlx::Error>`.
impl<T> MapErrorExt<T> for Result<T, sqlx::Error> {
    fn map_error(self) -> Result<T, BackendError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(sql_error(err)),
        }
    }
}
