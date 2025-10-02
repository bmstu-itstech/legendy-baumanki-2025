use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{MediaProvider, TeamByMemberProvider, TrackProvider};
use crate::app::usecases::dto::{MediaDTO, TrackInProgressDTO};
use crate::domain::models::{TrackTag, UserID};

#[derive(Clone)]
pub struct GetTrackInProgress {
    team_provider: Arc<dyn TeamByMemberProvider>,
    track_provider: Arc<dyn TrackProvider>,
    media_provider: Arc<dyn MediaProvider>,
}

impl GetTrackInProgress {
    pub fn new(team_provider: Arc<dyn TeamByMemberProvider>, track_provider: Arc<dyn TrackProvider>, media_provider: Arc<dyn MediaProvider>) -> Self {
        Self { team_provider, track_provider, media_provider }
    }
    
    pub async fn execute(&self, user_id: UserID, track_tag: TrackTag) -> Result<TrackInProgressDTO, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            Some(team) => {
                let track = self.track_provider.track(track_tag).await?;
                let answers = team.answers();
                let progress = track.progress(&answers);
                let media = MediaDTO::from(self.media_provider.media(track.media_id()).await?);
                Ok(TrackInProgressDTO::new(&track, media, team.track_status(track_tag)?.clone(), progress.percent()))
            }
            None => Err(AppError::UserNotInTeam(user_id)),
        }
    }
}
