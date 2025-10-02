use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{PostgresStorage, enter};
use teloxide::dispatching::{DefaultKey, UpdateHandler};
use teloxide::prelude::*;

use crate::app::error::AppError;
use crate::app::usecases::app::App;
use crate::bot::fsm::BotState;
use crate::bot::handlers::commands::commands_scheme;
use crate::bot::handlers::menu::menu_scheme;
use crate::bot::handlers::tracks::tracks_scheme;

pub struct BotDispatcher;

impl BotDispatcher {
    pub async fn create(
        bot: Bot,
        app: App,
        postgres_storage: Arc<PostgresStorage<Json>>,
    ) -> Dispatcher<Bot, AppError, DefaultKey> {
        Dispatcher::builder(bot, Self::scheme())
            .dependencies(dptree::deps![
                app.answer_task,
                app.check_admin,
                app.check_captain,
                app.check_registered,
                app.check_started_track,
                app.get_available_tasks,
                app.get_character,
                app.get_character_names,
                app.get_completed_tasks,
                app.get_media,
                app.get_player,
                app.get_profile,
                app.get_task,
                app.get_available_tracks,
                app.get_team_with_members,
                app.get_track_in_progress,
                app.get_user,
                app.get_user_team,
                app.give_feedback,
                app.start_track,
                app.upload_media,
                postgres_storage
            ])
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .enable_ctrlc_handler()
            .build()
    }

    fn scheme() -> UpdateHandler<AppError> {
        enter::<Update, PostgresStorage<Json>, BotState, _>()
            .branch(commands_scheme())
            .branch(menu_scheme())
            .branch(tracks_scheme())
    }
}
