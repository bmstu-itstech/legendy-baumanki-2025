use levenshtein::levenshtein;
use crate::domain::error::DomainError;
use crate::domain::models::{Answer, AnswerText, MediaID, Points};
use crate::not_empty_string_impl;

pub type TaskID = i32;

#[derive(Debug, Clone)]
pub struct TaskText(String);
not_empty_string_impl!(TaskText);

#[derive(Debug, Clone)]
pub struct TaskOption(String);
not_empty_string_impl!(TaskOption);

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Text,
    Choice,
    Photo,
}

fn normalize(s: String) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}

#[derive(Debug, Clone)]
pub struct CorrectAnswer(String);

impl CorrectAnswer {
    pub fn new(mut s: String) -> Result<Self, DomainError> {
        s = normalize(s);
        if s.is_empty() {
            Err(DomainError::InvalidValue("invalid CorrectAnswer: expected not empty string".to_string()))
        } else {
            Ok(Self(s))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    id: TaskID,
    task_type: TaskType,
    question: TaskText,
    explanation: TaskText,
    media_id: Option<MediaID>,
    options: Vec<TaskOption>,
    dependencies: Vec<TaskID>,
    correct_answers: Vec<CorrectAnswer>,
    points: Points,
    price: Points,
    max_levenshtein_distance: usize,
}

impl Task {
    pub fn new(
        id: TaskID,
        task_type: TaskType,
        question: TaskText,
        explanation: TaskText,
        media_id: Option<MediaID>,
        options: Vec<TaskOption>,
        dependencies: Vec<TaskID>,
        correct_answers: Vec<CorrectAnswer>,
        points: Points,
        price: Points,
        max_levenshtein_distance: usize,
    ) -> Self {
        Self {
            id,
            task_type,
            question,
            explanation,
            media_id,
            options,
            dependencies,
            correct_answers,
            points,
            price,
            max_levenshtein_distance,
        }
    }

    pub fn answer(&self, answer: &str) -> Answer {
        let answer = AnswerText::new(normalize(answer.to_string()));
        for correct in self.correct_answers.iter() {
            if levenshtein(answer.as_str(), correct.as_str()) <= self.max_levenshtein_distance {
                return Answer::new(self.id, answer, self.points);
            }
        }
        Answer::new(self.id, answer, Points::zero())
    }

    pub fn id(&self) -> TaskID {
        self.id
    }

    pub fn task_type(&self) -> TaskType {
        self.task_type
    }

    pub fn question(&self) -> &TaskText {
        &self.question
    }

    pub fn explanation(&self) -> &TaskText {
        &self.explanation
    }

    pub fn media_id(&self) -> Option<&MediaID> {
        self.media_id.as_ref()
    }

    pub fn options(&self) -> &Vec<TaskOption> {
        &self.options
    }

    pub fn dependencies(&self) -> &Vec<TaskID> {
        &self.dependencies
    }

    pub fn points(&self) -> Points {
        self.points
    }

    pub fn price(&self) -> Points {
        self.price
    }
    
    pub fn correct_answers(&self) -> &Vec<CorrectAnswer> {
        &self.correct_answers
    }
}
