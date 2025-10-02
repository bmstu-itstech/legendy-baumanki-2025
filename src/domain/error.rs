use crate::domain::models::{TeamID, TrackTag, UserID};

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("team is full: {0}")]
    TeamIsFull(usize),

    #[error("user {0:?} already in team '{1:?}'")]
    UserAlreadyInTeam(UserID, TeamID),

    #[error("user {0:?} is not member of team")]
    UserIsNotMemberOfTeam(UserID),

    #[error("track {0:?} can not be started")]
    TrackCanNotBeStarted(TrackTag),

    #[error("track {0:?} can not be finished")]
    TrackCanNotBeFinished(TrackTag),

    #[error("track {0:?} not started")]
    TrackNotStarted(TrackTag),
}
