use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamRepository;
use crate::app::usecases::dto::TeamDTO;
use crate::domain::models::{Team, TeamName, UserID};

#[derive(Clone)]
pub struct CreateTeam {
    repos: Arc<dyn TeamRepository>,
}

impl CreateTeam {
    pub fn new(repos: Arc<dyn TeamRepository>) -> Self {
        Self { repos }
    }

    pub async fn create_team(
        &self,
        name: TeamName,
        captain_id: UserID,
    ) -> Result<TeamDTO, AppError> {
        let team = Team::new(name, captain_id);
        let dto = team.clone().into();
        self.repos.save(team).await?;
        Ok(dto)
    }
}
