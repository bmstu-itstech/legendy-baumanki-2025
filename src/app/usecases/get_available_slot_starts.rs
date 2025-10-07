use chrono::NaiveTime;
use std::collections::HashSet;
use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::SlotsProvider;

#[derive(Clone)]
pub struct GetAvailableSlotStarts {
    slots_provider: Arc<dyn SlotsProvider>,
}

impl GetAvailableSlotStarts {
    pub fn new(slots_provider: Arc<dyn SlotsProvider>) -> Self {
        Self { slots_provider }
    }

    pub async fn execute(&self) -> Result<Vec<NaiveTime>, AppError> {
        let slots = self.slots_provider.slots().await?;
        let mut starts: HashSet<NaiveTime> = HashSet::new();
        for slot in &slots {
            if slot.available_places() > 0 {
                starts.insert(slot.start());
            }
        }
        let mut starts: Vec<_> = starts.into_iter().collect();
        starts.sort_by(|l, r| l.cmp(r));
        Ok(starts)
    }
}
