use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TaskProvider, TeamByMemberProvider, TeamRepository, TrackProvider};
use crate::app::usecases::dto::AnswerDTO;
use crate::domain::models::TaskID;
use crate::domain::models::{TrackTag, UserID};

#[derive(Clone)]
pub struct AnswerTask {
    team_provider: Arc<dyn TeamByMemberProvider>,
    task_provider: Arc<dyn TaskProvider>,
    team_repository: Arc<dyn TeamRepository>,
    track_provider: Arc<dyn TrackProvider>,
}

impl AnswerTask {
    pub fn new(
        team_provider: Arc<dyn TeamByMemberProvider>,
        task_provider: Arc<dyn TaskProvider>,
        team_repository: Arc<dyn TeamRepository>,
        track_provider: Arc<dyn TrackProvider>,
    ) -> Self {
        Self {
            team_provider,
            task_provider,
            team_repository,
            track_provider,
        }
    }

    pub async fn execute(
        &self,
        user_id: UserID,
        track_tag: TrackTag,
        task_id: TaskID,
        text: String,
    ) -> Result<AnswerDTO, AppError> {
        match self.team_provider.team_by_member(user_id).await? {
            Some(mut team) => {
                let task = self.task_provider.task(task_id).await?;
                let answer = task.answer(&text);
                let dto = AnswerDTO {
                    points: answer.points(),
                    completed: answer.is_ok(),
                };
                team.save_answer(answer);

                let track = self.track_provider.track(track_tag).await?;
                let progress = track.progress(&team.answers());
                if progress.full_completed() {
                    team.finish_track(track_tag)?;
                }

                self.team_repository.save_team(team).await?;
                Ok(dto)
            }
            None => Err(AppError::UserNotInTeam(user_id)),
        }
    }
}
