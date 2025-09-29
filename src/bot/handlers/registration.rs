use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{KeyboardRemove, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::{GetUser, JoinTeam, RegisterUser};
use crate::bot::BotHandlerResult;
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::menu::{
    prompt_menu, send_joining_team_successful, send_team_is_full, send_team_not_exists,
};
use crate::bot::keyboards::{BTN_AGREEMENT, make_agreement_keyboard};
use crate::bot::texts;
use crate::domain::models::{FullName, GroupName, TeamID, UserID, Username};

use crate::bot::handlers::shared::{send_enter_message, send_internal_error};
use crate::domain::error::DomainError;

pub async fn prompt_pd_agreement(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    team_id_opt: Option<TeamID>,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_PD_AGREEMENT)
        .reply_markup(make_agreement_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::PDAgreement(team_id_opt)).await?;
    Ok(())
}

async fn receive_pd_agreement(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    team_id_opt: Option<TeamID>,
) -> BotHandlerResult {
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => {
            if text != BTN_AGREEMENT {
                send_pd_agreement_is_required(&bot, &msg).await?;
            } else {
                prompt_full_name(bot, msg, dialogue, team_id_opt).await?;
            }
        }
    }
    Ok(())
}

async fn send_pd_agreement_is_required(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PD_AGREEMENT_IS_REQUIRED)
        .reply_markup(make_agreement_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_full_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    team_id_opt: Option<TeamID>,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_FULL_NAME)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::FullName(team_id_opt)).await?;
    Ok(())
}

async fn receive_full_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    team_id_opt: Option<TeamID>,
) -> BotHandlerResult {
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match FullName::new(text.to_string()) {
            Err(_) => send_full_name_is_invalid(&bot, &msg).await?,
            Ok(full_name) => {
                prompt_group_name(bot, msg, dialogue, (team_id_opt, full_name)).await?
            }
        },
    }
    Ok(())
}

async fn send_full_name_is_invalid(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_FULL_NAME)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_group_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    (team_id_opt, full_name): (Option<TeamID>, FullName),
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::ENTER_GROUP_NAME)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue
        .update(BotState::GroupName(team_id_opt, full_name))
        .await?;
    Ok(())
}

async fn receive_group_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    (team_id_opt, full_name): (Option<TeamID>, FullName),
    register_user: RegisterUser,
    join_team: JoinTeam,
    get_user: GetUser,
) -> BotHandlerResult {
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match GroupName::new(text.to_string()) {
            Err(_) => send_group_name_is_invalid(&bot, &msg).await?,
            Ok(group_name) => {
                if !group_name.is_first_course() {
                    send_only_for_first_course(bot, msg, dialogue).await?;
                } else {
                    let user_id = UserID::new(msg.chat.id.0);
                    let username = username_from_message(&msg);
                    register_user
                        .execute(user_id, username, full_name, group_name)
                        .await?;
                    send_registration_successful(&bot, &msg).await?;
                    if let Some(team_id) = team_id_opt.clone() {
                        match join_team.execute(user_id, team_id).await {
                            Err(AppError::DomainError(DomainError::TeamIsFull(_))) => {
                                send_team_is_full(&bot, &msg).await?;
                                let user = get_user.execute(user_id).await?;
                                prompt_menu(bot, msg, dialogue, &user).await?;
                            }
                            Err(AppError::TeamNotFound(_)) => {
                                send_team_not_exists(&bot, &msg).await?;
                                let user = get_user.execute(user_id).await?;
                                prompt_menu(bot, msg, dialogue, &user).await?;
                            }
                            Err(err) => {
                                log::error!("failed to join team: {:?}", err);
                                send_internal_error(&bot, &msg).await?;
                                return Err(err);
                            }
                            Ok(team) => {
                                send_joining_team_successful(&bot, &msg, team.name.clone()).await?;
                                let user = get_user.execute(user_id).await?;
                                prompt_menu(bot, msg, dialogue, &user).await?;
                            }
                        }
                    } else {
                        let user = get_user.execute(user_id).await?;
                        prompt_menu(bot, msg, dialogue, &user).await?;
                    }
                }
            }
        },
    }
    Ok(())
}

fn username_from_message(msg: &Message) -> Option<Username> {
    msg.clone() // Может быть как-то выйдет сделать лучше?..
        .from
        .and_then(|from| from.username)
        .map(|u| Username::new(u).unwrap()) // Верим, что telegram не вернёт пустой Username
}

async fn send_group_name_is_invalid(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_GROUP_NAME)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_only_for_first_course(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::NOT_FIRST_COURSE)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn send_registration_successful(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::SUCCESSFUL_REGISTRATION)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn registration_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    Update::filter_message()
        .branch(case![BotState::PDAgreement(team_id_opt)].endpoint(receive_pd_agreement))
        .branch(case![BotState::FullName(team_id_opt)].endpoint(receive_full_name))
        .branch(case![BotState::GroupName(team_id_opt, full_name)].endpoint(receive_group_name))
}
