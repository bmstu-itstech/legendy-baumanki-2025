use chrono::NaiveTime;
use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{SlotRepository, SlotsProvider, TeamByMemberProvider, TeamRepository};
use crate::app::usecases::dto::SlotDTO;
use crate::domain::models::{Places, UserID};

#[derive(Clone)]
pub struct ReserveSlot {
    slots_provider: Arc<dyn SlotsProvider>,
    slot_repository: Arc<dyn SlotRepository>,
    team_provider: Arc<dyn TeamByMemberProvider>,
    team_repository: Arc<dyn TeamRepository>,
}

impl ReserveSlot {
    pub fn new(
        slots_provider: Arc<dyn SlotsProvider>,
        slot_repository: Arc<dyn SlotRepository>,
        team_provider: Arc<dyn TeamByMemberProvider>,
        team_repository: Arc<dyn TeamRepository>,
    ) -> Self {
        Self {
            slots_provider,
            slot_repository,
            team_provider,
            team_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: UserID,
        start: NaiveTime,
        places: Places,
    ) -> Result<SlotDTO, AppError> {
        let mut team = self
            .team_provider
            .team_by_member(user_id)
            .await?
            .ok_or(AppError::UserNotInTeam(user_id))?;
        if places > team.size() {
            return Err(AppError::PlacesGreaterThanTeamSize(places, team.size()));
        }

        let slots = self.slots_provider.slots_by_start(start).await?;
        let available_slots: Vec<_> = slots
            .into_iter()
            .filter(|s| s.can_be_reserved(places))
            .collect();

        let best_slot = available_slots
            .into_iter()
            .min_by_key(|s| s.available_places());
        if let Some(mut best_slot) = best_slot {
            best_slot.reserve(team.id().clone(), places)?;
            let dto = SlotDTO::from(best_slot.clone());
            team.reserve(best_slot.id().clone())?;
            self.slot_repository.save_slot(best_slot).await?;
            self.team_repository.save_team(team).await?;
            Ok(dto)
        } else {
            Err(AppError::NoAvailableSlots(start, places))
        }
    }
}
