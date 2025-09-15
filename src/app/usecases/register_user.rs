use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserRepository;
use crate::domain::models::{FullName, GroupName, User, UserID, Username};

#[derive(Clone)]
pub struct RegisterUser {
    repos: Arc<dyn UserRepository>,
}

impl RegisterUser {
    pub fn new(repos: Arc<dyn UserRepository>) -> Self {
        Self { repos }
    }

    pub async fn register(
        &self,
        id: UserID,
        username: Option<Username>,
        full_name: FullName,
        group_name: GroupName,
    ) -> Result<User, AppError> {
        let user = User::new(id, username, full_name, group_name);
        self.repos.save_user(user.clone()).await?;
        Ok(user.into())
    }
}
