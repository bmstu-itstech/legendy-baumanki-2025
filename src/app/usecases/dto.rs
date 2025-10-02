use crate::domain::models::{CharacterFact, CharacterID, CharacterLegacy, CharacterName, CharacterQuote, FileID, FullName, GroupName, MAX_TEAM_SIZE, Media, MediaID, MediaType, Team, TeamID, TeamName, User, Username, TrackTag, TrackDescription, Track, TrackStatus, TaskID, TaskType, TaskText, TaskOption, Task, Points, CorrectAnswer};

pub struct UserDTO {
    pub username: Option<Username>,
    pub full_name: FullName,
    pub group_name: GroupName,
    pub team_id: Option<TeamID>,
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        Self {
            username: u.username().cloned(),
            full_name: u.full_name().clone(),
            group_name: u.group_name().clone(),
            team_id: u.team_id().cloned(),
        }
    }
}

pub struct TeamDTO {
    pub id: TeamID,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
}

impl From<Team> for TeamDTO {
    fn from(t: Team) -> Self {
        Self {
            id: t.id().clone(),
            name: t.name().clone(),
            size: t.member_ids().len(),
            max_size: MAX_TEAM_SIZE,
        }
    }
}

pub struct Profile {
    pub user: UserDTO,
    pub team_name: Option<TeamName>,
}

pub struct TeamWithMembersDTO {
    pub id: TeamID,
    pub solo: bool,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
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

pub struct CharacterDTO {
    pub id: CharacterID,
    pub name: CharacterName,
    pub quote: CharacterQuote,
    pub facts: Vec<CharacterFact>,
    pub legacy: CharacterLegacy,
    pub image_id: FileID,
}

pub struct TrackDescriptionDTO {
    pub tag: TrackTag,
    pub description: TrackDescription,
    pub media: MediaDTO,
}

impl TrackDescriptionDTO {
    pub fn new(track: &Track, media: MediaDTO) -> Self {
        Self {
            tag: track.tag(),
            description: track.description().clone(),
            media,
        }
    }
}

pub struct TrackInProgressDTO {
    pub tag: TrackTag,
    pub description: TrackDescription,
    pub media: MediaDTO,
    pub status: TrackStatus,
    pub percent: f32,
}

impl TrackInProgressDTO {
    pub fn new(track: &Track, media: MediaDTO, status: TrackStatus, percent: f32) -> Self {
        Self {
            tag: track.tag(),
            description: track.description().clone(),
            media,
            status,
            percent,
        }
    }
}

pub struct TaskDTO {
    pub id: TaskID,
    pub task_type: TaskType,
    pub question: TaskText,
    pub explanation: TaskText,
    pub media: Option<MediaDTO>,
    pub options: Vec<TaskOption>,
    pub correct_answers: Vec<CorrectAnswer>,
}

impl TaskDTO {
    pub fn new(task: &Task, media: Option<MediaDTO>) -> Self {
        Self {
            id: task.id(),
            task_type: task.task_type(),
            question: task.question().clone(),
            explanation: task.explanation().clone(),
            media,
            options: task.options().clone(),
            correct_answers: task.correct_answers().clone(),
        }
    }
}

pub struct AnswerDTO {
    pub points: Points,
    pub completed: bool,
}
