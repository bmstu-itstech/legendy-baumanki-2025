use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::types::{InputFile, Me, Message, ParseMode};

use crate::app::error::AppError;
use crate::app::usecases::dto::{
    CharacterDTO, Profile, TaskDTO, TeamDTO, TeamWithMembersDTO, UserDTO, UserTaskDTO,
};
use crate::app::usecases::{
    AnswerTask, CreateTeam, ExitTeam, GetCharacter, GetCharacterNames, GetMedia, GetProfile,
    GetTask, GetTeamWithMembers, GetUser, GetUserTask, GetUserTasks, GetUserTeam, GiveFeedback,
    JoinTeam, SwitchToLookingForTeam, SwitchToSoloMode,
};
use crate::bot::fsm::{BotDialogue, BotState};
use crate::bot::handlers::shared::{send_enter_message, send_use_keyboard};
use crate::bot::keyboards::{
    make_back_keyboard, make_characters_keyboard_with_back, make_menu_keyboard,
    make_task_keyboard_with_back, make_yes_and_back_keyboard,
};
use crate::bot::{BotHandlerResult, keyboards, texts};
use crate::domain::error::DomainError;
use crate::domain::models::{
    CharacterName, FeedbackText, TaskID, TaskType, TeamID, TeamName, UserID,
};

pub async fn prompt_menu(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    user: &UserDTO,
) -> BotHandlerResult {
    let markup = make_menu_keyboard(user);
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
    get_user: GetUser,
    get_user_team: GetUserTeam,
    get_team_with_members: GetTeamWithMembers,
    get_profile: GetProfile,
    get_user_tasks: GetUserTasks,
    get_character_names: GetCharacterNames,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(text) => match text {
            keyboards::BTN_PROFILE => {
                let profile = get_profile.execute(user_id).await?;
                send_profile(&bot, &msg, &profile).await?;
                let user = profile.into();
                prompt_menu(bot, msg, dialogue, &user).await?;
            }
            keyboards::BTN_JOIN_TEAM => prompt_team_code(bot, msg, dialogue).await?,
            keyboards::BTN_CREATE_TEAM => prompt_team_name(bot, msg, dialogue).await?,
            keyboards::BTN_MY_TEAM => {
                if let Some(team) = get_user_team.execute(user_id).await? {
                    let team = get_team_with_members.execute(team.id).await?;
                    send_my_team(&bot, &me, &msg, team).await?;
                    let user = get_user.execute(user_id).await?;
                    prompt_menu(bot, msg, dialogue, &user).await?;
                }
            }
            keyboards::BTN_EXIT_TEAM => prompt_exit_approval(bot, msg, dialogue).await?,
            keyboards::BTN_REBUSES => {
                let rebuses = get_user_tasks.execute(user_id, TaskType::Rebus).await?;
                prompt_rebus(bot, msg, dialogue, rebuses.as_ref()).await?
            }
            keyboards::BTN_RIDDLES => {
                let riddles = get_user_tasks.execute(user_id, TaskType::Riddle).await?;
                prompt_riddle(bot, msg, dialogue, riddles.as_ref()).await?
            }
            keyboards::BTN_CHARACTERS => {
                let names = get_character_names.execute().await?;
                prompt_character_name(bot, msg, dialogue, &names).await?
            }
            keyboards::BTN_TO_SOLO_MODE => prompt_solo_mode_approval(bot, msg, dialogue).await?,
            keyboards::BTN_TO_LOOKING_FOR_TEAM => {
                prompt_team_mode_approval(bot, msg, dialogue).await?
            }
            keyboards::BTN_GIVE_FEEDBACK => prompt_feedback(bot, msg, dialogue).await?,
            _ => {
                send_unknown_menu_option(&bot, &msg).await?;
                let user = get_user.execute(user_id).await?;
                prompt_menu(bot, msg, dialogue, &user).await?
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
        .reply_markup(make_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TeamCode).await?;
    Ok(())
}

async fn prompt_team_name(bot: Bot, msg: Message, dialogue: BotDialogue) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TEAM_NAME)
        .reply_markup(make_back_keyboard())
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

async fn send_profile(bot: &Bot, msg: &Message, profile: &Profile) -> BotHandlerResult {
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
    get_user: GetUser,
    join_team: JoinTeam,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
        }
        Some(text) => match TeamID::try_from(text.to_string()) {
            Err(_) => send_invalid_team_code(&bot, &msg).await?,
            Ok(team_id) => match join_team.execute(user_id, team_id).await {
                Err(AppError::TeamNotFound(_)) => {
                    let user = get_user.execute(user_id).await?;
                    send_team_not_found(&bot, &msg).await?;
                    prompt_menu(bot, msg, dialogue, &user).await?;
                }
                Err(AppError::DomainError(DomainError::TeamIsFull(_))) => {
                    let user = get_user.execute(user_id).await?;
                    send_team_is_full(&bot, &msg).await?;
                    prompt_menu(bot, msg, dialogue, &user).await?;
                }
                Err(err) => return Err(err),
                Ok(team) => {
                    let user = get_user.execute(user_id).await?;
                    send_joining_team_successful(&bot, &msg, team.name).await?;
                    prompt_menu(bot, msg, dialogue, &user).await?;
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
    get_user: GetUser,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
        }
        Some(text) => match TeamName::new(text.to_string()) {
            Err(_) => send_team_name_is_invalid(&bot, &msg).await?,
            Ok(team_name) => {
                let team = create_team.execute(team_name, user_id).await?;
                send_team_successful_created(&bot, &me, &msg, team).await?;
                let user = get_user.execute(user_id).await?;
                prompt_menu(bot, msg, dialogue, &user).await?;
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
    get_user: GetUser,
) -> BotHandlerResult {
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let user_id = UserID::new(msg.chat.id.0);
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?
        }
        Some(keyboards::BTN_YES) => {
            let user_id = UserID::new(msg.chat.id.0);
            exit_team.execute(user_id).await?;
            send_successfully_exited_team(&bot, &msg).await?;
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
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

async fn prompt_rebus(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    rebuses: &[UserTaskDTO],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::rebuses_menu_text(rebuses))
        .reply_markup(make_task_keyboard_with_back(rebuses, TaskType::Rebus))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::Rebus).await?;
    Ok(())
}

async fn prompt_riddle(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    riddles: &[UserTaskDTO],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::riddle_menu_text(riddles))
        .reply_markup(make_task_keyboard_with_back(riddles, TaskType::Riddle))
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::Riddle).await?;
    Ok(())
}

async fn receive_rebus(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_tasks: GetUserTasks,
    get_user_task: GetUserTask,
    get_media: GetMedia,
    get_user: GetUser,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
        }
        Some(text) => match text.strip_prefix("Ребус ") {
            None => send_use_keyboard(&bot, &msg).await?,
            Some(name) => match name.parse::<u32>() {
                Err(_) => send_use_keyboard(&bot, &msg).await?,
                Ok(idx) => {
                    let tasks = get_tasks.execute(user_id, TaskType::Rebus).await?;
                    match tasks.iter().find(|&t| t.index == idx) {
                        None => send_use_keyboard(&bot, &msg).await?,
                        Some(user_task) => {
                            let task_id = user_task.id.clone();
                            let task = get_user_task.execute(user_id, task_id).await?;
                            if task.solved {
                                send_rebus_already_solved(&bot, &msg, &tasks).await?;
                            } else {
                                prompt_rebus_answer(bot, msg, dialogue, get_media, task).await?;
                            }
                        }
                    }
                }
            },
        },
    }
    Ok(())
}

async fn receive_riddle(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_tasks: GetUserTasks,
    get_user_task: GetUserTask,
    get_media: GetMedia,
    get_user: GetUser,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await?;
        }
        Some(text) => match text.strip_prefix("Загадка ") {
            None => send_use_keyboard(&bot, &msg).await?,
            Some(name) => match name.parse::<u32>() {
                Err(_) => send_use_keyboard(&bot, &msg).await?,
                Ok(idx) => {
                    let tasks = get_tasks.execute(user_id, TaskType::Riddle).await?;
                    match tasks.iter().find(|&t| t.index == idx) {
                        None => send_use_keyboard(&bot, &msg).await?,
                        Some(user_task) => {
                            let task_id = user_task.id.clone();
                            let task = get_user_task.execute(user_id, task_id).await?;
                            if task.solved {
                                send_riddle_already_solved(&bot, &msg, &tasks).await?;
                            } else {
                                prompt_riddle_answer(bot, msg, dialogue, get_media, task).await?;
                            }
                        }
                    }
                }
            },
        },
    }
    Ok(())
}

async fn send_rebus_already_solved(
    bot: &Bot,
    msg: &Message,
    tasks: &[UserTaskDTO],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::REBUS_ALREADY_SOLVED)
        .reply_markup(make_task_keyboard_with_back(tasks, TaskType::Rebus))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_riddle_already_solved(
    bot: &Bot,
    msg: &Message,
    tasks: &[UserTaskDTO],
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::RIDDLE_ALREADY_SOLVED)
        .reply_markup(make_task_keyboard_with_back(tasks, TaskType::Riddle))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_rebus_answer(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_media: GetMedia,
    task: UserTaskDTO,
) -> BotHandlerResult {
    let media = get_media.execute(task.media_id).await?;
    bot.send_photo(msg.chat.id, media.into())
        .reply_markup(make_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::RebusAnswer(task.id)).await?;
    Ok(())
}

async fn prompt_riddle_answer(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_media: GetMedia,
    task: UserTaskDTO,
) -> BotHandlerResult {
    let media = get_media.execute(task.media_id).await?;
    bot.send_photo(msg.chat.id, media.into())
        .reply_markup(make_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::RiddleAnswer(task.id)).await?;
    Ok(())
}

async fn receive_rebus_answer(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    task_id: TaskID,
    answer_task: AnswerTask,
    get_task: GetTask,
    get_user_tasks: GetUserTasks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let tasks = get_user_tasks.execute(user_id, TaskType::Rebus).await?;
            prompt_rebus(bot, msg, dialogue, &tasks).await?;
        }
        Some(text) => {
            let text = text.to_string();
            let answer = answer_task.execute(user_id, task_id.clone(), text).await?;
            if answer.solved {
                let task = get_task.execute(task_id).await?;
                send_task_solved(&bot, &msg, task).await?;
                let tasks = get_user_tasks.execute(user_id, TaskType::Rebus).await?;
                prompt_rebus(bot, msg, dialogue, &tasks).await?;
            } else {
                send_task_incorrect_answer(&bot, &msg).await?;
            }
        }
    }
    Ok(())
}

async fn receive_riddle_answer(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    task_id: TaskID,
    answer_task: AnswerTask,
    get_task: GetTask,
    get_user_tasks: GetUserTasks,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await?,
        Some(keyboards::BTN_BACK) => {
            let tasks = get_user_tasks.execute(user_id, TaskType::Riddle).await?;
            prompt_riddle(bot, msg, dialogue, &tasks).await?;
        }
        Some(text) => {
            let text = text.to_string();
            let answer = answer_task.execute(user_id, task_id.clone(), text).await?;
            if answer.solved {
                let task = get_task.execute(task_id).await?;
                send_task_solved(&bot, &msg, task).await?;
                let tasks = get_user_tasks.execute(user_id, TaskType::Riddle).await?;
                prompt_riddle(bot, msg, dialogue, &tasks).await?;
            } else {
                send_task_incorrect_answer(&bot, &msg).await?;
            }
        }
    }
    Ok(())
}

async fn send_task_solved(bot: &Bot, msg: &Message, task: TaskDTO) -> BotHandlerResult {
    bot.send_message(msg.chat.id, task.explanation.as_str())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn send_task_incorrect_answer(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::TASK_INCORRECT_ANSWER)
        .reply_markup(make_back_keyboard())
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
    get_user: GetUser,
    get_character: GetCharacter,
    get_character_names: GetCharacterNames,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
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

async fn prompt_solo_mode_approval(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_SOLO_MODE_APPROVAL)
        .reply_markup(make_yes_and_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::SoloModeApproval).await?;
    Ok(())
}

async fn receive_solo_mode_approval(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_user: GetUser,
    switch_to_solo_mode: SwitchToSoloMode,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(keyboards::BTN_YES) => {
            switch_to_solo_mode.execute(user_id).await?;
            send_solo_mode_enabled(&bot, &msg).await?;
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(_) => {
            send_use_keyboard(&bot, &msg).await?;
            prompt_exit_approval(bot, msg, dialogue).await
        }
    }
}

async fn send_solo_mode_enabled(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::SOLO_MODE_ENABLED)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn prompt_team_mode_approval(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::PROMPT_TEAM_MODE_APPROVAL)
        .reply_markup(make_yes_and_back_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(BotState::TeamModeApproval).await?;
    Ok(())
}

async fn receive_team_mode_approval(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    get_user: GetUser,
    switch_to_team_mode: SwitchToLookingForTeam,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(keyboards::BTN_YES) => {
            switch_to_team_mode.execute(user_id).await?;
            send_team_mode_enabled(&bot, &msg).await?;
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(_) => {
            send_use_keyboard(&bot, &msg).await?;
            prompt_exit_approval(bot, msg, dialogue).await
        }
    }
}

async fn send_team_mode_enabled(bot: &Bot, msg: &Message) -> BotHandlerResult {
    bot.send_message(msg.chat.id, texts::TEAM_MODE_ENABLED)
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
    get_user: GetUser,
) -> BotHandlerResult {
    let user_id = UserID::new(msg.chat.id.0);
    match msg.text() {
        None => send_enter_message(&bot, &msg).await,
        Some(keyboards::BTN_BACK) => {
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
        }
        Some(text) => {
            let text = FeedbackText::new(text.to_string())?;
            give_feedback.execute(user_id, text).await?;
            send_feedback_sent(&bot, &msg).await?;
            let user = get_user.execute(user_id).await?;
            prompt_menu(bot, msg, dialogue, &user).await
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
        .branch(case![BotState::TeamCode].endpoint(receive_team_code))
        .branch(case![BotState::TeamName].endpoint(receive_team_name))
        .branch(case![BotState::ExitApproval].endpoint(receive_exit_approval))
        .branch(case![BotState::Rebus].endpoint(receive_rebus))
        .branch(case![BotState::RebusAnswer(task_id)].endpoint(receive_rebus_answer))
        .branch(case![BotState::Riddle].endpoint(receive_riddle))
        .branch(case![BotState::RiddleAnswer(task_id)].endpoint(receive_riddle_answer))
        .branch(case![BotState::CharacterName].endpoint(receive_character_name))
        .branch(case![BotState::SoloModeApproval].endpoint(receive_solo_mode_approval))
        .branch(case![BotState::TeamModeApproval].endpoint(receive_team_mode_approval))
        .branch(case![BotState::Feedback].endpoint(receive_feedback))
}
