use teloxide::dispatching::UpdateHandler;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::{CheckRegistered, GetUserTeam, JoinTeam};
use crate::bot::fsm::BotDialogue;
use crate::bot::handlers::menu::{
    prompt_menu, send_joining_team_successful, send_team_is_full, send_team_not_exists,
};
use crate::bot::handlers::registration::prompt_pd_agreement;
use crate::bot::{BotHandlerResult, texts};
use crate::bot::resources::{RULES_IMAGE_1, RULES_IMAGE_2};
use crate::domain::error::DomainError;
use crate::domain::models::{TeamID, UserID};

#[derive(BotCommands, Clone)]
#[command(description = "Команды регистрации")]
enum BotCommand {
    #[command(rename = "start", description = "начать регистрацию")]
    Start(String),

    #[command(rename = "cancel", description = "отменить текущую операцию")]
    Cancel,
}

async fn handle_start_command(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    command: BotCommand,
    check_registered: CheckRegistered,
    get_user_team: GetUserTeam,
    join_team: JoinTeam,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    let registered = check_registered.is_registered(user_id).await?;
    let team_id_opt = if let BotCommand::Start(code) = command {
        if code == "" {
            None
        } else {
            TeamID::try_from(code).ok()
        }
    } else {
        None
    };

    match (registered, team_id_opt) {
        // Не зарегистрирован и нет кода команды
        (false, None) => {
            send_greeting_message(&bot, &msg).await?;
            prompt_pd_agreement(bot, msg, dialogue, None).await?;
        }
        // Не зарегистрирован и есть код команды в ссылке
        (false, Some(team_id)) => {
            send_greeting_message(&bot, &msg).await?;
            prompt_pd_agreement(bot, msg, dialogue, Some(team_id)).await?;
        }
        // Зарегистрирован и нет кода команды
        (true, None) => {
            let has_team = get_user_team.user_team(user_id).await?.is_some();
            prompt_menu(bot, msg, dialogue, has_team).await?;
        }
        // Зарегистрирован и есть код команды
        (true, Some(team_id)) => {
            match get_user_team.user_team(user_id).await? {
                Some(user_team) => {
                    if user_team.id == team_id {
                        send_already_in_this_team(&bot, &msg).await?;
                    } else {
                        send_already_in_other_team(&bot, &msg).await?;
                    }
                }
                None => match join_team.join_team(user_id, team_id).await {
                    Err(AppError::DomainError(DomainError::TeamIsFull(_))) => {
                        send_team_is_full(&bot, &msg).await?;
                    }
                    Err(AppError::DomainError(DomainError::TeamNotFound(_))) => {
                        send_team_not_exists(&bot, &msg).await?;
                    }
                    Err(err) => return Err(err),
                    Ok(team) => send_joining_team_successful(&bot, &msg, team.name).await?,
                },
            }
            prompt_menu(bot, msg, dialogue, true).await?;
        }
    }
    Ok(())
}

async fn send_greeting_message(bot: &Bot, msg: &Message) -> BotHandlerResult {
    let media_group = vec![
        InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id(RULES_IMAGE_1.into()))),
        InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id(RULES_IMAGE_2.into())))
    ];
    
    bot.send_media_group(msg.chat.id, media_group).await?;

    bot.send_message(msg.chat.id, texts::GREETING_MSG)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_already_in_this_team(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::ALREADY_IN_THIS_TEAM)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_already_in_other_team(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::ALREADY_IN_OTHER_TEAM)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn commands_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<BotCommand, _>()
        .branch(case![BotCommand::Start(payload)].endpoint(handle_start_command));

    let message_handler = Update::filter_message().branch(command_handler);

    message_handler
}
