use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{MediaProvider, TeamByMemberProvider, TeamRepository, TrackProvider};
use crate::app::usecases::dto::{MediaDTO, TrackInProgressDTO};
use crate::domain::models::{TrackTag, UserID};

#[derive(Clone)]
pub struct StartTrack {
    team_provider: Arc<dyn TeamByMemberProvider>,
    team_repository: Arc<dyn TeamRepository>,
    track_provider: Arc<dyn TrackProvider>,
    media_provider: Arc<dyn MediaProvider>,
}

impl StartTrack {
    pub fn new(
        team_provider: Arc<dyn TeamByMemberProvider>,
        team_repository: Arc<dyn TeamRepository>,
        track_provider: Arc<dyn TrackProvider>,
        media_provider: Arc<dyn MediaProvider>,
    ) -> Self {
        Self { team_provider, team_repository, track_provider, media_provider }
    }
    
    pub async fn execute(&self, user_id: UserID, track_tag: TrackTag) -> Result<TrackInProgressDTO, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            Some(mut team) => {
                team.start_track(track_tag)?;
                let track = self.track_provider.track(track_tag).await?;
                let answers = team.answers();
                let progress = track.progress(&answers);
                let media = MediaDTO::from(self.media_provider.media(track.media_id()).await?);
                let dto = TrackInProgressDTO {
                    tag: track_tag,
                    description: track.description().clone(),
                    media,
                    status: team.track_status(track_tag)?.clone(),
                    percent: progress.percent(),
                };
                self.team_repository.save_team(team).await?;
                Ok(dto)
            },
            None => Err(AppError::UserNotInTeam(user_id)),
        }
    }
}
