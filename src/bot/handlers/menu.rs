use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{Me, Message, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::dto::{Profile, TeamDTO, TeamWithMembersDTO};
use crate::app::usecases::{
    CreateTeam, ExitTeam, GetProfile, GetTeamWithMembers, GetUserTeam, JoinTeam,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::shared::{send_enter_message, send_use_keyboard};
use crate::bot::keyboards::{
    make_menu_keyboard_with_team, make_menu_keyboard_without_team,
    make_yes_and_back_keyboard,
};
use crate::bot::{BotHandlerResult, keyboards, texts};
use crate::domain::error::DomainError;
use crate::domain::models::{TeamID, TeamName, UserID};

pub async fn prompt_menu(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    has_team: bool,
) -> BotHandlerResult {
    let markup = if has_team {
        make_menu_keyboard_with_team()
    } else {
        make_menu_keyboard_without_team()
    };
    bot.send_message(msg.chat.id, texts::MENU_TEXT)
        .reply_markup(markup)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::MenuOption).await?;
    Ok(())
}

async fn receive_menu_option(
    bot: Bot,
    me: Me,
    msg: Message,
    dialogue: BotDialogue,
    get_user_team: GetUserTeam,
    get_team_with_members: GetTeamWithMembers,
    get_profile: GetProfile,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match text {
            keyboards::BTN_PROFILE => {
                let profile = get_profile.profile(user_id).await?;
                let has_team = profile.team_name.is_some();
                send_profile(&bot, &msg, profile).await?;
                prompt_menu(bot, msg, dialogue, has_team).await?;
            }
            keyboards::BTN_JOIN_TEAM => prompt_team_code(bot, msg, dialogue).await?,
            keyboards::BTN_CREATE_TEAM => prompt_team_name(bot, msg, dialogue).await?,
            keyboards::BTN_MY_TEAM => {
                if let Some(team) = get_user_team.user_team(user_id).await? {
                    let team = get_team_with_members.team_with_members(team.id).await?;
                    send_my_team(&bot, &me, &msg, team).await?;
                    prompt_menu(bot, msg, dialogue, true).await?;
                }
            }
            keyboards::BTN_EXIT_TEAM => prompt_exit_approval(bot, msg, dialogue).await?,
            _ => {
                send_unknown_menu_option(&bot, &msg).await?;
                let has_team = get_user_team.user_team(user_id).await?.is_some();
                prompt_menu(bot, msg, dialogue, has_team).await?
            }
        },
    }
    Ok(())
}

async fn send_unknown_menu_option(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::UNKNOWN_MENU_OPTION)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_team_code(bot: Bot, msg: Message, dialogue: BotDialogue) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TEAM_CODE)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TeamCode).await?;
    Ok(())
}

async fn prompt_team_name(bot: Bot, msg: Message, dialogue: BotDialogue) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TEAM_NAME)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TeamName).await?;
    Ok(())
}

async fn send_my_team(
    bot: &Bot,
    me: &Me,
    msg: &Message,
    team: TeamWithMembersDTO,
) -> BotHandlerResult {
    let link = team_invite_link(me.tme_url().as_str(), (&team).id.as_str());
    bot.send_message(msg.chat.id, texts::my_team(team, &link))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_profile(bot: &Bot, msg: &Message, profile: Profile) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::profile(profile))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_exit_approval(bot: Bot, msg: Message, dialogue: BotDialogue) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::SEND_APPROVAL_EXIT_TEAM)
        .reply_markup(make_yes_and_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::ExitApproval).await?;
    Ok(())
}

async fn receive_team_code(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    join_team: JoinTeam,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match TeamID::try_from(text.to_string()) {
            Err(_) => send_invalid_team_code(&bot, &msg).await?,
            Ok(team_id) => match join_team.join_team(user_id, team_id).await {
                Err(AppError::DomainError(DomainError::TeamNotFound(_))) => {
                    send_team_not_found(&bot, &msg).await?;
                    prompt_menu(bot, msg, dialogue, false).await?;
                }
                Err(AppError::DomainError(DomainError::TeamIsFull(_))) => {
                    send_team_is_full(&bot, &msg).await?;
                    prompt_menu(bot, msg, dialogue, false).await?;
                }
                Err(err) => return Err(err),
                Ok(team) => {
                    send_joining_team_successful(&bot, &msg, team.name).await?;
                    prompt_menu(bot, msg, dialogue, true).await?;
                }
            },
        },
    }
    Ok(())
}

async fn send_invalid_team_code(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_INVITE_CODE)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_team_not_found(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::TEAM_NOT_FOUND)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_team_not_exists(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::TEAM_NOT_EXISTS)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_joining_team_successful(
    bot: &Bot,
    msg: &Message,
    team_name: TeamName,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::successful_joined_team(team_name))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_team_is_full(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::TEAM_IS_FULL)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn receive_team_name(
    bot: Bot,
    me: Me,
    msg: Message,
    dialogue: BotDialogue,
    create_team: CreateTeam,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match TeamName::new(text.to_string()) {
            Err(_) => send_team_name_is_invalid(&bot, &msg).await?,
            Ok(team_name) => {
                let team = create_team.create_team(team_name, user_id).await?;
                send_team_successful_created(&bot, &me, &msg, team).await?;
                prompt_menu(bot, msg, dialogue, true).await?;
            }
        },
    }
    Ok(())
}

fn team_invite_link(base: &str, team_id: &str) -> String {
    format!("{base}?start={team_id}")
}

async fn send_team_successful_created(
    bot: &Bot,
    me: &Me,
    msg: &Message,
    team: TeamDTO,
) -> BotHandlerResult {
    let link = team_invite_link(me.tme_url().as_str(), (&team).id.as_str());
    bot.send_message(msg.chat.id, texts::team_created(team, &link))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_team_name_is_invalid(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_TEAM_NAME)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn receive_exit_approval(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    exit_team: ExitTeam,
) -> BotHandlerResult {
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => prompt_menu(bot, msg, dialogue, true).await?,
        Some(keyboards::BTN_YES) => {
            let user_id = UserID::new(msg.chat.id.0);
            exit_team.exit(user_id).await?;
            send_successfully_exited_team(&bot, &msg).await?;
            prompt_menu(bot, msg, dialogue, false).await?;
        }
        Some(_) => {
            send_use_keyboard(&bot, &msg).await?;
            prompt_exit_approval(bot, msg, dialogue).await?;
        }
    }
    Ok(())
}

async fn send_successfully_exited_team(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::SUCCESSFUL_EXIT_TEAM)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn menu_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    Update::filter_message()
        .branch(case![BotState::MenuOption].endpoint(receive_menu_option))
        .branch(case![BotState::TeamCode].endpoint(receive_team_code))
        .branch(case![BotState::TeamName].endpoint(receive_team_name))
        .branch(case![BotState::ExitApproval].endpoint(receive_exit_approval))
}
