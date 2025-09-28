use crate::app::error::AppError;
use crate::domain::models::{
    Character, CharacterName, Media, MediaID, Task, TaskID, TaskType, Team, TeamID, User, UserID,
};

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
    async fn team(&self, id: &TeamID) -> Result<Team, AppError>;
}

#[async_trait::async_trait]
pub trait TeamByMemberProvider: Send + Sync {
    async fn team_by_member(&self, member_id: UserID) -> Result<Option<Team>, AppError>;
}

#[async_trait::async_trait]
pub trait UserRepository: UserProvider + Send + Sync {
    async fn save_user(&self, user: User) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait TeamRepository: TeamProvider + Send + Sync {
    async fn save_team(&self, team: Team) -> Result<(), AppError>;
    async fn delete_team(&self, team_id: &TeamID) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait IsAdminProvider: Send + Sync {
    async fn is_admin(&self, user_id: UserID) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait MediaProvider: Send + Sync {
    async fn media(&self, id: &MediaID) -> Result<Media, AppError>;
}

#[async_trait::async_trait]
pub trait MediaRepository: MediaProvider + Send + Sync {
    async fn save_media(&self, media: Media) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait TaskProvider: Send + Sync {
    async fn task(&self, task_id: TaskID) -> Result<Task, AppError>;
}

#[async_trait::async_trait]
pub trait TasksProvider: Send + Sync {
    async fn tasks(&self, task_type: TaskType) -> Result<Vec<Task>, AppError>;
}

#[async_trait::async_trait]
pub trait CharactersProvider: Send + Sync {
    async fn characters(&self) -> Result<Vec<Character>, AppError>;
    async fn character_by_name(&self, name: &CharacterName) -> Result<Option<Character>, AppError>;
}
