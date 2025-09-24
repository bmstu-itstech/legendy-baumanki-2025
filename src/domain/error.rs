#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("team is full: {0}")]
    TeamIsFull(usize),

    #[error("user already in team: {0}")]
    UserAlreadyInTeam(i64),

    #[error("user {0} is not member of team")]
    UserIsNotMemberOfTeam(i64),
}
