use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TaskProvider, UserRepository};
use crate::app::usecases::dto::AnswerDTO;
use crate::domain::models::{TaskID, UserID};

#[derive(Clone)]
pub struct AnswerTask {
    task_provider: Arc<dyn TaskProvider>,
    user_repository: Arc<dyn UserRepository>,
}

impl AnswerTask {
    pub fn new(
        task_provider: Arc<dyn TaskProvider>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            task_provider,
            user_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: UserID,
        task_id: TaskID,
        text: String,
    ) -> Result<AnswerDTO, AppError> {
        let task = self.task_provider.task(task_id).await?;
        let mut user = self.user_repository.user(user_id).await?;
        let answer = task.answer(text);
        let dto = answer.clone().into();
        user.add_answer(answer);
        self.user_repository.save_user(user).await?;
        Ok(dto)
    }
}
