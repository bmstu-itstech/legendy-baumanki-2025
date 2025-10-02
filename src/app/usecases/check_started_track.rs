use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TeamByMemberProvider;
use crate::domain::models::TrackTag;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct CheckStartedTrack {
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl CheckStartedTrack {
    pub fn new(team_provider: Arc<dyn TeamByMemberProvider>) -> Self {
        Self { team_provider }
    }

    pub async fn execute(&self, user_id: UserID, track_tag: TrackTag) -> Result<bool, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => Ok(team.track_is_started(track_tag)),
            None => Err(AppError::UserNotInTeam(user_id)),
        }
    }
}
