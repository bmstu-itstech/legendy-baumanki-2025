use teloxide::prelude::*;
use teloxide::types::{FileId as TelegramFileId, InputFile, KeyboardMarkup, KeyboardRemove, ParseMode};

use crate::bot::{BotHandlerResult, texts};
use crate::domain::models::{FileID, Media, MediaType};

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

pub async fn send_permission_denied(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PERMISSION_DENIED)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_media_with_caption(
    bot: &Bot,
    msg: &Message,
    media: Media,
    caption: &str,
) -> BotHandlerResult {
    match media.media_type() {
        MediaType::Image => {
            bot.send_photo(msg.chat.id, media.into())
                .caption(caption)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        MediaType::VideoNote => {
            bot.send_video_note(msg.chat.id, media.into()).await?;
            bot.send_message(msg.chat.id, caption)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}

impl Into<TelegramFileId> for FileID {
    fn into(self) -> TelegramFileId {
        TelegramFileId::from(self.to_string())
    }
}

impl Into<InputFile> for Media {
    fn into(self) -> InputFile {
        InputFile::file_id(self.file_id().clone().into())
    }
}
