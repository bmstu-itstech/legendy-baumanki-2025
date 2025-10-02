use crate::app::usecases::{AnswerTask, CheckCaptain, GetAvailableTracks, GetCompletedTasks};
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{InputFile, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::{CheckStartedTrack, GetAvailableTasks, GetTask, GetTrackInProgress, GetUser, StartTrack};
use crate::app::usecases::dto::{TaskDTO, TrackInProgressDTO};
use crate::bot::{fsm::BotDialogue, keyboards, texts, BotHandlerResult};
use crate::bot::fsm::BotState;
use crate::bot::handlers::menu::prompt_menu;
use crate::bot::handlers::shared::{send_enter_message, send_use_keyboard};
use crate::bot::keyboards::{make_back_keyboard, make_options_keyboard_with_back, make_start_and_back_keyboard, make_tasks_group_keyboard_with_back, make_tasks_keyboard_with_back, make_tracks_keyboard_with_back, BTN_TASK_ID_PREFIX};
use crate::domain::models::{TaskID, TaskType, TrackTag, UserID};

pub async fn prompt_track(bot: Bot, msg: Message, dialogue: BotDialogue, tracks: &[TrackTag]) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TRACK)
        .reply_markup(make_tracks_keyboard_with_back(tracks))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::Track).await?;
    Ok(())
}

async fn receive_track(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_user: GetUser,
    check_started_track: CheckStartedTrack,
    get_track_in_progress: GetTrackInProgress,
    check_captain: CheckCaptain,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(text) => {
            if let Some(tag) = TrackTag::try_parse(text) {
                let started= check_started_track.execute(user_id, tag).await?;
                if started {
                    let track = get_track_in_progress.execute(user_id, tag).await?;
                    let is_captain = check_captain.execute(user_id).await?;
                    prompt_track_task_groups(bot, msg, dialogue, &track, is_captain).await
                } else {
                    prompt_track_start(bot, msg, dialogue, tag).await
                }
            } else {
                send_use_keyboard(&bot, &msg).await
            }
        }
    }
}

async fn prompt_track_start(bot: Bot, msg: Message, dialogue: BotDialogue, tag: TrackTag) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TRACK_START)
        .reply_markup(make_start_and_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::StartTrack(tag)).await?;
    Ok(())
}

async fn receive_track_start(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    tag: TrackTag,
    start_track: StartTrack,
    get_available_tracks: GetAvailableTracks,
    check_captain: CheckCaptain,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let tracks = get_available_tracks.execute(user_id).await?;
            prompt_track(bot, msg, dialogue, &tracks).await
        }
        Some(keyboards::BTN_START) => {
            let track = start_track.execute(user_id, tag).await?;
            let is_captain = check_captain.execute(user_id).await?;
            prompt_track_task_groups(bot, msg, dialogue, &track, is_captain).await
        }
        Some(_) => send_use_keyboard(&bot, &msg).await,
    }
}

async fn prompt_track_task_groups(
    bot: Bot, 
    msg: Message, 
    dialogue: BotDialogue, 
    track: &TrackInProgressDTO,
    is_captain: bool,
) -> BotHandlerResult {
    bot.send_photo(msg.chat.id, InputFile::file_id(track.media.file_id.clone().into()))
        .caption(texts::track_menu(&track))
        .reply_markup(make_tasks_group_keyboard_with_back(is_captain))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TrackTaskGroup(track.tag)).await?;
    Ok(())
}

async fn receive_tasks_group(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    tag: TrackTag,
    get_available_tasks: GetAvailableTasks,
    get_completed_tasks: GetCompletedTasks,
    get_available_tracks: GetAvailableTracks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let tracks = get_available_tracks.execute(user_id).await?;
            prompt_track(bot, msg, dialogue, &tracks).await
        }
        Some(keyboards::BTN_AVAILABLE_TASKS) => {
            let available_tasks = get_available_tasks.execute(user_id, tag).await?;
            if available_tasks.is_empty() {
                send_all_task_completed(&bot, &msg).await?;
                let tracks = get_available_tracks.execute(user_id).await?;
                prompt_track(bot, msg, dialogue, &tracks).await
            } else {
                prompt_available_task(bot, msg, dialogue, tag, &available_tasks).await
            }
        }
        Some(keyboards::BTN_COMPLETED_TASKS) => {
            let completed_tasks = get_completed_tasks.execute(user_id, tag).await?;
            if completed_tasks.is_empty() {
                send_no_tasks_completed(&bot, &msg).await?;
                let tracks = get_available_tracks.execute(user_id).await?;
                prompt_track(bot, msg, dialogue, &tracks).await
            } else {
                prompt_completed_task(bot, msg, dialogue, tag, &completed_tasks).await
            }
        }
        Some(_) => send_use_keyboard(&bot, &msg).await,
    }
}

async fn send_all_task_completed(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::ALL_TRACK_TASKS_COMPLETED)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_no_tasks_completed(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::NO_COMPLETED_TASKS)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_available_task(bot: Bot, msg: Message, dialogue: BotDialogue, track_tag: TrackTag, task_ids: &[TaskID]) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_AVAILABLE_TASK)
        .reply_markup(make_tasks_keyboard_with_back(task_ids))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::AvailableTask(track_tag)).await?;
    Ok(())
}

async fn prompt_completed_task(bot: Bot, msg: Message, dialogue: BotDialogue, track_tag: TrackTag, task_ids: &[TaskID]) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_COMPLETED_TASK)
        .reply_markup(make_tasks_keyboard_with_back(task_ids))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::CompletedTask(track_tag)).await?;
    Ok(())
}

async fn receive_available_task(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    tag: TrackTag,
    get_task: GetTask,
    get_available_tracks: GetAvailableTracks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let tracks = get_available_tracks.execute(user_id).await?;
            prompt_track(bot, msg, dialogue, &tracks).await
        }
        Some(text) => match text.strip_prefix(BTN_TASK_ID_PREFIX) {
            None => send_use_keyboard(&bot, &msg).await,
            Some(name) => match name.parse::<i32>() {
                Err(_) => send_use_keyboard(&bot, &msg).await,
                Ok(id) => {
                    let task = get_task.execute(id).await?;
                    prompt_text_task_answer(bot, msg, dialogue, tag, &task).await
                }
            }
        }
    }
}

async fn receive_completed_task(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    tag: TrackTag,
    get_task: GetTask,
    get_available_tracks: GetAvailableTracks,
    get_completed_tasks: GetCompletedTasks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let tracks = get_available_tracks.execute(user_id).await?;
            prompt_track(bot, msg, dialogue, &tracks).await
        }
        Some(text) => match text.strip_prefix(BTN_TASK_ID_PREFIX) {
            None => send_use_keyboard(&bot, &msg).await,
            Some(name) => match name.parse::<i32>() {
                Err(_) => send_use_keyboard(&bot, &msg).await,
                Ok(id) => {
                    let task = get_task.execute(id).await?;
                    send_task_question_and_explanation(&bot, &msg, &task).await?;
                    let completed_tasks = get_completed_tasks.execute(user_id, tag).await?;
                    prompt_completed_task(bot, msg, dialogue, tag, &completed_tasks).await
                }
            }
        }
    }
}

async fn prompt_text_task_answer(bot: Bot, msg: Message, dialogue: BotDialogue, track_tag: TrackTag, task: &TaskDTO) -> BotHandlerResult {
    let keyboard = if matches!(task.task_type, TaskType::Choice) {
        make_options_keyboard_with_back(&task.options)
    } else {
        make_back_keyboard()
    };

    bot.send_message(msg.chat.id, task.question.as_str())
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TaskAnswer(track_tag, task.id)).await?;
    Ok(())
}

async fn receive_task_answer(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    (tag, task_id): (TrackTag, TaskID),
    get_track_in_progress: GetTrackInProgress,
    answer_task: AnswerTask,
    get_task: GetTask,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let track = get_track_in_progress.execute(user_id, tag).await?;
            prompt_track_task_groups(bot, msg, dialogue, &track, true).await
        }
        Some(text) => {
            let answer = answer_task.execute(user_id, tag, task_id, text.into()).await?;
            if answer.completed {
                send_answer_is_correct(&bot, &msg).await?;
                let task = get_task.execute(task_id).await?;
                send_task_explanation(&bot, &msg, &task).await?;
                let track = get_track_in_progress.execute(user_id, tag).await?;
                prompt_track_task_groups(bot, msg, dialogue, &track, true).await
            } else {
                send_answer_is_invalid(&bot, &msg).await
            }
        }
    }
}

async fn send_answer_is_correct(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::correct_answer())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_task_explanation(bot: &Bot, msg: &Message, task: &TaskDTO) -> BotHandlerResult {
    bot.send_message(msg.chat.id, task.explanation.to_string())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_task_question_and_explanation(bot: &Bot, msg: &Message, task: &TaskDTO) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::task_question_and_explanation(&task))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_answer_is_invalid(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::invalid_answer())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub fn tracks_scheme() -> UpdateHandler<AppError> {
    use dptree::case;

    Update::filter_message()
        .branch(case![BotState::Track].endpoint(receive_track))
        .branch(case![BotState::StartTrack(tag)].endpoint(receive_track_start))
        .branch(case![BotState::TrackTaskGroup(tag)].endpoint(receive_tasks_group))
        .branch(case![BotState::AvailableTask(tag)].endpoint(receive_available_task))
        .branch(case![BotState::TaskAnswer(tag, task_id)].endpoint(receive_task_answer))
        .branch(case![BotState::CompletedTask(tag)].endpoint(receive_completed_task))
}
