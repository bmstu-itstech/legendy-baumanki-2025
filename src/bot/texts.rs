use crate::app::usecases::dto::{CharacterDTO, Profile, TeamDTO, TeamWithMembersDTO, UserTaskDTO};
use crate::domain::models::{FileID, TeamName};

type StaticStr = &'static str;

pub const ENTER_MESSAGE_TEXT: StaticStr = "📝 Напиши мне текстовое сообщение";

pub const USE_KEYBOARD: StaticStr =
    "Пожалуйста, используй кнопки клавиатуры внизу для ответа. Так будет удобнее!";

pub const PERMISSION_DENIED: StaticStr = "<b>❌ Доступ запрещен</b>\n\
    У вас недостаточно прав для использования этой команды.";

pub const INVALID_INVITE_CODE: StaticStr = "❌ <b>Ой-ой!</b>\n\
     Этот код-приглашения не подходит. Проверь, что ввел все 6 символов правильно, и \
     попробуй еще раз.";

pub const TEAM_NOT_EXISTS: StaticStr = "❌ <b>Ой-ой!</b>\n\
     Кажется, эта ссылка больше не действительна. Запроси у капитана новую и попробуй ещё раз.";

pub const ALREADY_IN_THIS_TEAM: StaticStr = "🤝 <b>Ты уже с нами!</b>\n\
     Твое место в этой команде уже зарезервировано";

pub const ALREADY_IN_OTHER_TEAM: StaticStr = "🔄 <b>Сначала нужно выйти</b>\n\
     Ты состоишь в другой команде. Чтобы присоединиться к новой, сначала выйди из текущей \
     (все твои баллы сохранятся). После этого перейди по ссылке-приглашению заново.";

pub const GREETING_MSG: StaticStr = "🏛 <b>Добро пожаловать в «Легенды Бауманки» — главное приключение этого семестра для \
     первокурсников! Настало время отправиться в уникальное путешествие, которое стало настоящей \
     традицией для новых студентов нашего Университета. Если, конечно, ты готов принять вызов...</b>\n\
     \n\
     <b>«Легенды Бауманки»</b> — это погружение в мир бауманских традиций, история которых \
     оживает в каждой загадке. Здесь ты сможешь проявить интеллект и находчивость, объединиться \
     с командой единомышленников и, возможно, вписать своё имя в новую главу легенд Университета.\n\
     \n\
     📱 Все это станет возможным благодаря твоему желанию и этому боту — здесь ты будешь получать \
     задания, подсказки, узнавать интересные факты и многое-многое другое, что поможет тебе \
     разобраться в этой запутанной истории.\n\
     \n\
     🌟 Напоминаем, что ты можешь зарегистрироваться один или же собрать команду — выбор за тобой. \
     Правила, инструкции и подробности о том, как присоединиться к нашему загадочному \
     приключению, уже ждут тебя на картинках выше.\n\
     \n\
     <i>Разгадывай загадки, зарабатывай баллы и узнавай все больше секретов, таящихся в стенах \
     Университета. До встречи!🤍</i>";

pub const PROMPT_PD_AGREEMENT: StaticStr = "📄 <b>Подтверждение данных</b>\n\
    Для участия в квесте нам требуется твое согласие на обработку персональных данных в \
    соответствии с Федеральным законом №152-ФЗ.";

pub const PD_AGREEMENT_IS_REQUIRED: StaticStr = "⚠️ <b>Требуется подтверждение</b>\n\
     Пожалуйста, подтверди свое согласие на обработку персональных данных, чтобы мы могли \
     продолжить регистрацию.";

pub const PROMPT_FULL_NAME: StaticStr = "✏️ <b>Представься, пожалуйста!</b>\n\
     Введи свои Фамилию, Имя и Отчество полностью.\n\
     <i>Например: Иванов Иван Иванович</i>";

pub const INVALID_FULL_NAME: StaticStr = "❌ <b>Что-то не так с ФИО</b>\n\
     Кажется, мы не смогли распознать твое ФИО. Пожалуйста, проверь написание и попробуй ввести \
     еще раз.";

pub const ENTER_GROUP_NAME: StaticStr = "✏️ <b>Твоя учебная группа</b>\n\
     Теперь введи номер своей учебной группы. Будь внимателен к формату!\n\
     <b>*Пример: ИУ13-11Б*</b>\n\
     💡 <i>Важно: участие в квесте доступно только для студентов первого курса</i>";

pub const INVALID_GROUP_NAME: StaticStr = "❌ <b>Неверный формат группы</b>\n\
     Мне кажется, такой группы мы не существует. Пожалуйста, введи номер строго в том формате, \
     который указан в примере выше.";

pub const NOT_FIRST_COURSE: StaticStr = "😢 <b>К сожалению, ты не проходишь</b>\n\
    Участие в «Легендах Бауманки» доступно только для студентов первого курса. Ждем тебя на других \
    мероприятиях Студенческого совета!";

pub const SUCCESSFUL_REGISTRATION: StaticStr = "🎉 <b>Регистрация завершена!</b>\n\
     Ты успешно зарегистрировался(ась) на квест «Легенды Бауманки»!";

pub const MENU_TEXT: StaticStr = "📲 <b>Главное меню</b>\n\
    Выбери нужный пункт из меню ниже.";

pub const UNKNOWN_MENU_OPTION: StaticStr = "❌ <b>Непонятная команда</b>\n\
     К сожалению, я не понимаю этот запрос. Пожалуйста, выбери одну из доступных опций в меню.";

pub const PROMPT_TEAM_CODE: StaticStr = "🔑 <b>Введи код-приглашение</b>\n\
     Чтобы присоединиться к команде, введи 6-значный код, который тебе дал капитан.";

pub const TEAM_IS_FULL: StaticStr = "🚪 <b>В команде нет мест</b>\n\
     В этой команде уже максимальное количество участников (8 человек). Создай новую или найди \
     другую команду.";

pub const TEAM_NOT_FOUND: StaticStr = "🔍 <b>Команда не найдена</b>\n\
     К сожалению, команды с таким кодом не существует. Проверь, правильно ли ты ввёл(а) 6-значный \
     код, и попробуй ещё раз.\n\
     💡 <i>Код можно получить у капитана команды</i>";

pub const PROMPT_TEAM_NAME: StaticStr = "🏷 <b>Придумай название для своей команды</b>\n\
     Дай ей крутое и запоминающееся название!";

pub const INVALID_TEAM_NAME: StaticStr = "❌ <b>Название не подходит</b>\n\
     Такое название нельзя использовать. Попробуй придумать другое.";

pub const INTERNAL_ERROR: StaticStr = "⚠️ <b>Неизвестная ошибка</b>\n\
     Произошла непредвиденная ошибка. Пожалуйста, попробуй повторить действие позже или сообщи \
     об этом организаторам (см. описание бота).";

pub const SEND_APPROVAL_EXIT_TEAM: StaticStr =
    "❓ Ты действительно хочешь выйти из своей текущей команды?";

pub const SUCCESSFUL_EXIT_TEAM: StaticStr = "👋 <b>Ты вышел из команды</b>\n\
     Ты успешно покинул(а) команду. Теперь ты можешь вступить в новую, используя код-приглашение.";

pub const INVALID_UPLOAD_COMMAND_USAGE: StaticStr = "<b>❌ Неверный формат команды</b>\n\
    Использование:\n\
    <code>/upload key</code>";

pub const PROMPT_MEDIA: StaticStr = "📤 <b>Загрузка файла</b>\n\
    Отправьте файл (изображение или видеосообщение) в чат, чтобы получить его FileID.";

pub const UNKNOWN_MEDIA_FORMAT: StaticStr = "❌ <b>Формат не поддерживается</b>
    Отправьте изображение или видеосообщение.";

pub const TASK_INCORRECT_ANSWER: StaticStr = "❕ <b>Упс…</b>\n\
    Ответ неверный, попробуй ещё раз!\n\
    \n\
    ❔ <i>Хорошенько подумай, иногда истина находится где-то совсем рядом!</i>";

pub const REBUS_ALREADY_SOLVED: StaticStr = "✅ <b>Ответ уже засчитан</b>\n\
    \n\
    Ты уже дал(а) правильный ответ на этот ребус! Молодец!\n\
    \n\
    <i>Можешь перейти к следующему заданию.</i>";

pub const RIDDLE_ALREADY_SOLVED: StaticStr = "✅ <b>Ответ уже засчитан</b>\n\
    \n\
    Ты уже дал(а) правильный ответ на эту загадку! Молодец!\n\
    \n\
    <i>Можешь перейти к следующему заданию.</i>";

pub fn successful_joined_team(team_name: TeamName) -> String {
    format!(
        "🎉 <b>Ты в команде!</b>\n\
         Поздравляем! Теперь ты участник команды «{}». Удачи на квесте!",
        team_name.as_str()
    )
}

pub const PROMPT_CHARACTER_NAME: StaticStr = "☺️ <b>Отличный выбор!</b>\n\
    \n\
    Познакомься с великими учёными, инженерами и конструкторами, которые начинали свой путь здесь, в стенах МГТУ им. Н.Э. Баумана.\n\
    \n\
    <b><i>Кого из них ты хочешь узнать лучше?</i></b>";

pub fn profile(profile: Profile) -> String {
    match profile.team_name {
        Some(team_name) => {
            format!(
                "📋 <b>Твой профиль:</b>\n\
                 • ФИО: {}\n\
                 • Группа: {}\n\
                 • Команда: {}",
                profile.full_name.as_str(),
                profile.group_name.as_str(),
                team_name.as_str()
            )
        }
        None => {
            format!(
                "📋 <b>Твой профиль:</b>\n\
                 • ФИО: {}\n\
                 • Группа: {}\n\
                 • Команда: ты не состоишь в команде.",
                profile.full_name.as_str(),
                profile.group_name.as_str(),
            )
        }
    }
}

pub fn my_team(team: TeamWithMembersDTO, invite_link: &str) -> String {
    let completed_str = if team.completed {
        "✅ Команда может участвовать в финале!"
    } else {
        "Недостаточно участников для участия в финале. Для участия в финале необходимо, чтобы команда состояла минимум из 5 участников!"
    };

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
        • Код: <code>{}</code>\n\
        • Ссылка-приглашение: {}\n\
        • Участники ({}/{}):\n\
        {}\n\
        • <i>{}</i>\n\
        ",
        team.name.as_str(),
        team.id.as_str(),
        invite_link,
        team.size,
        team.max_size,
        usernames_text,
        completed_str,
    )
}

pub fn team_created(team: TeamDTO, invite_link: &str) -> String {
    format!(
        "🎊 <b>Команда создана!</b>\n\
         Поздравляем! Твоя команда «{}» готова к квесту.\n\
         • Код для друзей: <code>{}</code>\n\
         • Ссылка-приглашение: {}\n\
         💡 <i>Важно: для выхода в финал в команде должно быть не меньше 5 участников. Зови друзей!</i>",
        team.name.as_str(),
        team.id.as_str(),
        invite_link,
    )
}

pub fn media_uploaded(file_id: &FileID) -> String {
    format!(
        "✅ <b>FileID получен!</b>\n<code>{}</code>",
        file_id.as_str()
    )
}

pub fn rebuses_menu_text(tasks: &[UserTaskDTO]) -> String {
    let completed = tasks
        .iter()
        .fold(0, |acc, task| acc + if task.solved { 1 } else { 0 });
    let total = tasks.len();
    let list = tasks
        .into_iter()
        .map(|t| {
            format!(
                "• Ребус #{} {}\n",
                t.index,
                if t.solved { "✅" } else { "⏳" }
            )
        })
        .fold(String::new(), |acc, s| acc + s.as_str());
    format!(
        "🔍 <b>Меню ребусов</b>\n\
        <i>Решено: {completed}/{total}</i>\n\
        \n\
        Вот список всех ребусов. Выбери номер, чтобы перейти к ребусу.\n\
        \n\
        Статус:\n\
        ✅ — решён\n\
        ⏳ — не решён\n\
        \n\
        {list}"
    )
}

pub fn riddle_menu_text(tasks: &[UserTaskDTO]) -> String {
    let completed = tasks
        .iter()
        .fold(0, |acc, task| acc + if task.solved { 1 } else { 0 });
    let total = tasks.len();
    let list = tasks
        .into_iter()
        .map(|t| {
            format!(
                "• Загадка #{} {}\n",
                t.index,
                if t.solved { "✅" } else { "⏳" }
            )
        })
        .fold(String::new(), |acc, s| acc + s.as_str());
    format!(
        "🔍 <b>Меню загадок</b>\n\
        <i>Решено: {completed}/{total}</i>\n\
        \n\
        Вот список всех загадок. Выбери номер, чтобы перейти к загадке.\n\
        \n\
        Статус:\n\
        ✅ — решена\n\
        ⏳ — не решена\n\
        \n\
        {list}"
    )
}

pub fn character(character: CharacterDTO) -> String {
    let facts = character.facts
        .into_iter()
        .map(|f| {
            format!("🔹 {}\n\n", f.as_str())
        })
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
