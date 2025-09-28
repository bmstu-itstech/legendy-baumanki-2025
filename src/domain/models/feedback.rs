use crate::domain::error::DomainError;
use crate::domain::models::UserID;
use crate::not_empty_string_impl;

pub struct FeedbackText(String);
not_empty_string_impl!(FeedbackText);

pub struct Feedback {
    author_id: UserID,
    text: FeedbackText,
}

impl Feedback {
    pub fn new(author_id: UserID, text: FeedbackText) -> Self {
        Self { author_id, text }
    }

    pub fn author_id(&self) -> UserID {
        self.author_id
    }

    pub fn text(&self) -> &FeedbackText {
        &self.text
    }
}
