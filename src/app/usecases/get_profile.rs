use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamByMemberProvider, UserProvider};
use crate::app::usecases::dto::Profile;
use crate::domain::models::UserID;

use super::dto::UserDTO;

#[derive(Clone)]
pub struct GetProfile {
    user_provider: Arc<dyn UserProvider>,
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl GetProfile {
    pub fn new(
        user_provider: Arc<dyn UserProvider>,
        team_provider: Arc<dyn TeamByMemberProvider>,
    ) -> GetProfile {
        Self {
            user_provider,
            team_provider,
        }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<Profile, AppError> {
        let user = self.user_provider.user(user_id).await?;
        let dto = UserDTO::from(user);
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => Ok(Profile {
                user: dto,
                team_name: Some(team.name().clone()),
            }),
            None => Ok(Profile {
                user: dto,
                team_name: None,
            }),
        }
    }
}
