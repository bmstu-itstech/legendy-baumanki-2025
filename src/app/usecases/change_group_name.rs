use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserRepository;
use crate::domain::models::{GroupName, UserID};

#[derive(Clone)]
pub struct ChangeGroupName {
    repos: Arc<dyn UserRepository>,
}

impl ChangeGroupName {
    pub fn new(repos: Arc<dyn UserRepository>) -> Self {
        Self { repos }
    }

    pub async fn change_group_name(&self, user_id: UserID, new: GroupName) -> Result<(), AppError> {
        let mut user = self.repos.user(user_id).await?;
        user.change_group_name(new);
        self.repos.save(user).await?;
        Ok(())
    }
}
