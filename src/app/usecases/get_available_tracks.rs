use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamByMemberProvider;
use crate::domain::models::{TrackTag, UserID};

#[derive(Clone)]
pub struct GetAvailableTracks {
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl GetAvailableTracks {
    pub fn new(team_provider: Arc<dyn TeamByMemberProvider>) -> Self {
        Self { team_provider }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<Vec<TrackTag>, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => Ok(team.available_tracks().into()),
            None => Err(AppError::UserNotInTeam(user_id)),
        }
    }
}
