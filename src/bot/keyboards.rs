use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::app::usecases::dto::UserDTO;
use crate::domain::models::{CharacterName, TaskID, TaskOption, TrackTag};

type StaticStr = &'static str;

pub const BTN_BACK: StaticStr = "Назад";

pub fn make_back_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new(BTN_BACK)]])
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_START: StaticStr = "Начать";

pub fn make_start_and_back_keyboard() -> KeyboardMarkup {
    let buttons = vec![vec![
        KeyboardButton::new(BTN_START),
        KeyboardButton::new(BTN_BACK),
    ]];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_tracks_keyboard_with_back(tracks: &[TrackTag]) -> KeyboardMarkup {
    let mut keyboard = Vec::new();
    for chunk in tracks.chunks(2) {
        let row: Vec<_> = chunk
            .iter()
            .map(|name| KeyboardButton::new(name.as_str()))
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

pub const BTN_AVAILABLE_TASKS: StaticStr = "Доступные задания";
pub const BTN_COMPLETED_TASKS: StaticStr = "Решённые задания";

pub fn make_tasks_group_keyboard_with_back(is_captain: bool) -> KeyboardMarkup {
    let mut first_row = vec![
        KeyboardButton::new(BTN_COMPLETED_TASKS),
    ];
    if is_captain {
        first_row.push(KeyboardButton::new(BTN_AVAILABLE_TASKS));
    }
    let buttons = vec![
        first_row,
        vec![KeyboardButton::new(BTN_BACK)],
    ];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()

}

pub const BTN_TASK_ID_PREFIX: StaticStr = "Задание ";

pub fn make_tasks_keyboard_with_back(task_ids: &[TaskID]) -> KeyboardMarkup {
    let mut keyboard = Vec::new();
    for chunk in task_ids.chunks(4) {
        let row: Vec<_> = chunk
            .iter()
            .map(|&id| KeyboardButton::new(format!("{BTN_TASK_ID_PREFIX}{id}")))
            .collect();
        keyboard.push(row);
    }
    keyboard.push(vec![KeyboardButton::new(BTN_BACK)]);
    KeyboardMarkup::new(keyboard)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_options_keyboard_with_back(options: &[TaskOption]) -> KeyboardMarkup {
    let mut keyboard = Vec::new();
    for chunk in options.chunks(2) {
        let row: Vec<_> = chunk
            .iter()
            .map(|option| KeyboardButton::new(option.to_string()))
            .collect();
        keyboard.push(row);
    }
    keyboard.push(vec![KeyboardButton::new(BTN_BACK)]);
    KeyboardMarkup::new(keyboard)
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_PROFILE: StaticStr = "Профиль";
pub const BTN_MY_TEAM: StaticStr = "Моя команда";
pub const BTN_TRACKS: StaticStr = "Треки";
pub const BTN_CHARACTERS: StaticStr = "Личности";
pub const BTN_GIVE_FEEDBACK: StaticStr = "Комментарий";

pub fn make_menu_keyboard(user: &UserDTO) -> KeyboardMarkup {
    let mut buttons = Vec::new();

    buttons.push(vec![KeyboardButton::new(BTN_TRACKS)]);
    
    //let mut first = vec![KeyboardButton::new(BTN_PROFILE)];
    //if user.team_id.is_some() { 
    //    first.push(KeyboardButton::new(BTN_MY_TEAM));
    //}
    //buttons.push(first);

    buttons.push(vec![KeyboardButton::new(BTN_CHARACTERS)]);
    buttons.push(vec![KeyboardButton::new(BTN_GIVE_FEEDBACK)]);

    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}
