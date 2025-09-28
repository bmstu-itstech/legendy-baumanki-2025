use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserProvider;
use crate::app::usecases::dto::UserDTO;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct GetUser {
    user_provider: Arc<dyn UserProvider>,
}

impl GetUser {
    pub fn new(user_provider: Arc<dyn UserProvider>) -> Self {
        Self { user_provider }
    }

    pub async fn user(&self, id: UserID) -> Result<UserDTO, AppError> {
        let user = self.user_provider.user(id).await?;
        Ok(UserDTO::from(user))
    }
}
