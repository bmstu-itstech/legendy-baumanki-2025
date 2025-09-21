use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::TaskProvider;
use crate::app::usecases::dto::TaskDTO;
use crate::domain::models::TaskID;

#[derive(Clone)]
pub struct GetTask {
    provider: Arc<dyn TaskProvider>
}

impl GetTask {
    pub fn new(provider: Arc<dyn TaskProvider>) -> Self {
        Self { provider }
    }
    
    pub async fn task(&self, id: TaskID) -> Result<TaskDTO, AppError> {
        self.provider.task(id).await.map(Into::into)
    }
}
