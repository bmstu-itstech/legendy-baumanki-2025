use std::fmt::{Debug, Display};
use teloxide::RequestError;
use teloxide::dispatching::dialogue::{InMemStorageError, PostgresStorageError};

use crate::app::error::AppError;

pub mod dispatcher;
mod fsm;
mod handlers;
mod keyboards;
mod resources;
mod texts;

pub type BotHandlerResult = Result<(), AppError>;

impl From<RequestError> for AppError {
    fn from(value: RequestError) -> Self {
        Self::Internal(value.into())
    }
}

impl From<InMemStorageError> for AppError {
    fn from(value: InMemStorageError) -> Self {
        Self::Internal(value.into())
    }
}

impl<SE: Display + Debug + Send + Sync + 'static> From<PostgresStorageError<SE>> for AppError {
    fn from(value: PostgresStorageError<SE>) -> Self {
        Self::Internal(value.into())
    }
}
