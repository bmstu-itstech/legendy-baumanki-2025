use std::sync::Arc;

use crate::domain::models::UserID;
use crate::app::error::AppError;
use crate::app::ports::{TeamByMemberProvider, TrackProvider};
use crate::domain::models::{TaskID, TrackTag};

#[derive(Clone)]
pub struct GetCompletedTasks {
    track_provider: Arc<dyn TrackProvider>,
    team_provider: Arc<dyn TeamByMemberProvider>,
}

impl GetCompletedTasks {
    pub fn new(track_provider: Arc<dyn TrackProvider>, team_provider: Arc<dyn TeamByMemberProvider>) -> Self {
        Self { track_provider, team_provider }
    }

    pub async fn execute(&self, user_id: UserID, track_tag: TrackTag) -> Result<Vec<TaskID>, AppError> {
        let track = self.track_provider.track(track_tag).await?;
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => {
                let progress = track.progress(&team.answers());
                let ids = progress.completed_tasks()
                    .iter()
                    .map(|&t| t.id())
                    .collect();
                Ok(ids)
            },
            None => Err(AppError::UserNotInTeam(user_id))?,
        }
    }
}

