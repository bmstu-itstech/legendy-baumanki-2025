use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{TasksProvider, UserProvider};
use crate::app::usecases::dto::UserTaskDTO;
use crate::domain::models::{TaskType, UserID};

#[derive(Clone)]
pub struct GetUserTasks {
    tasks_provider: Arc<dyn TasksProvider>,
    user_provider: Arc<dyn UserProvider>,
}

impl GetUserTasks {
    pub fn new(tasks_provider: Arc<dyn TasksProvider>, user_provider: Arc<dyn UserProvider>) -> Self {
        Self { tasks_provider, user_provider }
    }
    
    pub async fn tasks(&self, user_id: UserID, task_type: TaskType) -> Result<Vec<UserTaskDTO>, AppError> {
        let tasks = self.tasks_provider.tasks(task_type).await?;
        let user = self.user_provider.user(user_id).await?;
        
        let mut user_tasks = Vec::new();
        for task in tasks {
            let solved = if let Some(answer) = user.answer(task.id()) {
                answer.solved()
            } else {
                false
            };
            user_tasks.push(UserTaskDTO {
                id: task.id().clone(),
                index: task.index(),
                media_id: task.media_id().clone(),
                explanation: task.explanation().clone(),
                solved,
            });
        }
        Ok(user_tasks)
    }
}
