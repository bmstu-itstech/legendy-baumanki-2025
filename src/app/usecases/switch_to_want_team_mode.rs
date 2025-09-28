use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::UserRepository;
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct SwitchToWantTeamMode {
    user_repository: Arc<dyn UserRepository>,
}

impl SwitchToWantTeamMode {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn switch_to_want_team_mode(&self, id: UserID) -> Result<(), AppError> {
        let mut user = self.user_repository.user(id).await?;
        user.switch_to_want_team_mode();
        self.user_repository.save_user(user).await
    }
}
