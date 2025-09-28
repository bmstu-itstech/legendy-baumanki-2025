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
use crate::bot::handlers::registration::registration_scheme;

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
                app.check_registered,
                app.create_team,
                app.exit_team,
                app.get_character,
                app.get_character_names,
                app.get_media,
                app.get_profile,
                app.get_task,
                app.get_team_with_members,
                app.get_user,
                app.get_user_tasks,
                app.get_user_task,
                app.get_user_team,
                app.join_team,
                app.register_user,
                app.switch_to_solo_mode,
                app.switch_to_want_team_mode,
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
            .branch(registration_scheme())
            .branch(menu_scheme())
    }
}
