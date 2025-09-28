use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use tokio_postgres::{Client, GenericClient};
use tokio_postgres::{Row, Transaction};

use crate::app::error::AppError;
use crate::app::ports::{
    CharactersProvider, FeedbackRepository, IsAdminProvider, IsRegisteredUserProvider,
    MediaProvider, MediaRepository, TaskProvider, TasksProvider, TeamByMemberProvider,
    TeamProvider, TeamRepository, UserProvider, UserRepository,
};
use crate::domain::models::{
    Answer, AnswerID, AnswerText, Character, CharacterFact, CharacterID, CharacterLegacy,
    CharacterName, CharacterQuote, CorrectAnswer, Feedback, FileID, FullName, GroupName,
    LevenshteinDistance, Media, MediaID, MediaType as DomainMediaType,
    ParticipationMode as DomainParticipationMode, Points, SerialNumber, Task, TaskID, TaskText,
    TaskType as DomainTaskType, Team, TeamID, TeamName, User, UserID, Username,
};
use crate::{with_client, with_transaction};

pub struct PostgresRepository {
    pool: Pool,
}

impl PostgresRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

struct UserRow {
    id: i64,
    username: Option<String>,
    full_name: String,
    group_name: String,
    mode: ParticipationMode,
    team_id: Option<String>,
}

impl UserRow {
    pub fn fetch_from_row(row: &Row) -> Result<UserRow, tokio_postgres::Error> {
        Ok(UserRow {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            full_name: row.try_get("full_name")?,
            group_name: row.try_get("group_name")?,
            mode: row.try_get("mode")?,
            team_id: row.try_get("team_id")?,
        })
    }
}

struct TeamRow {
    id: String,
    name: String,
    captain_id: i64,
}

impl TeamRow {
    pub fn fetch_from_row(row: &Row) -> Result<TeamRow, tokio_postgres::Error> {
        Ok(TeamRow {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            captain_id: row.try_get("captain_id")?,
        })
    }
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "media_type", rename_all = "snake_case")]
enum MediaType {
    Image,
    VideoNote,
}

impl Into<DomainMediaType> for MediaType {
    fn into(self) -> DomainMediaType {
        match self {
            MediaType::Image => DomainMediaType::Image,
            MediaType::VideoNote => DomainMediaType::VideoNote,
        }
    }
}

impl From<DomainMediaType> for MediaType {
    fn from(value: DomainMediaType) -> Self {
        match value {
            DomainMediaType::Image => MediaType::Image,
            DomainMediaType::VideoNote => MediaType::VideoNote,
        }
    }
}

struct MediaRow {
    id: String,
    file_id: String,
    media_type: MediaType,
}

impl MediaRow {
    pub fn fetch_from_row(row: &Row) -> Result<MediaRow, tokio_postgres::Error> {
        Ok(MediaRow {
            id: row.try_get("id")?,
            file_id: row.try_get("file_id")?,
            media_type: row.try_get("media_type")?,
        })
    }
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "task_type", rename_all = "snake_case")]
enum TaskType {
    Rebus,
    Riddle,
}

impl From<DomainTaskType> for TaskType {
    fn from(value: DomainTaskType) -> Self {
        match value {
            DomainTaskType::Rebus => TaskType::Rebus,
            DomainTaskType::Riddle => TaskType::Riddle,
        }
    }
}

impl Into<DomainTaskType> for TaskType {
    fn into(self) -> DomainTaskType {
        match self {
            TaskType::Rebus => DomainTaskType::Rebus,
            TaskType::Riddle => DomainTaskType::Riddle,
        }
    }
}

struct TaskRow {
    id: String,
    index: i32,
    task_type: TaskType,
    media_id: String,
    explanation: String,
    correct_answer: String,
    points: i32,
    max_levenshtein_distance: i32,
}

impl TaskRow {
    pub fn fetch_from_row(row: &Row) -> Result<TaskRow, tokio_postgres::Error> {
        Ok(TaskRow {
            id: row.try_get("id")?,
            index: row.try_get("index")?,
            task_type: row.try_get("task_type")?,
            media_id: row.try_get("media_id")?,
            explanation: row.try_get("explanation")?,
            correct_answer: row.try_get("correct_answer")?,
            points: row.try_get("points")?,
            max_levenshtein_distance: row.try_get("max_levenshtein_distance")?,
        })
    }
}

struct AnswerRow {
    id: String,
    task_id: String,
    text: String,
    points: i32,
    created_at: DateTime<Utc>,
}

impl AnswerRow {
    pub fn fetch_from_row(row: &Row) -> Result<AnswerRow, tokio_postgres::Error> {
        Ok(AnswerRow {
            id: row.try_get("id")?,
            task_id: row.try_get("task_id")?,
            text: row.try_get("text")?,
            points: row.try_get("points")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

struct CharacterRow {
    id: String,
    index: i32,
    name: String,
    quote: String,
    legacy: String,
    media_id: String,
}

impl CharacterRow {
    pub fn fetch_from_row(row: &Row) -> Result<CharacterRow, tokio_postgres::Error> {
        Ok(CharacterRow {
            id: row.try_get("id")?,
            index: row.try_get("index")?,
            name: row.try_get("name")?,
            quote: row.try_get("quote")?,
            legacy: row.try_get("legacy")?,
            media_id: row.try_get("media_id")?,
        })
    }
}

struct CharacterFactRow {
    character_id: String,
    fact: String,
}

impl CharacterFactRow {
    pub fn fetch_from_row(row: &Row) -> Result<CharacterFactRow, tokio_postgres::Error> {
        Ok(CharacterFactRow {
            character_id: row.try_get("character_id")?,
            fact: row.try_get("fact")?,
        })
    }
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "participation_mode", rename_all = "snake_case")]
enum ParticipationMode {
    Solo,
    WantTeam,
    Team,
}

impl From<DomainParticipationMode> for ParticipationMode {
    fn from(value: DomainParticipationMode) -> Self {
        match value {
            DomainParticipationMode::Solo => ParticipationMode::Solo,
            DomainParticipationMode::WantTeam => ParticipationMode::WantTeam,
            DomainParticipationMode::Team(_) => ParticipationMode::Team,
        }
    }
}

#[async_trait::async_trait]
impl UserProvider for PostgresRepository {
    async fn user(&self, id: UserID) -> Result<User, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row_opt = client
                .query_opt(
                    r#"
                    SELECT
                        id,
                        username,
                        full_name,
                        group_name,
                        mode,
                        team_id
                    FROM users
                    WHERE 
                        id = $1
                    "#,
                    &[&id.as_i64()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let user_row =
                    UserRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

                let username = user_row.username.map(|s| Username::new(s)).transpose()?;

                let rows = client
                    .query(
                        r#"
                        SELECT
                            id,
                            task_id,
                            text,
                            points,
                            created_at
                        FROM answers
                        WHERE
                            user_id = $1
                        "#,
                        &[&id.as_i64()],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut answers = Vec::new();
                for row in rows {
                    let row = AnswerRow::fetch_from_row(&row)
                        .map_err(|err| AppError::Internal(err.into()))?;
                    let answer = Answer::restore(
                        AnswerID::try_from(row.id)?,
                        TaskID::try_from(row.task_id)?,
                        AnswerText::new(row.text),
                        Points::new(row.points)?,
                        row.created_at,
                    );
                    answers.push(answer);
                }

                let mode = match user_row.mode {
                    ParticipationMode::Solo => DomainParticipationMode::Solo,
                    ParticipationMode::WantTeam => DomainParticipationMode::WantTeam,
                    ParticipationMode::Team => {
                        let team_id = user_row
                            .team_id
                            .expect("expected team_id if user in team mode");
                        DomainParticipationMode::Team(TeamID::try_from(team_id)?)
                    }
                };

                Ok(User::restore(
                    UserID::new(user_row.id),
                    username,
                    FullName::new(user_row.full_name)?,
                    GroupName::new(user_row.group_name)?,
                    answers,
                    mode,
                ))
            } else {
                Err(AppError::UserNotFound(id.as_i64()))
            }
        })
    }
}

#[async_trait::async_trait]
impl IsRegisteredUserProvider for PostgresRepository {
    async fn is_registered(&self, user_id: UserID) -> Result<bool, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row = client
                .query_opt(
                    r#"
                    SELECT 1
                    FROM users
                    WHERE
                        id = $1
                    LIMIT 1
                    "#,
                    &[&user_id.as_i64()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(row.is_some())
        })
    }
}

#[async_trait::async_trait]
impl TeamProvider for PostgresRepository {
    async fn team(&self, id: &TeamID) -> Result<Team, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row_opt = client
                .query_opt(
                    r#"
                    SELECT
                        id,
                        name,
                        captain_id
                    FROM teams
                    WHERE
                        id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let row = if let Some(row) = row_opt {
                row
            } else {
                return Err(AppError::TeamNotFound(id.to_string()));
            };

            let team_row =
                TeamRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

            let rows = client
                .query(
                    r#"
                    SELECT id
                    FROM users
                    WHERE
                        team_id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            let member_ids: Vec<i64> = rows
                .iter()
                .map(|row| row.try_get::<&str, i64>("id"))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let member_ids = member_ids
                .into_iter()
                .map(|int_id| UserID::new(int_id))
                .collect();

            let team = Team::restore(
                TeamID::try_from(team_row.id)?,
                TeamName::new(team_row.name)?,
                UserID::new(team_row.captain_id),
                member_ids,
            )?;

            Ok(team)
        })
    }
}

#[async_trait::async_trait]
impl TeamByMemberProvider for PostgresRepository {
    async fn team_by_member(&self, member_id: UserID) -> Result<Option<Team>, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row_opt = client
                .query_opt(
                    r#"
                    SELECT
                        t.id,
                        t.name,
                        t.captain_id
                    FROM teams t
                    LEFT JOIN
                        users u
                        ON u.team_id = t.id
                    WHERE u.id = $1
                    LIMIT 1
                    "#,
                    &[&member_id.as_i64()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let row = if let Some(row) = row_opt {
                row
            } else {
                return Ok(None);
            };

            let team_row =
                TeamRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

            let rows = client
                .query(
                    r#"
                    SELECT id
                    FROM users
                    WHERE
                        team_id = $1
                    "#,
                    &[&team_row.id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            let member_ids: Vec<i64> = rows
                .iter()
                .map(|row| row.try_get::<&str, i64>("id"))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let member_ids = member_ids
                .into_iter()
                .map(|int_id| UserID::new(int_id))
                .collect();

            let team = Team::restore(
                TeamID::try_from(team_row.id)?,
                TeamName::new(team_row.name)?,
                UserID::new(team_row.captain_id),
                member_ids,
            )?;

            Ok(Some(team))
        })
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresRepository {
    async fn save_user(&self, user: User) -> Result<(), AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            let mode = ParticipationMode::from(user.mode().clone());
            tx.execute(
                r#"
                INSERT INTO 
                    users (
                        id,
                        username,
                        full_name,
                        group_name,
                        mode
                    )
                VALUES
                    ($1, $2, $3, $4, $5)
                ON CONFLICT (id) DO UPDATE SET
                    username = $2, 
                    full_name = $3, 
                    group_name = $4,
                    mode = $5
                "#,
                &[
                    &user.id().as_i64(),
                    &user.username().clone().map(|u| u.to_string()),
                    &user.full_name().to_string(),
                    &user.group_name().to_string(),
                    &mode,
                ],
            )
            .await
            .map_err(|err| AppError::Internal(err.into()))?;

            tx.execute(
                r#"
                DELETE FROM answers
                WHERE
                    user_id = $1
                "#,
                &[&user.id().as_i64()],
            )
            .await
            .map_err(|err| AppError::Internal(err.into()))?;

            for answer in user.answers().values() {
                tx.execute(
                    r#"
                    INSERT INTO
                        answers (
                            id,
                            task_id,
                            user_id,
                            text,
                            points,
                            created_at
                        )
                    VALUES
                        ($1, $2, $3, $4, $5, $6)
                    "#,
                    &[
                        &answer.id().as_str(),
                        &answer.task_id().as_str(),
                        &user.id().as_i64(),
                        &answer.text().as_str(),
                        &answer.points().as_i32(),
                        &answer.created_at(),
                    ],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            }

            Ok::<(), AppError>(())
        })
    }
}

#[async_trait::async_trait]
impl TeamRepository for PostgresRepository {
    async fn save_team(&self, team: Team) -> Result<(), AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            tx.execute(
                r#"
                INSERT INTO
                    teams (
                        id,
                        name,
                        captain_id
                    )
                VALUES
                    ($1, $2, $3)
                ON CONFLICT (id) DO UPDATE SET
                    name = $2,
                    captain_id = $3
                "#,
                &[
                    &team.id().to_string(),
                    &team.name().to_string(),
                    &team.captain_id().as_i64(),
                ],
            )
            .await
            .map_err(|err| AppError::Internal(err.into()))?;

            // Удаляем все привязки участников в команде, чтобы потом заново их добавить.
            tx.execute(
                r#"
                UPDATE users
                SET
                    team_id = NULL
                WHERE
                    team_id = $1
                "#,
                &[&team.id().to_string()],
            )
            .await
            .map_err(|err| AppError::Internal(err.into()))?;

            for member_id in team.member_ids() {
                tx.execute(
                    r#"
                    UPDATE users
                    SET
                        team_id = $2
                    WHERE
                        id = $1
                    "#,
                    &[&member_id.as_i64(), &team.id().to_string()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            }

            Ok::<(), AppError>(())
        })
    }

    async fn delete_team(&self, team_id: &TeamID) -> Result<(), AppError> {
        with_client!(self.pool, async |client: &Client| {
            client
                .execute(
                    r#"
                    DELETE FROM teams
                    WHERE
                        id = $1
                    "#,
                    &[&team_id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(())
        })
    }
}

#[async_trait::async_trait]
impl MediaProvider for PostgresRepository {
    async fn media(&self, id: &MediaID) -> Result<Media, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row_opt = client
                .query_opt(
                    r#"
                    SELECT
                        id,
                        file_id,
                        media_type
                    FROM media
                    WHERE id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let media_row =
                    MediaRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;
                let media = Media::new(
                    MediaID::new(media_row.id)?,
                    FileID::new(media_row.file_id)?,
                    media_row.media_type.into(),
                );
                Ok(media)
            } else {
                Err(AppError::MediaNotFound(id.clone()))
            }
        })
    }
}

#[async_trait::async_trait]
impl MediaRepository for PostgresRepository {
    async fn save_media(&self, media: Media) -> Result<(), AppError> {
        let media_type: MediaType = media.media_type().into();
        with_client!(self.pool, async |client: &Client| {
            client
                .execute(
                    r#"
                    INSERT INTO 
                        media (
                            id,
                            file_id,
                            media_type
                        )
                    VALUES
                        ($1, $2, $3)
                    ON CONFLICT (id) DO UPDATE SET
                        file_id = $2, 
                        media_type = $3
                    "#,
                    &[&media.id().as_str(), &media.file_id().as_str(), &media_type],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(())
        })
    }
}

#[async_trait::async_trait]
impl IsAdminProvider for PostgresRepository {
    async fn is_admin(&self, user_id: UserID) -> Result<bool, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row = client
                .query_opt(
                    r#"
                    SELECT 1
                    FROM admins
                    WHERE
                        user_id = $1
                    LIMIT 1
                    "#,
                    &[&user_id.as_i64()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(row.is_some())
        })
    }
}

#[async_trait::async_trait]
impl TaskProvider for PostgresRepository {
    async fn task(&self, id: TaskID) -> Result<Task, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row_opt = client
                .query_opt(
                    r#"
                    SELECT
                        id,
                        index,
                        task_type,
                        media_id,
                        explanation,
                        correct_answer,
                        points,
                        max_levenshtein_distance
                    FROM tasks
                    WHERE
                        id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let task_row =
                    TaskRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;
                let task = Task::restore(
                    TaskID::try_from(task_row.id)?,
                    task_row.index as SerialNumber,
                    task_row.task_type.into(),
                    MediaID::new(task_row.media_id)?,
                    TaskText::new(task_row.explanation)?,
                    CorrectAnswer::new(task_row.correct_answer)?,
                    Points::new(task_row.points)?,
                    task_row.max_levenshtein_distance as LevenshteinDistance,
                );
                Ok(task)
            } else {
                Err(AppError::TaskNotFound(id))
            }
        })
    }
}

#[async_trait::async_trait]
impl TasksProvider for PostgresRepository {
    async fn tasks(&self, task_type: DomainTaskType) -> Result<Vec<Task>, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let task_type: TaskType = task_type.into();
            let rows = client
                .query(
                    r#"
                    SELECT
                        id,
                        index,
                        task_type,
                        media_id,
                        explanation,
                        correct_answer,
                        points,
                        max_levenshtein_distance
                    FROM tasks
                    WHERE
                        task_type = $1
                    ORDER BY index ASC
                    "#,
                    &[&task_type],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut tasks = Vec::new();
            for row in rows {
                let task_row =
                    TaskRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;
                let task = Task::restore(
                    TaskID::try_from(task_row.id)?,
                    task_row.index as SerialNumber,
                    task_row.task_type.into(),
                    MediaID::new(task_row.media_id)?,
                    TaskText::new(task_row.explanation)?,
                    CorrectAnswer::new(task_row.correct_answer)?,
                    Points::new(task_row.points)?,
                    task_row.max_levenshtein_distance as LevenshteinDistance,
                );
                tasks.push(task);
            }
            Ok(tasks)
        })
    }
}

#[async_trait::async_trait]
impl CharactersProvider for PostgresRepository {
    async fn characters(&self) -> Result<Vec<Character>, AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            let rows = tx
                .query(
                    r#"
                    SELECT
                        id,
                        index,
                        name,
                        quote,
                        legacy,
                        media_id
                    FROM characters
                    ORDER BY index ASC
                    "#,
                    &[],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut characters = Vec::new();
            for row in rows {
                let char_row = CharacterRow::fetch_from_row(&row)
                    .map_err(|err| AppError::Internal(err.into()))?;
                let char_id = CharacterID::try_from(char_row.id.clone())?;

                let fact_rows = tx
                    .query(
                        r#"
                        SELECT
                            character_id,
                            fact
                        FROM character_facts
                        WHERE 
                            character_id = $1
                        "#,
                        &[&char_id.as_str()],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut facts = Vec::new();
                for fact_row in fact_rows {
                    let fact_row = CharacterFactRow::fetch_from_row(&fact_row)
                        .map_err(|err| AppError::Internal(err.into()))?;
                    let fact = CharacterFact::new(fact_row.fact)?;
                    facts.push(fact);
                }

                let char = Character::restore(
                    char_id,
                    char_row.index as SerialNumber,
                    CharacterName::new(char_row.name)?,
                    CharacterQuote::new(char_row.quote)?,
                    facts,
                    CharacterLegacy::new(char_row.legacy)?,
                    MediaID::new(char_row.media_id)?,
                );
                characters.push(char);
            }
            Ok::<_, AppError>(characters)
        })
    }

    async fn character_by_name(&self, name: &CharacterName) -> Result<Option<Character>, AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            let row_opt = tx
                .query_opt(
                    r#"
                    SELECT
                        id,
                        index,
                        name,
                        quote,
                        legacy,
                        media_id
                    FROM characters
                    WHERE 
                        name = $1
                    "#,
                    &[&name.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let char_row = CharacterRow::fetch_from_row(&row)
                    .map_err(|err| AppError::Internal(err.into()))?;

                let fact_rows = tx
                    .query(
                        r#"
                        SELECT
                            character_id,
                            fact
                        FROM character_facts
                        WHERE 
                            character_id = $1
                        "#,
                        &[&char_row.id],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut facts = Vec::new();
                for fact_row in fact_rows {
                    let fact_row = CharacterFactRow::fetch_from_row(&fact_row)
                        .map_err(|err| AppError::Internal(err.into()))?;
                    let fact = CharacterFact::new(fact_row.fact)?;
                    facts.push(fact);
                }

                let char = Character::restore(
                    CharacterID::try_from(char_row.id)?,
                    char_row.index as SerialNumber,
                    CharacterName::new(char_row.name)?,
                    CharacterQuote::new(char_row.quote)?,
                    facts,
                    CharacterLegacy::new(char_row.legacy)?,
                    MediaID::new(char_row.media_id)?,
                );
                Ok::<_, AppError>(Some(char))
            } else {
                Ok(None)
            }
        })
    }
}

#[async_trait::async_trait]
impl FeedbackRepository for PostgresRepository {
    async fn save_feedback(&self, feedback: Feedback) -> Result<(), AppError> {
        with_client!(self.pool, async |client: &Client| {
            client
                .execute(
                    r#"
                INSERT INTO feedbacks (
                    author_id,
                    text
                )
                VALUES
                    ($1, $2)
                "#,
                    &[&feedback.author_id().as_i64(), &feedback.text().as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(())
        })
    }
}
