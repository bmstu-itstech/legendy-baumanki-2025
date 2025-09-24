use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamByMemberProvider, TeamRepository};
use crate::domain::error::DomainError;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct ExitTeam {
    provider: Arc<dyn TeamByMemberProvider>,
    repos: Arc<dyn TeamRepository>,
}

impl ExitTeam {
    pub fn new(provider: Arc<dyn TeamByMemberProvider>, repos: Arc<dyn TeamRepository>) -> Self {
        Self { provider, repos }
    }

    pub async fn exit(&self, user_id: UserID) -> Result<(), AppError> {
        match self.provider.team_by_member(user_id).await? {
            None => Err(AppError::DomainError(DomainError::UserIsNotMemberOfTeam(
                user_id.as_i64(),
            ))),
            Some(team) => {
                let team_id = team.id().clone();
                match team.remove_member(user_id)? {
                    Some(team) => self.repos.save_team(team).await,
                    None => self.repos.delete_team(&team_id).await,
                }
            }
        }
    }
}
