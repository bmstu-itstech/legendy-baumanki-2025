use crate::domain::error::DomainError;
use crate::domain::models::{MediaID, TaskID};

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    DomainError(DomainError),

    #[error("media {0:?} not found")]
    MediaNotFound(MediaID),

    #[error("task {0:?} not found")]
    TaskNotFound(TaskID),

    #[error(transparent)]
    Internal(#[from] StdError),
}
