use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::IsTeamExistsProvider;
use crate::domain::models::TeamID;

#[derive(Clone)]
pub struct CheckTeamExists {
    provider: Arc<dyn IsTeamExistsProvider>,
}

impl CheckTeamExists {
    pub fn new(provider: Arc<dyn IsTeamExistsProvider>) -> Self {
        Self { provider }
    }

    pub async fn team_exists(&self, team_id: TeamID) -> Result<bool, AppError> {
        self.provider.is_team_exists(&team_id).await
    }
}
