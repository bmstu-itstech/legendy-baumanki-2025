use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserRepository;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct SwitchToLookingForTeam {
    user_repository: Arc<dyn UserRepository>,
}

impl SwitchToLookingForTeam {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: UserID) -> Result<(), AppError> {
        let mut user = self.user_repository.user(id).await?;
        user.switch_to_looking_for_team()?;
        self.user_repository.save_user(user).await
    }
}
