use crate::domain::error::DomainError;
use crate::domain::models::{CharacterName, MediaID, TaskID, TrackTag, UserID};

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error("user not found: {0}")]
    UserNotFound(i64),

    #[error("team not found: {0}")]
    TeamNotFound(String),

    #[error("media {0:?} not found")]
    MediaNotFound(MediaID),

    #[error("task {0:?} not found")]
    TaskNotFound(TaskID),

    #[error("character {0:?} not found")]
    CharacterNotFound(CharacterName),

    #[error("track {0:?} not found")]
    TrackNotFound(TrackTag),

    #[error("user {0:?} is not in team")]
    UserNotInTeam(UserID),

    #[error(transparent)]
    Internal(#[from] StdError),
}
