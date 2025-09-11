use teloxide::types::{KeyboardButton, KeyboardMarkup};

type StaticStr = &'static str;

pub const BTN_AGREEMENT: StaticStr = "Подтверждаю";

pub fn make_agreement_keyboard() -> KeyboardMarkup {
    let buttons = vec![vec![KeyboardButton::new(BTN_AGREEMENT)]];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_YES: StaticStr = "Да";
pub const BTN_BACK: StaticStr = "Назад";

pub fn make_yes_and_back_keyboard() -> KeyboardMarkup {
    let buttons = vec![vec![
        KeyboardButton::new(BTN_YES),
        KeyboardButton::new(BTN_BACK),
    ]];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_back_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new(BTN_BACK)]])
        .resize_keyboard()
        .one_time_keyboard()
}

pub const BTN_JOIN_TEAM: StaticStr = "Вступить в команду";
pub const BTN_CREATE_TEAM: StaticStr = "Создать команду";
pub const BTN_MY_TEAM: StaticStr = "Моя команда";
pub const BTN_EXIT_TEAM: StaticStr = "Покинуть команду";
pub const BTN_PROFILE: StaticStr = "Профиль";

pub fn make_menu_keyboard_without_team() -> KeyboardMarkup {
    let buttons = vec![
        vec![
            KeyboardButton::new(BTN_JOIN_TEAM),
            KeyboardButton::new(BTN_CREATE_TEAM),
        ],
        vec![KeyboardButton::new(BTN_PROFILE)],
    ];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}

pub fn make_menu_keyboard_with_team() -> KeyboardMarkup {
    let buttons = vec![
        vec![
            KeyboardButton::new(BTN_MY_TEAM),
            KeyboardButton::new(BTN_EXIT_TEAM),
        ],
        vec![KeyboardButton::new(BTN_PROFILE)],
    ];
    KeyboardMarkup::new(buttons)
        .resize_keyboard()
        .one_time_keyboard()
}
