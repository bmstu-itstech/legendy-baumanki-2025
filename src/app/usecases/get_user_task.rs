use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TaskProvider, UserProvider};
use crate::app::usecases::dto::UserTaskDTO;
use crate::domain::models::{TaskID, UserID};

#[derive(Clone)]
pub struct GetUserTask {
    task_provider: Arc<dyn TaskProvider>,
    user_provider: Arc<dyn UserProvider>,
}

impl GetUserTask {
    pub fn new(task_provider: Arc<dyn TaskProvider>, user_provider: Arc<dyn UserProvider>) -> Self {
        Self {
            task_provider,
            user_provider,
        }
    }

    pub async fn execute(&self, user_id: UserID, task_id: TaskID) -> Result<UserTaskDTO, AppError> {
        let task = self.task_provider.task(task_id).await?;
        let user = self.user_provider.user(user_id).await?;
        let solved = if let Some(answer) = user.answer(task.id()) {
            answer.solved()
        } else {
            false
        };
        Ok(UserTaskDTO {
            id: task.id().clone(),
            index: task.index(),
            media_id: task.media_id().clone(),
            explanation: task.explanation().clone(),
            solved,
        })
    }
}
