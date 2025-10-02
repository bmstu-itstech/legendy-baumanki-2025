use std::env;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{PostgresStorage, serializer};
use teloxide::prelude::*;

use crate::app::usecases::app::App;
use crate::app::usecases::{AnswerTask, CheckAdmin, CheckRegistered, CheckStartedTrack, GetAvailableTasks, GetCharacter, GetCharacterNames, GetMedia, GetProfile, GetTask, GetAvailableTracks, GetTeamWithMembers, GetTrackInProgress, GetUser, GetUserTeam, GiveFeedback, StartTrack, UploadMedia, GetCompletedTasks, CheckCaptain};
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
        answer_task: AnswerTask::new(repos.clone(), repos.clone(), repos.clone(), repos.clone()),
        check_admin: CheckAdmin::new(repos.clone()),
        check_captain: CheckCaptain::new(repos.clone()),
        check_registered: CheckRegistered::new(repos.clone()),
        check_started_track: CheckStartedTrack::new(repos.clone()),
        get_available_tasks: GetAvailableTasks::new(repos.clone(), repos.clone()),
        get_character: GetCharacter::new(repos.clone(), repos.clone()),
        get_character_names: GetCharacterNames::new(repos.clone()),
        get_completed_tasks: GetCompletedTasks::new(repos.clone(), repos.clone()),
        get_media: GetMedia::new(repos.clone()),
        get_profile: GetProfile::new(repos.clone(), repos.clone()),
        get_task: GetTask::new(repos.clone(), repos.clone()),
        get_available_tracks: GetAvailableTracks::new(repos.clone()),
        get_team_with_members: GetTeamWithMembers::new(repos.clone(), repos.clone()),
        get_track_in_progress: GetTrackInProgress::new(repos.clone(), repos.clone(), repos.clone()),
        get_user: GetUser::new(repos.clone()),
        get_user_team: GetUserTeam::new(repos.clone()),
        give_feedback: GiveFeedback::new(repos.clone()),
        start_track: StartTrack::new(repos.clone(), repos.clone(), repos.clone(), repos.clone()),
        upload_media: UploadMedia::new(repos.clone()),
    };

    let bot = Bot::from_env();
    let mut dispatcher = BotDispatcher::create(bot, app, state_storage).await;

    dispatcher.dispatch().await;
}
