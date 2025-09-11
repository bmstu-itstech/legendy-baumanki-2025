#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("team is full: {0}")]
    TeamIsFull(usize),

    #[error("user not found: {0}")]
    UserNotFound(i64),

    #[error("user already in team: {0}")]
    UserAlreadyInTeam(i64),

    #[error("team not found: {0}")]
    TeamNotFound(String),

    #[error("user {0} is not member of team")]
    UserIsNotMemberOfTeam(i64),

    #[error("not allowed: {0}")]
    NotAllowed(String),
}
