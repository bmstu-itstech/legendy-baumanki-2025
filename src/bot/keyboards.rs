use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::app::usecases::dto::{UserDTO, UserTaskDTO};
use crate::domain::models::{CharacterName, ParticipantStatus, TaskType};

type StaticStr = &'static str;

pub const BTN_AGREEMENT: StaticStr = "Подтверждаю";

pub fn make_agreement_keyboard() -> KeyboardMarkup {
    let buttons = vec![vec![KeyboardButton::new(BTN_AGREEMENT)]];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_BACK: StaticStr = "Назад";

pub fn make_back_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new(BTN_BACK)]])
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_YES: StaticStr = "Да";

pub fn make_yes_and_back_keyboard() -> KeyboardMarkup {
    let buttons = vec![vec![
        KeyboardButton::new(BTN_YES),
        KeyboardButton::new(BTN_BACK),
    ]];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_JOIN_TEAM: StaticStr = "Вступить в команду";
pub const BTN_CREATE_TEAM: StaticStr = "Создать команду";
pub const BTN_MY_TEAM: StaticStr = "Моя команда";
pub const BTN_EXIT_TEAM: StaticStr = "Покинуть команду";
pub const BTN_PROFILE: StaticStr = "Профиль";
pub const BTN_REBUSES: StaticStr = "Ребусы";
pub const BTN_RIDDLES: StaticStr = "Загадки";
pub const BTN_CHARACTERS: StaticStr = "Личности";
pub const BTN_TO_SOLO_MODE: StaticStr = "Перейти в соло-режим";
pub const BTN_TO_LOOKING_FOR_TEAM: StaticStr = "Вернуться в командный режим";
pub const BTN_GIVE_FEEDBACK: StaticStr = "Комментарий";

pub fn make_menu_keyboard(user: &UserDTO) -> KeyboardMarkup {
    let mut buttons = Vec::new();

    buttons.push(vec![KeyboardButton::new(BTN_PROFILE)]);
    if matches!(user.mode, ParticipantStatus::Team(_)) {
        buttons.push(vec![
            KeyboardButton::new(BTN_MY_TEAM),
        ]);
    }

    buttons.push(vec![
        KeyboardButton::new(BTN_REBUSES),
        KeyboardButton::new(BTN_RIDDLES),
    ]);
    buttons.push(vec![KeyboardButton::new(BTN_CHARACTERS)]);
    buttons.push(vec![KeyboardButton::new(BTN_GIVE_FEEDBACK)]);

    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_task_keyboard_with_back(tasks: &[UserTaskDTO], task_type: TaskType) -> KeyboardMarkup {
    let mut keyboard = Vec::new();
    for chunk in tasks.chunks(3) {
        let row: Vec<_> = chunk
            .iter()
            .map(|t| KeyboardButton::new(format!("{} {}", task_type.as_str(), t.index)))
            .collect();
        keyboard.push(row);
    }
    keyboard.push(vec![KeyboardButton::new(BTN_BACK)]);
    KeyboardMarkup::new(keyboard)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_characters_keyboard_with_back(names: &[CharacterName]) -> KeyboardMarkup {
    let mut keyboard = Vec::new();
    for chunk in names.chunks(3) {
        let row: Vec<_> = chunk
            .iter()
            .map(|name| KeyboardButton::new(name.to_string()))
            .collect();
        keyboard.push(row);
    }
    keyboard.push(vec![KeyboardButton::new(BTN_BACK)]);
    KeyboardMarkup::new(keyboard)
        .resize_keyboard()
        .one_time_keyboard()
}
