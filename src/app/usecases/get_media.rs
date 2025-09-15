use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{MediaProvider};
use crate::domain::models::{Media, MediaID};

#[derive(Clone)]
pub struct GetMedia {
    provider: Arc<dyn MediaProvider + Send + Sync>,
}

impl GetMedia {
    pub fn new(provider: Arc<dyn MediaProvider + Send + Sync>) -> Self {
        Self { provider }
    }
    
    pub async fn media(&self, id: MediaID) -> Result<Media, AppError> {
        self.provider.media(id).await.map(Into::into)
    }
}
