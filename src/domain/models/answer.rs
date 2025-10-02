use chrono::{DateTime, Utc};

use crate::domain::error::DomainError;
use crate::domain::models::points::Points;
use crate::domain::models::TaskID;
use crate::pseudo_uuid_impl;
use crate::utils::uuid::new_pseudo_uuid;

#[derive(Debug, Clone)]
pub struct AnswerText(String);

impl AnswerText {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    task_id: TaskID,
    text: AnswerText,
    points: Points,
    created_at: DateTime<Utc>,
}

impl Answer {
    pub fn new(task_id: TaskID, text: AnswerText, points: Points) -> Self {
        Self {
            task_id,
            text,
            points,
            created_at: Utc::now(),
        }
    }

    pub fn restore(
        task_id: TaskID,
        text: AnswerText,
        points: Points,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            task_id,
            text,
            points,
            created_at,
        }
    }

    pub fn task_id(&self) ->TaskID {
        self.task_id
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

    pub fn is_ok(&self) -> bool {
        self.points.is_positive()
    }
    
    pub fn is_failed(&self) -> bool {
        self.points.is_zero()
    }
}
