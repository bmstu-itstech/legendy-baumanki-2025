use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamRepository;
use crate::app::usecases::dto::TeamDTO;
use crate::domain::models::{TeamID, UserID};

#[derive(Clone)]
pub struct JoinTeam {
    repos: Arc<dyn TeamRepository>,
}

impl JoinTeam {
    pub fn new(repos: Arc<dyn TeamRepository>) -> Self {
        Self { repos }
    }

    pub async fn join_team(&self, user_id: UserID, team_id: TeamID) -> Result<TeamDTO, AppError> {
        let mut team = self.repos.team(team_id).await?;
        team.add_member(user_id)?;
        self.repos.save_team(team.clone()).await?;
        Ok(team.into())
    }
}
