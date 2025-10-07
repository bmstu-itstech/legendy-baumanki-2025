use crate::app::usecases::dto::{
    CharacterDTO, SlotDTO, TaskDTO, TeamWithMembersDTO, TrackDescriptionDTO, TrackInProgressDTO,
};
use crate::domain::models::{FileID, MediaID, TrackStatus};
use chrono::{Duration, Utc};
use rand::seq::IndexedRandom;

type StaticStr = &'static str;

pub const ENTER_MESSAGE_TEXT: StaticStr = "📝 Напиши мне текстовое сообщение";

pub const USE_KEYBOARD: StaticStr =
    "Пожалуйста, используй кнопки клавиатуры внизу для ответа. Так будет удобнее!";

pub const PERMISSION_DENIED: StaticStr = "<b>❌ Доступ запрещен</b>\n\
    У вас недостаточно прав для использования этой команды.";

pub const INTERNAL_ERROR: StaticStr = "⚠️ <b>Неизвестная ошибка</b>\n\
     Произошла непредвиденная ошибка. Пожалуйста, попробуй повторить действие позже или сообщи \
     об этом организаторам (см. описание бота).";

pub const MENU_TEXT: StaticStr = "📲 <b>Главное меню</b>\n\
    Выбери нужный пункт из меню ниже.";

pub const UNKNOWN_MENU_OPTION: StaticStr = "❌ <b>Непонятная команда</b>\n\
     К сожалению, я не понимаю этот запрос. Пожалуйста, выбери одну из доступных опций в меню.";

pub const SEND_APPROVAL_EXIT_TEAM: StaticStr =
    "❓ Ты действительно хочешь выйти из своей текущей команды?";

pub const SUCCESSFUL_EXIT_TEAM: StaticStr = "👋 <b>Ты вышел из команды</b>\n\
     Ты успешно покинул(а) команду. Теперь ты можешь вступить в новую, используя код-приглашение.";

pub const INVALID_UPLOAD_COMMAND_USAGE: StaticStr = "<b>❌ Неверный формат команды</b>\n\
    Использование:\n\
    <code>/upload key</code>";

pub const INVALID_MEDIA_COMMAND_USAGE: StaticStr = "<b>❌ Неверный формат команды</b>\n\
    Использование:\n\
    <code>/media key</code>";

pub const PROMPT_MEDIA: StaticStr = "📤 <b>Загрузка файла</b>\n\
    Отправьте файл (изображение или видеосообщение) в чат, чтобы получить его FileID.";

pub const UNKNOWN_MEDIA_FORMAT: StaticStr = "❌ <b>Формат не поддерживается</b>
    Отправьте изображение или видеосообщение.";

pub const PROMPT_CHARACTER_NAME: StaticStr = "☺️ <b>Отличный выбор!</b>\n\
    \n\
    Познакомься с великими учёными, инженерами и конструкторами, которые начинали свой путь здесь, в стенах МГТУ им. Н.Э. Баумана.\n\
    \n\
    <b><i>Кого из них ты хочешь узнать лучше?</i></b>";

pub const PROMPT_FEEDBACK: StaticStr = "✍️ Теперь ты можешь написать комментарий организаторам!";

pub const FEEDBACK_SENT: StaticStr = "📩 <b>Отлично, твой комментарий отправлен!</b>\n\
    \n\
    <i>Мы обязательно прочитаем его в ближайшее время!</i>";

pub const REGISTRATION_CLOSED: StaticStr = "🥺 <b>К сожалению, зарегистрироваться уже нельзя!</b>\n\
    \n\
    👉🏻 <i>Регистрация была доступна до 30 сентября, но не спеши расстраиваться. В следующем год ты сам сможешь стать организатором «Легенд Бауманки» — присоединяйся к <a href=\"https://forms.yandex.ru/u/6897839490fa7b44d1601991\">команде Студенческого совета</a> и вместе мы сделаем так, чтобы о новых «Легендах» говорили ещё громче!</i>";

pub const PROMPT_TRACK: StaticStr = "✨ <b>Выбери трек</b>";

pub const PROMPT_TRACK_START: StaticStr = "
    Как только вы начнёте этот трек запустится таймер, который покажет, насколько быстро вы способны прокачивать свои навыки!\n\
    \n\
    <b>Ваши задачи:</b>\n\
    💡 Отвечать на вопросы\n\
    💡 Выбирать верные ответы\n\
    💡 Присылать фотографии\n\
    💡 Действовать слаженно и быстро\n\
    \n\
    📸 <b>Важно:</b>\n\
    Помните, что проверка фотографий требует времени — не откладывайте их выполнение!\n\
    \n\
    <i>Вы уверены, что готовы начать?</i>";

const TASK_CORRECT_ANSWER_1: StaticStr =
    "✅ <b>Снова прав!</b> Ты уже не участник — ты мастер. Каждый твой шаг ближе к легенде.";

const TASK_CORRECT_ANSWER_2: StaticStr =
    "✅ <b>Ты справился!</b> И справился с достоинством. Такие моменты и делают нас сильнее!";

const TASK_CORRECT_ANSWER_3: StaticStr =
    "✅ <b>Верно!</b> Бауманка открывает свои секреты только избранным!";

const TASK_CORRECT_ANSWER_4: StaticStr =
    "✅ <b>Правильно!</b> Ты чувствуешь дух Бауманки как никто другой!";

const TASK_CORRECT_ANSWER_5: StaticStr =
    "✅ <b>Точно!</b> Ты читаешь историю Бауманки как открытую книгу!";

const TASK_CORRECT_ANSWERS: [StaticStr; 5] = [
    TASK_CORRECT_ANSWER_1,
    TASK_CORRECT_ANSWER_2,
    TASK_CORRECT_ANSWER_3,
    TASK_CORRECT_ANSWER_4,
    TASK_CORRECT_ANSWER_5,
];

pub fn correct_answer() -> &'static str {
    TASK_CORRECT_ANSWERS.choose(&mut rand::rng()).unwrap()
}

pub const TASK_INVALID_ANSWER_1: StaticStr =
    "❌ Мимо, но каждый промах — это шаг ближе к цели. Вперёд, у тебя получится!";

pub const TASK_INVALID_ANSWER_2: StaticStr = "❌ Почти! Но если один путь оказался тупиком — значит, другой точно ведёт к успеху. Давай попробуем снова?";

// Пусть дублируется, будем считать, что повышенный шанс на выпадение
pub const TASK_INVALID_ANSWER_3: StaticStr = "❌ Почти! Но если один путь оказался тупиком — значит, другой точно ведёт к успеху. Давай попробуем снова?";

pub const TASK_INVALID_ANSWER_4: StaticStr =
    "❌ Близко, но не совсем. Зато теперь голова работает на полную — давай ещё один шанс! ";

pub const TASK_INVALID_ANSWER_5: StaticStr =
    "❌ Это не он, но ты уже почти чувствуешь правильный ответ, верно? Не останавливайся!";

const TASK_INVALID_ANSWERS: [StaticStr; 5] = [
    TASK_INVALID_ANSWER_1,
    TASK_INVALID_ANSWER_2,
    TASK_INVALID_ANSWER_3,
    TASK_INVALID_ANSWER_4,
    TASK_INVALID_ANSWER_5,
];

pub fn invalid_answer() -> &'static str {
    TASK_INVALID_ANSWERS.choose(&mut rand::rng()).unwrap()
}

pub const PROMPT_AVAILABLE_TASK: StaticStr = "📲 <b>Выбери задание из списка</b>";

pub const PROMPT_COMPLETED_TASK: StaticStr = "📲 <b>Выбери задание из списка</b>";

pub const ALL_TRACK_TASKS_COMPLETED: StaticStr = "🎉 <b>Трек завершён!</b>\n\
    Поздравляю! Ты успешно выполнил(а) все задания этого трека. Теперь ты можешь перейти к другим трекам и продолжить своё путешествие по «Легендам Бауманки»!";

pub const NO_COMPLETED_TASKS: StaticStr = "📭 <b>Пусто</b>\n\
    Здесь пока нет ни одного завершённого задания.";

pub const TRACK_NOT_STARTED: StaticStr = "☝🏻<b>Стой! Это трек еще не был запущен вашей командой.</b>\n\
    \n\
    ⭐️ Он откроется, как только вы договоритесь командой и капитан начнет его!";

pub const PHOTO_TASK_ACCEPTED: StaticStr =
    "Отлично! Пока что фотография проверяется, а ты можешь двигаться дальше 📸";

pub const PLEASE_SEND_PHOTO: StaticStr =
    "☝🏻 Принимается именно фотография, а ты, кажется, скинул что-то не то!";

pub const PROMPT_ACCEPT_FINAL: StaticStr = "⭐️ <b>Основной этап «Легенд Бауманки» подходит к концу</b>\n\
    \n\
    🕰 Пришло время собраться вместе, чтобы перевернуть эту страницу и начать новую главу.\n\
    \n\
    💡 <b><i>Готов присоединиться к финалу?</i></b>";

pub const PROMPT_SLOT_TIME: StaticStr = "🕐 <b>Выберите время для финала</b>\n\
    \n\
    Пожалуйста, выберите удобное время, когда ваша команда сможет подойти на очный финал.\n\
    \n\
    ⏱ <i>Мероприятие займёт не более часа.</i>\n\
    \n\
    <i>Доступные слоты:</i>";

pub const INVALID_TEAM_PLACES: StaticStr = "❌ <b>Неверный формат</b>\n\
    \n\
    Пожалуйста, введите обычное число. Например: 5";

pub const PROMPT_CANCEL_RESERVATION_REASON: StaticStr = "❓ <b>Уточните причину отмены записи</b>
    \n\
    • 🗓 Хочу изменить время\n\
    • 👋 Не смогу прийти на финал\n\
    \n\
    💡 <i>Вы можете свободно перезаписываться на другое время до 21:00 сегодня.</i>";

pub const OCHEN_ZHAL: StaticStr = "👋 <b>Очень жаль, что вы не сможете быть с нами на финале</b>\n\
    \n\
    Мы будем скучать! Но полностью понимаем, что обстоятельства бывают разными.\n\
    \n\
    🗝 <b>Главное</b> — вы уже стали частью «Легенд Бауманки» и успешно прошли основной этап! Это огромное достижение ✨\n\
    \n\
    <i>Надеемся увидеть вас на других наших мероприятиях! Оставайтесь на связи!</i>";

pub fn my_team(team: TeamWithMembersDTO) -> String {
    let usernames_text = team
        .members
        .iter()
        .map(|member| {
            member
                .username
                .clone()
                .map(|u| u.to_string())
                .unwrap_or("(без никнейма)".to_string())
        })
        .fold(String::new(), |acc, username| {
            acc + format!("@{username}\n").as_str()
        });

    format!(
        "📊 <b>Информация о команде:</b>\n\
        • Название: {}\n\
        • Капитан: @{}\n\
        • Участники ({}/{}):\n\
        {}",
        team.name.as_str(),
        team.captain
            .username
            .map(|u| u.to_string())
            .unwrap_or("(без никнейма)".to_string()),
        team.size,
        team.max_size,
        usernames_text,
    )
}

pub fn media_uploaded(file_id: &FileID) -> String {
    format!(
        "✅ <b>FileID получен!</b>\n<code>{}</code>",
        file_id.as_str()
    )
}

pub fn character(character: CharacterDTO) -> String {
    let facts = character
        .facts
        .into_iter()
        .map(|f| format!("🔹 {}\n\n", f.as_str()))
        .fold(String::new(), |acc, s| acc + s.as_str());
    format!(
        "<b>{}</b>\n\
        \n\
        <blockquote>«{}»</blockquote>\n\
        \n\
        {facts}\
        <b><i>{}</i></b>
        ",
        character.name.as_str(),
        character.quote.as_str(),
        character.legacy.as_str(),
    )
}

pub fn track_description(track: &TrackDescriptionDTO) -> String {
    format!(
        "<b>{}</b>\n\
        \n\
        {}",
        track.tag.as_str().to_uppercase(),
        track.description.as_str()
    )
}

const CELLS: usize = 15;

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn track_menu(track: &TrackInProgressDTO) -> String {
    let timer_str = match track.status {
        TrackStatus::Started(start) => {
            format!(
                "Прошло с момента старта трека: {}",
                format_duration(Utc::now() - start)
            )
        }
        TrackStatus::Finished(start, finished) => {
            format!(
                "Время прохождения трека: {} - {} ({})",
                start.format("%d.%m %H:%M"),
                finished.format("%d.%m %H:%M"),
                format_duration(finished - start),
            )
        }
    };

    let width: usize = 15;
    let filled = (track.percent * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "<b>{}</b>\n\
        \n\
        {}\n\
        \n\
        <b>Твой прогресс</b>\n\
        Трек: <i>{}</i>\n\
        {}\n\
        \n\
        <b>✨ ─── ТАЙМЕР ─── ✨</b>\n\
        🕒 {}",
        track.tag.as_str().to_uppercase(),
        track.description.as_str(),
        track.tag.as_str(),
        format!(
            "{}{} {}%",
            "▰".repeat(filled),
            "▱".repeat(empty),
            (track.percent * 100.0) as usize
        ),
        timer_str,
    )
}

pub fn task_question_and_explanation(task: &TaskDTO) -> String {
    let answers = task
        .correct_answers
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{}\n\
        \n\
        <b>Правильный ответ: </b>{}\n\
        \n\
        {}",
        task.question.as_str(),
        answers,
        task.explanation.as_str()
    )
}

pub fn media_not_found(media_id: &MediaID) -> String {
    format!(
        "<b>❌ Медиафайл <code>{}</code> не найден</b>",
        media_id.as_str()
    )
}

pub fn prompt_slot_places(team_size: usize) -> String {
    format!(
        "👥 <b>Укажите число участников на финале</b>\n\
        Пожалуйста, введите, сколько человек из вашей команды планирует прийти на очный финал.\n\
        \n\
        💡 <i>В команде сейчас {team_size} участников.</i>"
    )
}

pub fn places_greater_than_team(team_size: usize) -> String {
    format!(
        "❌ <b>Слишком большое число</b>\n\
        \n\
        Количество участников на финале не может превышать размер вашей команды ({team_size} человек). Пожалуйста, введите корректное число."
    )
}

pub fn no_slots(team_size: usize) -> String {
    format!(
        "⏰ <b>В это время нет мест</b>\n\
        К сожалению, на выбранное время не осталось слотов для {team_size} человек.\n\
        \n\
        <i>Пожалуйста, выберите другое время для участия в финале.</i>"
    )
}

pub fn slot_successfully_reserved(slot: SlotDTO) -> String {
    format!(
        "Отлично! Запись подтверждена.\n\
        \n\
        ⏰ Финал пройдет 08 октября в {} в аудитории {} Конгресс-центра (вход B4 со стороны ГЗ)\n\
        \n\
        🪧 <b><i>Ждём вашу команду, чтобы вместе поставить красивую точку. До встречи!</i></b>",
        slot.start.format("%H:%M"),
        slot.site.as_str(),
    )
}
