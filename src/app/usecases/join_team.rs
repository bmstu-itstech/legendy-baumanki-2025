use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamRepository, UserRepository};
use crate::app::usecases::dto::TeamDTO;
use crate::domain::models::{TeamID, UserID};

#[derive(Clone)]
pub struct JoinTeam {
    user_repos: Arc<dyn UserRepository>,
    team_repos: Arc<dyn TeamRepository>,
}

impl JoinTeam {
    pub fn new(user_repos: Arc<dyn UserRepository>, team_repos: Arc<dyn TeamRepository>) -> Self {
        Self {
            user_repos,
            team_repos,
        }
    }

    pub async fn execute(&self, user_id: UserID, team_id: TeamID) -> Result<TeamDTO, AppError> {
        let mut team = self.team_repos.team(&team_id).await?;
        team.add_member(user_id)?;
        let mut user = self.user_repos.user(user_id).await?;
        user.switch_to_team_mode(team_id);
        self.user_repos.save_user(user).await?;
        self.team_repos.save_team(team.clone()).await?;
        Ok(team.into())
    }
}
