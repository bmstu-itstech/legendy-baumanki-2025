use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamByMemberProvider;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct CheckCaptain {
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl CheckCaptain {
    pub fn new(team_provider: Arc<dyn TeamByMemberProvider>) -> Self {
        Self { team_provider }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<bool, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            None => Ok(false),
            Some(team) => Ok(team.is_captain(user_id)),
        }
    }
}
