use chrono::{DateTime, Utc};

use crate::domain::error::DomainError;
use crate::domain::models::TaskID;
use crate::domain::models::points::Points;
use crate::utils::short_uuid::new_short_uuid;

pub const ANSWER_ID_LENGTH: usize = 8;

#[derive(Debug, Clone)]
pub struct AnswerID(String);

impl AnswerID {
    pub fn new() -> Self {
        Self(new_short_uuid(ANSWER_ID_LENGTH))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<String> for AnswerID {
    type Error = DomainError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != ANSWER_ID_LENGTH {
            return Err(DomainError::InvalidValue(format!(
                "expected AnswerID length = {}, got {}",
                ANSWER_ID_LENGTH,
                s.len()
            )));
        }
        if !s.chars().into_iter().all(|c| c.is_alphanumeric()) {
            return Err(DomainError::InvalidValue(format!(
                "invalid AnswerID: expected alphanumeric character, got {}",
                s
            )));
        }
        Ok(Self(s))
    }
}

#[derive(Debug, Clone)]
pub struct AnswerText(String);

impl AnswerText {
    pub fn new(text: String) -> Self {
        Self(text)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    id: AnswerID,
    task_id: TaskID,
    text: AnswerText,
    points: Points,
    created_at: DateTime<Utc>,
}

impl Answer {
    pub fn new(task_id: TaskID, text: AnswerText, points: Points) -> Self {
        Self {
            id: AnswerID::new(),
            task_id,
            text,
            points,
            created_at: Utc::now(),
        }
    }
    
    pub fn restore(id: AnswerID, task_id: TaskID, text: AnswerText, points: Points, created_at: DateTime<Utc>) -> Self {
        Self { id, task_id, text, points, created_at, }
    }
    
    pub fn id(&self) -> &AnswerID {
        &self.id
    }
    
    pub fn task_id(&self) -> &TaskID {
        &self.task_id
    }
    
    pub fn text(&self) -> &AnswerText {
        &self.text
    }
    
    pub fn points(&self) -> Points {
        self.points
    }
    
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    
    pub fn solved(&self) -> bool {
        self.points > Points::zero()
    }
}
