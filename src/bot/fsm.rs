use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

use crate::domain::models::{FullName, MediaID, TeamID};

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum BotState {
    #[default]
    Idle,

    // Registration
    PDAgreement(Option<TeamID>),
    FullName(Option<TeamID>),
    GroupName(Option<TeamID>, FullName),

    // Menu
    MenuOption,
    TeamCode,
    TeamName,
    ExitApproval,

    // Admin
    Media(MediaID),
}

pub type BotDialogue = Dialogue<BotState, InMemStorage<BotState>>;
