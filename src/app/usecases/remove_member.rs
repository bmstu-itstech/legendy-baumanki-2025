use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamRepository;
use crate::domain::models::{TeamID, UserID};

#[derive(Clone)]
pub struct RemoveMember {
    repos: Arc<dyn TeamRepository>,
}

impl RemoveMember {
    pub fn new(repos: Arc<dyn TeamRepository>) -> Self {
        Self { repos }
    }

    pub async fn remove_member(&self, user_id: UserID, team_id: TeamID) -> Result<(), AppError> {
        let team = self.repos.team(team_id.clone()).await?;
        match team.remove_member(user_id).map_err(AppError::DomainError)? {
            Some(team) => self.repos.save_team(team).await,
            None => self.repos.delete_team(&team_id).await,
        }
    }
}
