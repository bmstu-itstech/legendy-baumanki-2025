use crate::domain::models::Places;
use crate::domain::models::SlotID;
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

    #[error("can not reserve slot {0:?} with {1:?} places")]
    CanNotReserveSlot(SlotID, Places),

    #[error("user {0:?} has not reserve slot {1:?}")]
    UserNotReservedSlot(UserID, SlotID),

    #[error("team {0:?} already reserved slot {1:?}")]
    TeamAlreadyReservedSlot(TeamID, SlotID),

    #[error("team {0:?} not reserved slot")]
    TeamNotReservedSlot(TeamID),
}
