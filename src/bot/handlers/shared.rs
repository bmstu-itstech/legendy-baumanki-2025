use teloxide::prelude::*;
use teloxide::types::{KeyboardRemove, ParseMode};

use crate::bot::{BotHandlerResult, texts};

pub async fn send_enter_message(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::ENTER_MESSAGE_TEXT)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_internal_error(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INTERNAL_ERROR)
        .reply_markup(KeyboardRemove::new())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_use_keyboard(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::USE_KEYBOARD)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
