use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamRepository, UserRepository};
use crate::app::usecases::dto::TeamDTO;
use crate::domain::models::{Team, TeamName, UserID};

#[derive(Clone)]
pub struct CreateTeam {
    team_repos: Arc<dyn TeamRepository>,
    user_repos: Arc<dyn UserRepository>,
}

impl CreateTeam {
    pub fn new(team_repos: Arc<dyn TeamRepository>, user_repos: Arc<dyn UserRepository>) -> Self {
        Self {
            team_repos,
            user_repos,
        }
    }

    pub async fn execute(&self, name: TeamName, captain_id: UserID) -> Result<TeamDTO, AppError> {
        let team = Team::new(name, captain_id);
        let dto = team.clone().into();
        let mut user = self.user_repos.user(captain_id).await?;
        user.join_team(team.id().clone())?;
        self.team_repos.save_team(team).await?;
        self.user_repos.save_user(user).await?;
        Ok(dto)
    }
}
