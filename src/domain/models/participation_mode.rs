use crate::domain::models::TeamID;

#[derive(Debug, Clone, PartialEq)]
pub enum ParticipationMode {
    Solo,
    WantTeam,
    Team(TeamID),
}
