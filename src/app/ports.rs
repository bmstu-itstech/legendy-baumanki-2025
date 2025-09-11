use crate::app::error::AppError;
use crate::domain::models::{Team, TeamID, User, UserID};

#[async_trait::async_trait]
pub trait UserProvider: Send + Sync {
    async fn user(&self, id: UserID) -> Result<User, AppError>;
}

#[async_trait::async_trait]
pub trait IsRegisteredUserProvider: Send + Sync {
    async fn is_registered(&self, user_id: UserID) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait TeamProvider: Send + Sync {
    async fn team(&self, id: TeamID) -> Result<Team, AppError>;
}

#[async_trait::async_trait]
pub trait TeamByMemberProvider: Send + Sync {
    async fn team_by_member(&self, member_id: UserID) -> Result<Option<Team>, AppError>;
}

#[async_trait::async_trait]
pub trait IsTeamExistsProvider: Send + Sync {
    async fn is_team_exists(&self, team_id: &TeamID) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait UserRepository: UserProvider + Send + Sync {
    async fn save(&self, user: User) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait TeamRepository: TeamProvider + Send + Sync {
    async fn save(&self, team: Team) -> Result<(), AppError>;
    async fn delete(&self, team_id: &TeamID) -> Result<(), AppError>;
}
