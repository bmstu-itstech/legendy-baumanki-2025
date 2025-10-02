use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{MediaProvider, TaskProvider};
use crate::app::usecases::dto::TaskDTO;
use crate::app::usecases::dto::MediaDTO;
use crate::domain::models::TaskID;

#[derive(Clone)]
pub struct GetTask {
    task_provider: Arc<dyn TaskProvider>,
    media_provider: Arc<dyn MediaProvider>,
}

impl GetTask {
    pub fn new(task_provider: Arc<dyn TaskProvider>, media_provider: Arc<dyn MediaProvider>) -> Self {
        Self { task_provider, media_provider }
    }
    
    pub async fn execute(&self, id: TaskID) -> Result<TaskDTO, AppError> {
        let task = self.task_provider.task(id).await?;
        let media = if let Some(media_id) = task.media_id() {
            let media = self.media_provider.media(media_id).await?;
            Some(MediaDTO::from(media))
        } else {
            None
        };
        Ok(TaskDTO::new(&task, media))
    }
}
