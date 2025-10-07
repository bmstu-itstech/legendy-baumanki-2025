use crate::GetPlayer;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{InputFile, Message, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::dto::{CharacterDTO, PlayerDTO, TeamWithMembersDTO};
use crate::app::usecases::{
    GetAvailableTracks, GetCharacter, GetCharacterNames, GetTeamWithMembers, GetUserTeam,
    GiveFeedback,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::shared::{send_enter_message, send_use_keyboard};
use crate::bot::handlers::slots::{prompt_accept_final, prompt_cancel_reservation_reason};
use crate::bot::handlers::tracks::prompt_track;
use crate::bot::keyboards::{
    make_back_keyboard, make_characters_keyboard_with_back, make_menu_keyboard,
};
use crate::bot::{BotHandlerResult, keyboards, texts};
use crate::domain::models::{CharacterName, FeedbackText, UserID};

pub async fn prompt_menu(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    player: &PlayerDTO,
) -> BotHandlerResult {
    let markup = make_menu_keyboard(player);
    bot.send_message(msg.chat.id, texts::MENU_TEXT)
        .reply_markup(markup)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::MenuOption).await?;
    Ok(())
}

async fn receive_menu_option(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_player: GetPlayer,
    get_user_team: GetUserTeam,
    get_team_with_members: GetTeamWithMembers,
    get_character_names: GetCharacterNames,
    get_available_tracks: GetAvailableTracks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match text {
            /*keyboards::BTN_MY_TEAM => {
                if let Some(team) = get_user_team.execute(user_id).await? {
                    let team = get_team_with_members.execute(team.id).await?;
                    if !team.solo {
                        send_my_team(&bot, &msg, team).await?;
                    }
                    let player = get_player.execute(user_id).await?;
                    prompt_menu(bot, msg, dialogue, &player).await?
                }
            }
            keyboards::BTN_TRACKS => {
                let tracks = get_available_tracks.execute(user_id).await?;
                prompt_track(bot, msg, dialogue, &tracks).await?
            }
            keyboards::BTN_CHARACTERS => {
                let names = get_character_names.execute().await?;
                prompt_character_name(bot, msg, dialogue, &names).await?
            }
            keyboards::BTN_RESERVE_SLOT => {
                let player = get_player.execute(user_id).await?;
                if player.is_captain {
                    prompt_accept_final(bot, msg, dialogue).await?
                } else {
                    send_unknown_menu_option(&bot, &msg).await?;
                    let player = get_player.execute(user_id).await?;
                    prompt_menu(bot, msg, dialogue, &player).await?
                }
            }
            keyboards::BTN_CANCEL_RESERVATION => {
                prompt_cancel_reservation_reason(bot, msg, dialogue).await?
            }*/
            keyboards::BTN_GIVE_FEEDBACK => prompt_feedback(bot, msg, dialogue).await?,
            _ => {
                send_unknown_menu_option(&bot, &msg).await?;
                let player = get_player.execute(user_id).await?;
                prompt_menu(bot, msg, dialogue, &player).await?
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

async fn send_my_team(bot: &Bot, msg: &Message, team: TeamWithMembersDTO) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::my_team(team))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_character_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    names: &[CharacterName],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_CHARACTER_NAME)
        .reply_markup(make_characters_keyboard_with_back(names))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::CharacterName).await?;
    Ok(())
}

async fn receive_character_name(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_player: GetPlayer,
    get_character: GetCharacter,
    get_character_names: GetCharacterNames,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(text) => {
            let name = CharacterName::new(text.to_string())?;
            match get_character.execute(&name).await {
                Err(AppError::CharacterNotFound(_)) => send_use_keyboard(&bot, &msg).await,
                Err(err) => Err(err),
                Ok(character) => {
                    let names = get_character_names.execute().await?;
                    send_character(&bot, &msg, character, &names).await
                }
            }
        }
    }
}

async fn send_character(
    bot: &Bot,
    msg: &Message,
    character: CharacterDTO,
    names: &[CharacterName],
) -> BotHandlerResult {
    bot.send_photo(
        msg.chat.id,
        InputFile::file_id(character.image_id.clone().into()),
    )
    .caption(texts::character(character))
    .reply_markup(make_characters_keyboard_with_back(names))
    .parse_mode(ParseMode::Html)
    .await?;
    Ok(())
}

async fn prompt_feedback(bot: Bot, msg: Message, dialogue: BotDialogue) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_FEEDBACK)
        .reply_markup(make_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::Feedback).await?;
    Ok(())
}

async fn receive_feedback(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    give_feedback: GiveFeedback,
    get_player: GetPlayer,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(text) => {
            let text = FeedbackText::new(text.to_string())?;
            give_feedback.execute(user_id, text).await?;
            send_feedback_sent(&bot, &msg).await?;
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
    }
}

async fn send_feedback_sent(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::FEEDBACK_SENT)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn menu_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    Update::filter_message()
        .branch(case![BotState::MenuOption].endpoint(receive_menu_option))
        .branch(case![BotState::CharacterName].endpoint(receive_character_name))
        .branch(case![BotState::Feedback].endpoint(receive_feedback))
}
