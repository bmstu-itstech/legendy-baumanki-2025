use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::PostgresStorage;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::prelude::Dialogue;

use crate::domain::models::{FullName, MediaID, TaskID, TeamID, TrackTag};

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum BotState {
    #[default]
    Idle,

    // Menu
    MenuOption,
    CharacterName,
    Feedback,

    // Tracks
    Track,
    StartTrack(TrackTag),
    TrackTaskGroup(TrackTag),
    AvailableTask(TrackTag),
    CompletedTask(TrackTag),
    TaskAnswer(TrackTag, TaskID),

    // Admin
    Media(MediaID),
}

pub type BotDialogue = Dialogue<BotState, PostgresStorage<Json>>;
