use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{SlotRepository, SlotsProvider, TeamByMemberProvider, TeamRepository};
use crate::domain::models::UserID;

#[derive(Clone)]
pub struct CancelReservation {
    team_provider: Arc<dyn TeamByMemberProvider>,
    team_repository: Arc<dyn TeamRepository>,
    slots_provider: Arc<dyn SlotsProvider>,
    slots_repository: Arc<dyn SlotRepository>,
}

impl CancelReservation {
    pub fn new(
        team_provider: Arc<dyn TeamByMemberProvider>,
        team_repository: Arc<dyn TeamRepository>,
        slots_provider: Arc<dyn SlotsProvider>,
        slots_repository: Arc<dyn SlotRepository>,
    ) -> Self {
        Self {
            team_provider,
            team_repository,
            slots_provider,
            slots_repository,
        }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<(), AppError> {
        let mut team = self
            .team_provider
            .team_by_member(user_id)
            .await?
            .ok_or(AppError::UserNotInTeam(user_id))?;
        let slot_id = team.cancel_reservation()?;
        let mut slot = self.slots_repository.slot(&slot_id).await?;
        slot.cancel_reservation(team.id())?;
        self.slots_repository.save_slot(slot).await?;
        self.team_repository.save_team(team).await?;
        Ok(())
    }
}
