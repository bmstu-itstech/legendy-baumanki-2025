use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::FeedbackRepository;
use crate::domain::models::{Feedback, FeedbackText, UserID};

#[derive(Clone)]
pub struct GiveFeedback {
    feedback_repository: Arc<dyn FeedbackRepository>,
}

impl GiveFeedback {
    pub fn new(feedback_repository: Arc<dyn FeedbackRepository>) -> Self {
        Self {
            feedback_repository,
        }
    }

    pub async fn give_feedback(
        &self,
        author_id: UserID,
        text: FeedbackText,
    ) -> Result<(), AppError> {
        let feedback = Feedback::new(author_id, text);
        self.feedback_repository.save_feedback(feedback).await
    }
}
