use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::MediaRepository;
use crate::domain::models::Media;

#[derive(Clone)]
pub struct UploadMedia {
    repos: Arc<dyn MediaRepository>,
}

impl UploadMedia {
    pub fn new(repos: Arc<dyn MediaRepository>) -> Self {
        Self { repos }
    }

    pub async fn upload_media(&self, media: Media) -> Result<(), AppError> {
        self.repos.save_media(media).await
    }
}
