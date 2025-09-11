use crate::domain::error::DomainError;

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    DomainError(DomainError),

    #[error(transparent)]
    Internal(#[from] StdError),
}
