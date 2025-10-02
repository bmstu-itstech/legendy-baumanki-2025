use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamByMemberProvider;
use crate::app::ports::UserProvider;
use crate::app::usecases::dto::PlayerDTO;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct GetPlayer {
    user_provider: Arc<dyn UserProvider>,
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl GetPlayer {
    pub fn new(
        user_provider: Arc<dyn UserProvider>,
        team_provider: Arc<dyn TeamByMemberProvider>,
    ) -> Self {
        Self {
            user_provider,
            team_provider,
        }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<PlayerDTO, AppError> {
        let user = self.user_provider.user(user_id).await?;
        let solo_team = if let Some(team) = self.team_provider.team_by_member(user_id).await? {
            team.is_solo()
        } else {
            true
        };
        Ok(PlayerDTO {
            username: user.username().cloned(),
            solo_team,
        })
    }
}
