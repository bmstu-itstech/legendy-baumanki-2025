use std::env;
use std::sync::Arc;
use teloxide::prelude::*;

use crate::app::usecases::app::App;
use crate::app::usecases::{
    ChangeFullName, ChangeGroupName, CheckAdmin, CheckRegistered, CheckTeamExists, CreateTeam,
    ExitTeam, GetMedia, GetProfile, GetTeamWithMembers, GetUserTeam, JoinTeam, RegisterUser,
    RemoveMember, UploadMedia,
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

    let app = App {
        change_full_name: ChangeFullName::new(repos.clone()),
        change_group_name: ChangeGroupName::new(repos.clone()),
        check_admin: CheckAdmin::new(repos.clone()),
        check_registered: CheckRegistered::new(repos.clone()),
        check_team_exists: CheckTeamExists::new(repos.clone()),
        create_team: CreateTeam::new(repos.clone()),
        exit_team: ExitTeam::new(repos.clone(), repos.clone()),
        get_media: GetMedia::new(repos.clone()),
        get_profile: GetProfile::new(repos.clone(), repos.clone()),
        get_team_with_members: GetTeamWithMembers::new(repos.clone(), repos.clone()),
        get_user_team: GetUserTeam::new(repos.clone()),
        join_team: JoinTeam::new(repos.clone()),
        register_user: RegisterUser::new(repos.clone()),
        remove_member: RemoveMember::new(repos.clone()),
        upload_media: UploadMedia::new(repos.clone()),
    };

    let bot = Bot::from_env();
    let mut dispatcher = BotDispatcher::create(bot, app).await;

    dispatcher.dispatch().await;
}
