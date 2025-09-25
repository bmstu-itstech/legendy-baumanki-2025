use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::PostgresStorage;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::prelude::Dialogue;

use crate::domain::models::{FullName, MediaID, TaskID, TeamID};

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
    Rebus,
    RebusAnswer(TaskID),
    Riddle,
    RiddleAnswer(TaskID),
    CharacterName,

    // Admin
    Media(MediaID),
}

pub type BotDialogue = Dialogue<BotState, PostgresStorage<Json>>;
