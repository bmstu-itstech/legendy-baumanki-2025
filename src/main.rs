use std::env;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{PostgresStorage, serializer};
use teloxide::prelude::*;

use crate::app::usecases::app::App;
use crate::app::usecases::{
    AnswerTask, CheckAdmin, CheckRegistered, CreateTeam, ExitTeam, GetCharacter, GetCharacterNames,
    GetMedia, GetProfile, GetTask, GetTeamWithMembers, GetUser, GetUserTask, GetUserTasks,
    GetUserTeam, GiveFeedback, JoinTeam, RegisterUser, SwitchToLookingForTeam, SwitchToSoloMode,
    UploadMedia,
};
use crate::bot::dispatcher::BotDispatcher;
use crate::infra::postgres::PostgresRepository;
use crate::utils::postgres::pool;

mod app;
mod bot;
mod domain;
mod infra;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set");
    let pool =
        pool::connect(&uri).expect(format!("unable to connect to database: {}", uri).as_str());
    log::info!("Connected to PostgreSQL database: {}", uri);

    let repos = Arc::new(PostgresRepository::new(pool));
    let state_storage = PostgresStorage::open(&uri, 1, serializer::Json)
        .await
        .expect("unable to create PostgreSQL state storage");

    let app = App {
        answer_task: AnswerTask::new(repos.clone(), repos.clone()),
        check_admin: CheckAdmin::new(repos.clone()),
        check_registered: CheckRegistered::new(repos.clone()),
        create_team: CreateTeam::new(repos.clone(), repos.clone()),
        exit_team: ExitTeam::new(repos.clone(), repos.clone(), repos.clone()),
        get_character: GetCharacter::new(repos.clone(), repos.clone()),
        get_character_names: GetCharacterNames::new(repos.clone()),
        get_media: GetMedia::new(repos.clone()),
        get_profile: GetProfile::new(repos.clone(), repos.clone()),
        get_task: GetTask::new(repos.clone()),
        get_team_with_members: GetTeamWithMembers::new(repos.clone(), repos.clone()),
        get_user: GetUser::new(repos.clone()),
        get_user_tasks: GetUserTasks::new(repos.clone(), repos.clone()),
        get_user_task: GetUserTask::new(repos.clone(), repos.clone()),
        get_user_team: GetUserTeam::new(repos.clone()),
        give_feedback: GiveFeedback::new(repos.clone()),
        join_team: JoinTeam::new(repos.clone(), repos.clone()),
        register_user: RegisterUser::new(repos.clone()),
        switch_to_solo_mode: SwitchToSoloMode::new(repos.clone()),
        switch_to_want_team_mode: SwitchToLookingForTeam::new(repos.clone()),
        upload_media: UploadMedia::new(repos.clone()),
    };

    let bot = Bot::from_env();
    let mut dispatcher = BotDispatcher::create(bot, app, state_storage).await;

    dispatcher.dispatch().await;
}
