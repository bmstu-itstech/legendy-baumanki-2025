use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::IsAdminProvider;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct CheckAdmin {
    provider: Arc<dyn IsAdminProvider>,
}

impl CheckAdmin {
    pub fn new(provider: Arc<dyn IsAdminProvider>) -> Self {
        Self { provider }
    }

    pub async fn is_admin(&self, user_id: UserID) -> Result<bool, AppError> {
        self.provider.is_admin(user_id).await
    }
}
