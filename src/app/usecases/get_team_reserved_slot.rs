use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{SlotProvider, TeamByMemberProvider};
use crate::app::usecases::dto::SlotDTO;
use crate::domain::models::UserID;

pub struct GetTeamReservedSlot {
    team_provider: Arc<dyn TeamByMemberProvider>,
    slot_provider: Arc<dyn SlotProvider>,
}

impl GetTeamReservedSlot {
    pub fn new(
        team_provider: Arc<dyn TeamByMemberProvider>,
        slot_provider: Arc<dyn SlotProvider>,
    ) -> Self {
        Self {
            team_provider,
            slot_provider,
        }
    }

    pub async fn execute(&self, user_id: UserID) -> Result<Option<SlotDTO>, AppError> {
        let team_opt = self.team_provider.team_by_member(user_id).await?;
        if let Some(team) = team_opt {
            if let Some(slot_id) = team.reserved_slot() {
                let slot = self.slot_provider.slot(slot_id).await?;
                Ok(Some(SlotDTO::from(slot)))
            } else {
                Ok(None)
            }
        } else {
            Err(AppError::UserNotInTeam(user_id))
        }
    }
}
