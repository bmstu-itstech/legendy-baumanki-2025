use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TeamByMemberProvider, UserProvider};
use crate::app::usecases::dto::Profile;
use crate::domain::models::UserID;

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
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => Ok(Profile {
                username: user.username().cloned(),
                full_name: user.full_name().clone(),
                group_name: user.group_name().clone(),
                team_name: Some(team.name().clone()),
                mode: user.status().clone(),
            }),
            None => Ok(Profile {
                username: user.username().cloned(),
                full_name: user.full_name().clone(),
                group_name: user.group_name().clone(),
                team_name: None,
                mode: user.status().clone(),
            }),
        }
    }
}
