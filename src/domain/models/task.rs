use levenshtein::levenshtein;
use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;
use crate::domain::models::points::Points;
use crate::domain::models::{Answer, AnswerText, MediaID};
use crate::utils::uuid::new_pseudo_uuid;
use crate::{not_empty_string_impl, pseudo_uuid_impl};

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Rebus,
    Riddle,
}

impl TaskType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskType::Rebus => "Ребус",
            TaskType::Riddle => "Загадка",
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TaskID(String);
pseudo_uuid_impl!(TaskID, 6);

pub type SerialNumber = u32;

#[derive(Debug, Clone)]
pub struct TaskText(String);
not_empty_string_impl!(TaskText);

#[derive(Debug, Clone)]
pub struct CorrectAnswer(String);

impl CorrectAnswer {
    pub fn new(mut s: String) -> Result<Self, DomainError> {
        if s == "" {
            return Err(DomainError::InvalidValue(
                "invalid CorrectAnswer: expected not empty string".to_string(),
            ));
        }
        s = normalize_string(s);
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}


pub type LevenshteinDistance = usize;

#[derive(Debug, Clone)]
pub struct Task {
    id: TaskID,
    index: SerialNumber,
    task_type: TaskType,
    media_id: MediaID,
    explanation: TaskText,
    correct_answer: CorrectAnswer,
    points: Points,
    max_levenshtein_distance: LevenshteinDistance,
}

impl Task {
    pub fn new(
        index: SerialNumber,
        task_type: TaskType,
        media_id: MediaID,
        explanation: TaskText,
        correct_answer: CorrectAnswer,
        points: Points,
        max_levenshtein_distance: LevenshteinDistance,
    ) -> Self {
        Self {
            id: TaskID::new(),
            index,
            task_type,
            media_id,
            explanation,
            correct_answer,
            points,
            max_levenshtein_distance,
        }
    }

    pub fn restore(
        id: TaskID,
        index: SerialNumber,
        task_type: TaskType,
        media_id: MediaID,
        explanation: TaskText,
        correct_answer: CorrectAnswer,
        points: Points,
        max_levenshtein_distance: LevenshteinDistance,
    ) -> Self {
        Self {
            id,
            index,
            task_type,
            media_id,
            explanation,
            correct_answer,
            points,
            max_levenshtein_distance,
        }
    }

    pub fn answer(&self, text: String) -> Answer {
        let points = if self.answer_match(&text) {
            self.points
        } else {
            Points::zero()
        };
        Answer::new(self.id.clone(), AnswerText::new(text), points)
    }

    fn answer_match(&self, text: &str) -> bool {
        let text = normalize_string(text);
        if text == self.correct_answer.as_str() {
            true
        } else {
            levenshtein(&text, self.correct_answer.as_str()) <= self.max_levenshtein_distance
        }
    }

    pub fn id(&self) -> &TaskID {
        &self.id
    }

    pub fn index(&self) -> SerialNumber {
        self.index
    }

    pub fn task_type(&self) -> TaskType {
        self.task_type
    }

    pub fn media_id(&self) -> &MediaID {
        &self.media_id
    }

    pub fn explanation(&self) -> &TaskText {
        &self.explanation
    }

    pub fn correct_answer(&self) -> &CorrectAnswer {
        &self.correct_answer
    }
}

fn normalize_string(s: impl Into<String>) -> String {
    s.into()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}
