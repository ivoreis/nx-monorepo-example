use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("Action type not supported: {0}")]
    ActionTypeNotSupported(String),

    #[error("Invalid state transitions {0}")]
    InvalidStateTransition(String),

    #[error("Invalid version {0}")]
    InvalidVersion(i32),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}
