use chrono::NaiveTime;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

use crate::app::error::AppError;
use crate::app::usecases::dto::SlotDTO;
use crate::app::usecases::{
    CancelReservation, GetAvailableSlotStarts, GetPlayer, GetUserTeam, ReserveSlot,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::menu::prompt_menu;
use crate::bot::handlers::shared::{send_enter_message, send_use_keyboard};
use crate::bot::keyboards::{
    make_back_keyboard, make_cancel_reservation_keyboard_with_back,
    make_slot_start_keyboard_with_back, make_start_and_back_keyboard,
};
use crate::bot::{BotHandlerResult, keyboards, texts};
use crate::domain::models::UserID;

pub async fn prompt_accept_final(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_ACCEPT_FINAL)
        .reply_markup(make_start_and_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::AcceptFinal).await?;
    Ok(())
}

async fn receive_accept_final(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_player: GetPlayer,
    get_available_slot_starts: GetAvailableSlotStarts,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(keyboards::BTN_START) => {
            let starts = get_available_slot_starts.execute().await?;
            prompt_slot_start(bot, msg, dialogue, &starts).await
        }
        Some(_) => send_use_keyboard(&bot, &msg).await,
    }
}

async fn prompt_slot_start(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    starts: &[NaiveTime],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_SLOT_TIME)
        .reply_markup(make_slot_start_keyboard_with_back(starts))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::SlotStart).await?;
    Ok(())
}

async fn receive_slot_start(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_player: GetPlayer,
    get_user_team: GetUserTeam,
    get_available_slot_starts: GetAvailableSlotStarts,
    reserve_slot: ReserveSlot,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(text) => match NaiveTime::parse_from_str(text, "%H:%M") {
            Ok(start) => {
                let team = get_user_team
                    .execute(user_id)
                    .await?
                    .ok_or(AppError::UserNotInTeam(user_id))?;
                if team.size > 1 {
                    prompt_slot_places(bot, msg, dialogue, start, team.size).await
                } else {
                    match reserve_slot.execute(user_id, start, 1).await {
                        Ok(slot) => {
                            send_slot_successfully_reserved(&bot, &msg, slot).await?;
                            let player = get_player.execute(user_id).await?;
                            prompt_menu(bot, msg, dialogue, &player).await
                        }
                        Err(AppError::PlacesGreaterThanTeamSize(_, team_size)) => {
                            send_places_greater_than_team_size(&bot, &msg, team_size).await
                        }
                        Err(AppError::NoAvailableSlots(_, team_size)) => {
                            send_no_slots(&bot, &msg, team_size).await?;
                            let slots = get_available_slot_starts.execute().await?;
                            prompt_slot_start(bot, msg, dialogue, &slots).await
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(_) => {
                send_use_keyboard(&bot, &msg).await?;
                let starts = get_available_slot_starts.execute().await?;
                prompt_slot_start(bot, msg, dialogue, &starts).await
            }
        },
    }
}

async fn prompt_slot_places(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    start: NaiveTime,
    team_size: usize,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::prompt_slot_places(team_size))
        .reply_markup(make_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::SlotPlaces(start)).await?;
    Ok(())
}

async fn receive_slot_places(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    start: NaiveTime,
    get_available_slot_starts: GetAvailableSlotStarts,
    reserve_slot: ReserveSlot,
    get_player: GetPlayer,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let starts = get_available_slot_starts.execute().await?;
            prompt_slot_start(bot, msg, dialogue, &starts).await
        }
        Some(text) => match text.parse::<usize>() {
            Ok(places) => match reserve_slot.execute(user_id, start, places).await {
                Ok(slot) => {
                    send_slot_successfully_reserved(&bot, &msg, slot).await?;
                    let player = get_player.execute(user_id).await?;
                    prompt_menu(bot, msg, dialogue, &player).await
                }
                Err(AppError::PlacesGreaterThanTeamSize(_, team_size)) => {
                    send_places_greater_than_team_size(&bot, &msg, team_size).await
                }
                Err(AppError::NoAvailableSlots(_, team_size)) => {
                    send_no_slots(&bot, &msg, team_size).await?;
                    let slots = get_available_slot_starts.execute().await?;
                    prompt_slot_start(bot, msg, dialogue, &slots).await
                }
                Err(err) => Err(err),
            },
            Err(_) => send_invalid_slot_places(&bot, &msg).await,
        },
    }
}

async fn send_slot_successfully_reserved(
    bot: &Bot,
    msg: &Message,
    slot: SlotDTO,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::slot_successfully_reserved(slot))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_invalid_slot_places(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_TEAM_PLACES)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_places_greater_than_team_size(
    bot: &Bot,
    msg: &Message,
    team_size: usize,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::places_greater_than_team(team_size))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_no_slots(bot: &Bot, msg: &Message, team_size: usize) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::no_slots(team_size))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn prompt_cancel_reservation_reason(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_CANCEL_RESERVATION_REASON)
        .reply_markup(make_cancel_reservation_keyboard_with_back())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::CancelReason).await?;
    Ok(())
}

async fn receive_cancel_reservation_reason(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_player: GetPlayer,
    cancel_reservation: CancelReservation,
    get_available_slot_starts: GetAvailableSlotStarts,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(keyboards::BTN_CAN_NOT_ACCEPT_FINAL) => {
            send_ochen_zhal(&bot, &msg).await?;
            cancel_reservation.execute(user_id).await?;
            let player = get_player.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &player).await
        }
        Some(keyboards::BTN_CHANGE_RESERVATION_TIME) => {
            cancel_reservation.execute(user_id).await?;
            let starts = get_available_slot_starts.execute().await?;
            prompt_slot_start(bot, msg, dialogue, &starts).await
        }
        Some(_) => send_use_keyboard(&bot, &msg).await,
    }
}

async fn send_ochen_zhal(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::OCHEN_ZHAL)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn slots_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    Update::filter_message()
        .branch(case![BotState::AcceptFinal].endpoint(receive_accept_final))
        .branch(case![BotState::SlotStart].endpoint(receive_slot_start))
        .branch(case![BotState::SlotPlaces(start)].endpoint(receive_slot_places))
        .branch(case![BotState::CancelReason].endpoint(receive_cancel_reservation_reason))
}
