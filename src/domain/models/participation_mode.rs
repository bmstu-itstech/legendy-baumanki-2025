use crate::domain::models::TeamID;

#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantStatus {
    Solo,
    LookingForTeam,
    Team(TeamID),
}
