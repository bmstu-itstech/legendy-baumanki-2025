use crate::domain::models::{ParticipantStatus, TeamID, UserID};

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("team is full: {0}")]
    TeamIsFull(usize),

    #[error("user {0:?} already in team '{1:?}'")]
    UserAlreadyInTeam(UserID, TeamID),

    #[error("user {0:?} can't switch to status {1:?}")]
    UserCannotSwitchToStatus(UserID, ParticipantStatus),

    #[error("user {0:?} is not member of team")]
    UserIsNotMemberOfTeam(UserID),
}
