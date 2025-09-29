use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamByMemberProvider;
use crate::app::usecases::dto::TeamDTO;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct GetUserTeam {
    provider: Arc<dyn TeamByMemberProvider>,
}

impl GetUserTeam {
    pub fn new(provider: Arc<dyn TeamByMemberProvider>) -> GetUserTeam {
        Self { provider }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<Option<TeamDTO>, AppError> {
        match self.provider.team_by_member(user_id).await? {
            Some(team) => Ok(Some(team.into())),
            None => Ok(None),
        }
    }
}
