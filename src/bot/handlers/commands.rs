use teloxide::dispatching::UpdateHandler;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::{
    CheckAdmin, CheckRegistered, GetMedia, GetUser, GetUserTeam, JoinTeam, UploadMedia,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::menu::{
    prompt_menu, send_joining_team_successful, send_team_is_full, send_team_not_exists,
};
use crate::bot::handlers::registration::prompt_pd_agreement;
use crate::bot::handlers::shared::{send_media_with_caption, send_permission_denied};
use crate::bot::resources::{RULES_IMAGE_1, RULES_IMAGE_2};
use crate::bot::{BotHandlerResult, texts};
use crate::domain::error::DomainError;
use crate::domain::models::{FileID, Media, MediaID, TeamID, UserID};

#[derive(BotCommands, Clone)]
#[command(description = "Команды регистрации")]
enum BotCommand {
    #[command(rename = "start", description = "начать регистрацию")]
    Start(String),

    #[command(rename = "upload", description = "загрузить медиа")]
    Upload(String),

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
    get_media: GetMedia,
    get_user: GetUser,
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
            send_greeting_message(&bot, &msg, get_media).await?;
            prompt_pd_agreement(bot, msg, dialogue, None).await?;
        }
        // Не зарегистрирован и есть код команды в ссылке
        (false, Some(team_id)) => {
            send_greeting_message(&bot, &msg, get_media).await?;
            prompt_pd_agreement(bot, msg, dialogue, Some(team_id)).await?;
        }
        // Зарегистрирован и нет кода команды
        (true, None) => {
            let user = get_user.user(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
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
                    Err(AppError::TeamNotFound(_)) => {
                        send_team_not_exists(&bot, &msg).await?;
                    }
                    Err(err) => return Err(err),
                    Ok(team) => send_joining_team_successful(&bot, &msg, team.name).await?,
                },
            }
            let user = get_user.user(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
        }
    }
    Ok(())
}

async fn send_greeting_message(bot: &Bot, msg: &Message, get_media: GetMedia) -> BotHandlerResult {
    let rules_1 = get_media
        .media(MediaID::new(RULES_IMAGE_1.to_string()).unwrap())
        .await?;
    let rules_2 = get_media
        .media(MediaID::new(RULES_IMAGE_2.to_string()).unwrap())
        .await?;
    log::debug!("{}", rules_1.file_id().as_str());
    let media_group = vec![
        InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id(
            rules_1.file_id().clone().into(),
        ))),
        InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id(
            rules_2.file_id().clone().into(),
        ))),
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

async fn handle_upload_command(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    command: BotCommand,
    check_admin: CheckAdmin,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    if !check_admin.is_admin(user_id).await? {
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
        upload_media.upload_media(media).await?;
        let media = get_media.media(media_id.clone()).await?;
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
        upload_media.upload_media(media).await?;
        let media = get_media.media(media_id.clone()).await?;
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

pub fn commands_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<BotCommand, _>()
        .branch(case![BotCommand::Start(payload)].endpoint(handle_start_command))
        .branch(case![BotCommand::Upload(key)].endpoint(handle_upload_command));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![BotState::Media(key)].endpoint(receive_media));

    message_handler
}
