use crate::domain::models::{Answer, AnswerID, AnswerText, FileID, FullName, GroupName, MAX_TEAM_SIZE, Media, MediaID, MediaType, Points, SerialNumber, Task, TaskID, TaskText, TaskType, Team, TeamID, TeamName, User, Username, CharacterID, CharacterName, CharacterQuote, CharacterLegacy, CharacterFact};
use chrono::{DateTime, Utc};

pub struct UserDTO {
    pub username: Option<Username>,
    pub full_name: FullName,
    pub group_name: GroupName,
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        Self {
            username: u.username().cloned(),
            full_name: u.full_name().clone(),
            group_name: u.group_name().clone(),
        }
    }
}

pub struct TeamDTO {
    pub id: TeamID,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
    pub completed: bool,
}

impl From<Team> for TeamDTO {
    fn from(t: Team) -> Self {
        Self {
            id: t.id().clone(),
            name: t.name().clone(),
            size: t.member_ids().len(),
            max_size: MAX_TEAM_SIZE,
            completed: t.is_completed(),
        }
    }
}

pub struct Profile {
    pub full_name: FullName,
    pub group_name: GroupName,
    pub team_name: Option<TeamName>,
}

pub struct TeamWithMembersDTO {
    pub id: TeamID,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
    pub completed: bool,
    pub members: Vec<UserDTO>,
}

pub struct MediaDTO {
    pub id: MediaID,
    pub file_id: FileID,
    pub media_type: MediaType,
}

impl From<Media> for MediaDTO {
    fn from(m: Media) -> Self {
        Self {
            id: m.id().clone(),
            file_id: m.file_id().clone(),
            media_type: m.media_type().clone(),
        }
    }
}

pub struct TaskDTO {
    pub id: TaskID,
    pub index: SerialNumber,
    pub task_type: TaskType,
    pub media_id: MediaID,
    pub explanation: TaskText,
}

impl From<Task> for TaskDTO {
    fn from(t: Task) -> Self {
        Self {
            id: t.id().clone(),
            index: t.index(),
            task_type: t.task_type(),
            media_id: t.media_id().clone(),
            explanation: t.explanation().clone(),
        }
    }
}

pub struct UserTaskDTO {
    pub id: TaskID,
    pub index: SerialNumber,
    pub media_id: MediaID,
    pub explanation: TaskText,
    pub solved: bool,
}

pub struct AnswerDTO {
    pub id: AnswerID,
    pub task_id: TaskID,
    pub text: AnswerText,
    pub points: Points,
    pub solved: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Answer> for AnswerDTO {
    fn from(a: Answer) -> Self {
        Self {
            id: a.id().clone(),
            task_id: a.task_id().clone(),
            text: a.text().clone(),
            points: a.points(),
            solved: a.solved(),
            created_at: a.created_at().clone(),
        }
    }
}

pub struct CharacterDTO {
    pub id: CharacterID,
    pub name: CharacterName,
    pub quote: CharacterQuote,
    pub facts: Vec<CharacterFact>,
    pub legacy: CharacterLegacy,
    pub image_id: FileID,
}
