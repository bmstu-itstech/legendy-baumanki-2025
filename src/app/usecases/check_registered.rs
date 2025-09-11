use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::IsRegisteredUserProvider;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct CheckRegistered {
    provider: Arc<dyn IsRegisteredUserProvider>,
}

impl CheckRegistered {
    pub fn new(provider: Arc<dyn IsRegisteredUserProvider>) -> Self {
        Self { provider }
    }

    pub async fn is_registered(&self, user_id: UserID) -> Result<bool, AppError> {
        self.provider.is_registered(user_id).await
    }
}
