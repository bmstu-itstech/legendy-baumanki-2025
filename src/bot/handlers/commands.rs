use teloxide::dispatching::UpdateHandler;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

use crate::app::error::AppError;
use crate::app::usecases::{
    CheckAdmin, CheckRegistered, GetMedia, GetPlayer, GetUser, UploadMedia,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::menu::prompt_menu;
use crate::bot::handlers::shared::{send_media_with_caption, send_permission_denied};
use crate::bot::{BotHandlerResult, texts};
use crate::domain::models::{FileID, Media, MediaID, UserID};

#[derive(BotCommands, Clone)]
#[command(description = "Команды регистрации")]
enum BotCommand {
    #[command(rename = "start", description = "начать регистрацию")]
    Start(String),

    #[command(rename = "upload", description = "загрузить медиа")]
    Upload(String),

    #[command(rename = "media", description = "получить медиафайл")]
    Media(String),

    #[command(rename = "cancel", description = "отменить текущую операцию")]
    Cancel,
}

async fn handle_start_command(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    check_registered: CheckRegistered,
    get_player: GetPlayer,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    let registered = check_registered.execute(user_id).await?;
    if !registered {
        send_registration_closed(&bot, &msg).await
    } else {
        let player = get_player.execute(user_id).await?;
        prompt_menu(bot, msg, dialogue, &player).await
    }
}

async fn send_registration_closed(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::REGISTRATION_CLOSED)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn handle_upload_command(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    command: BotCommand,
    check_admin: CheckAdmin,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    if !check_admin.execute(user_id).await? {
        send_permission_denied(&bot, &msg).await?;
    } else if let BotCommand::Upload(key) = command {
        match MediaID::new(key) {
            Err(_) => send_invalid_usage_upload_command(&bot, &msg).await?,
            Ok(id) => prompt_media(bot, msg, dialogue, id).await?,
        }
    }
    Ok(())
}

async fn send_invalid_usage_upload_command(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_UPLOAD_COMMAND_USAGE)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_media(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    media_id: MediaID,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_MEDIA)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::Media(media_id)).await?;
    Ok(())
}

async fn receive_media(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    upload_media: UploadMedia,
    get_media: GetMedia,
    media_id: MediaID,
) -> BotHandlerResult {
    if let Some(photos) = msg.photo() {
        let photo = photos.first().unwrap();
        let file_id = FileID::new(photo.file.id.0.clone())?;
        let media = Media::image(media_id.clone(), file_id.clone());
        upload_media.execute(media).await?;
        let media = get_media.execute(media_id.clone()).await?;
        send_media_with_caption(
            &bot,
            &msg,
            media,
            &format!("<code>{}</code>", media_id.as_str()),
        )
        .await?;
        send_successful_media_uploaded(bot, msg, dialogue, &file_id).await?;
    } else if let Some(video_note) = msg.video_note() {
        let file_id = FileID::new(video_note.file.id.0.clone())?;
        let media = Media::video_note(media_id.clone(), file_id.clone());
        upload_media.execute(media).await?;
        let media = get_media.execute(media_id.clone()).await?;
        send_media_with_caption(
            &bot,
            &msg,
            media,
            &format!("<code>{}</code>", media_id.as_str()),
        )
        .await?;
        send_successful_media_uploaded(bot, msg, dialogue, &file_id).await?;
    } else {
        send_unknown_media_format(bot, msg, dialogue).await?;
    }
    Ok(())
}

async fn send_unknown_media_format(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::UNKNOWN_MEDIA_FORMAT)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn send_successful_media_uploaded(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    file_id: &FileID,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::media_uploaded(file_id))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn handle_media_command(
    bot: Bot,
    msg: Message,
    command: BotCommand,
    get_media: GetMedia,
    check_admin: CheckAdmin,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    if !check_admin.execute(user_id).await? {
        send_permission_denied(&bot, &msg).await
    } else if let BotCommand::Media(key) = command {
        match MediaID::new(key) {
            Err(_) => send_invalid_usage_media_command(&bot, &msg).await,
            Ok(id) => match get_media.execute(id.clone()).await {
                Ok(media) => {
                    send_media_with_caption(
                        &bot,
                        &msg,
                        media,
                        &format!("<code>{}</code>", id.as_str()),
                    )
                    .await
                }
                Err(AppError::MediaNotFound(_)) => send_media_not_found(&bot, &msg, id).await,
                Err(err) => Err(err),
            },
        }
    } else {
        send_invalid_usage_media_command(&bot, &msg).await
    }
}

async fn send_invalid_usage_media_command(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::INVALID_MEDIA_COMMAND_USAGE)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_media_not_found(bot: &Bot, msg: &Message, id: MediaID) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::media_not_found(&id))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn commands_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<BotCommand, _>()
        .branch(case![BotCommand::Start(payload)].endpoint(handle_start_command))
        .branch(case![BotCommand::Upload(key)].endpoint(handle_upload_command))
        .branch(case![BotCommand::Media(key)].endpoint(handle_media_command));

    Update::filter_message()
        .branch(command_handler)
        .branch(case![BotState::Media(key)].endpoint(receive_media))
}
