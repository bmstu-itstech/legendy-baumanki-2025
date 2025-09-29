use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamByMemberProvider, TeamRepository, UserRepository};
use crate::domain::error::DomainError;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct ExitTeam {
    team_by_member_provider: Arc<dyn TeamByMemberProvider>,
    team_repos: Arc<dyn TeamRepository>,
    user_repos: Arc<dyn UserRepository>,
}

impl ExitTeam {
    pub fn new(
        team_by_member_provider: Arc<dyn TeamByMemberProvider>,
        team_repos: Arc<dyn TeamRepository>,
        user_repos: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            team_by_member_provider,
            team_repos,
            user_repos,
        }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<(), AppError> {
        let mut user = self.user_repos.user(user_id).await?;
        match self.team_by_member_provider.team_by_member(user_id).await? {
            None => Err(AppError::DomainError(DomainError::UserIsNotMemberOfTeam(
                user_id.as_i64(),
            ))),
            Some(team) => {
                let team_id = team.id().clone();
                match team.remove_member(user_id)? {
                    Some(team) => self.team_repos.save_team(team).await?,
                    None => self.team_repos.delete_team(&team_id).await?,
                }
                user.switch_to_want_team_mode();
                self.user_repos.save_user(user).await
            }
        }
    }
}
