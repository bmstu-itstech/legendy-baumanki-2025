use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserRepository;
use crate::domain::models::{FullName, UserID};

#[derive(Clone)]
pub struct ChangeFullName {
    repos: Arc<dyn UserRepository>,
}

impl ChangeFullName {
    pub fn new(repos: Arc<dyn UserRepository>) -> Self {
        Self { repos }
    }

    pub async fn change_full_name(&self, user_id: UserID, new: FullName) -> Result<(), AppError> {
        let mut user = self.repos.user(user_id).await?;
        user.change_full_name(new);
        self.repos.save_user(user).await?;
        Ok(())
    }
}
