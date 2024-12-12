use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}
