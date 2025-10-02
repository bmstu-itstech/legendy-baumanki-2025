use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamProvider, UserProvider};
use crate::app::usecases::dto::{TeamWithMembersDTO, UserDTO};
use crate::domain::models::{MAX_TEAM_SIZE, TeamID};

#[derive(Clone)]
pub struct GetTeamWithMembers {
    team_provider: Arc<dyn TeamProvider>,
    user_provider: Arc<dyn UserProvider>,
}

impl GetTeamWithMembers {
    pub fn new(team_provider: Arc<dyn TeamProvider>, user_provider: Arc<dyn UserProvider>) -> Self {
        Self {
            team_provider,
            user_provider,
        }
    }

    pub async fn execute(&self, team_id: TeamID) -> Result<TeamWithMembersDTO, AppError> {
        let team = self.team_provider.team(&team_id).await?;
        let mut members: Vec<UserDTO> = Vec::new();
        for member_id in team.member_ids() {
            let member = self.user_provider.user(*member_id).await?;
            members.push(member.into());
        }
        Ok(TeamWithMembersDTO {
            id: team.id().clone(),
            solo: team.is_solo(),
            name: team.name().clone(),
            size: members.len(),
            max_size: MAX_TEAM_SIZE,
            members,
        })
    }
}
