use crate::app::error::AppError;
use teloxide::RequestError;
use teloxide::dispatching::dialogue::InMemStorageError;

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
