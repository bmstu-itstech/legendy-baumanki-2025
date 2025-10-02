use crate::domain::models::{
    Answer, AnswerText, CorrectAnswer, Points, Task, TaskID, TaskText, TrackStatus,
};
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use std::collections::HashMap;
use tokio_postgres::{Client, GenericClient};
use tokio_postgres::{Row, Transaction};

use crate::app::error::AppError;
use crate::app::ports::{
    CharactersProvider, FeedbackRepository, IsAdminProvider, IsRegisteredUserProvider,
    MediaProvider, MediaRepository, TaskProvider, TeamByMemberProvider, TeamProvider,
    TeamRepository, TrackProvider, UserProvider, UserRepository,
};
use crate::app::usecases::AnswerTask;
use crate::domain::models::{
    Character, CharacterFact, CharacterID, CharacterLegacy, CharacterName, CharacterQuote,
    Feedback, FileID, FullName, GroupName, Media, MediaID, MediaType as DomainMediaType,
    SerialNumber, TaskOption, TaskType as DomainTaskType, Team, TeamID, TeamName, Track,
    TrackDescription, TrackTag as DomainTrackTag, User, UserID, Username,
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
    team_id: Option<String>,
}

impl UserRow {
    pub fn fetch_from_row(row: &Row) -> Result<UserRow, tokio_postgres::Error> {
        Ok(UserRow {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            full_name: row.try_get("full_name")?,
            group_name: row.try_get("group_name")?,
            team_id: row.try_get("team_id")?,
        })
    }
}

struct TeamRow {
    id: String,
    name: String,
    captain_id: i64,
    hint_points: i32,
}

impl TeamRow {
    pub fn fetch_from_row(row: &Row) -> Result<TeamRow, tokio_postgres::Error> {
        Ok(TeamRow {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            captain_id: row.try_get("captain_id")?,
            hint_points: row.try_get("hint_points")?,
        })
    }
}

struct AnswerRow {
    team_id: String,
    task_id: i32,
    text: String,
    points: i32,
    created_at: DateTime<Utc>,
}

impl AnswerRow {
    pub fn fetch_from_row(row: &Row) -> Result<AnswerRow, tokio_postgres::Error> {
        Ok(Self {
            team_id: row.try_get("team_id")?,
            task_id: row.try_get("task_id")?,
            text: row.try_get("text")?,
            points: row.try_get("points")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

struct TeamStartedTrackRow {
    team_id: String,
    track_tag: TrackTag,
    started_at: DateTime<Utc>,
    finished_at: Option<DateTime<Utc>>,
}

impl TeamStartedTrackRow {
    pub fn fetch_from_row(row: &Row) -> Result<TeamStartedTrackRow, tokio_postgres::Error> {
        Ok(Self {
            team_id: row.try_get("team_id")?,
            track_tag: row.try_get("track_tag")?,
            started_at: row.try_get("started_at")?,
            finished_at: row.try_get("finished_at")?,
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
#[postgres(name = "track_tag", rename_all = "snake_case")]
enum TrackTag {
    Muzhestvo,
    Volya,
    Trud,
    Uporstvo,
    Universitet,
}

impl From<DomainTrackTag> for TrackTag {
    fn from(v: DomainTrackTag) -> Self {
        match v {
            DomainTrackTag::Muzhestvo => Self::Muzhestvo,
            DomainTrackTag::Volya => Self::Volya,
            DomainTrackTag::Trud => Self::Trud,
            DomainTrackTag::Uporstvo => Self::Uporstvo,
            DomainTrackTag::Universitet => Self::Universitet,
        }
    }
}

impl Into<DomainTrackTag> for TrackTag {
    fn into(self) -> DomainTrackTag {
        match self {
            Self::Muzhestvo => DomainTrackTag::Muzhestvo,
            Self::Volya => DomainTrackTag::Volya,
            Self::Trud => DomainTrackTag::Trud,
            Self::Uporstvo => DomainTrackTag::Uporstvo,
            Self::Universitet => DomainTrackTag::Universitet,
        }
    }
}

struct TrackRow {
    tag: TrackTag,
    description: String,
    media_id: String,
}

impl TrackRow {
    pub fn fetch_from_row(row: &Row) -> Result<TrackRow, tokio_postgres::Error> {
        Ok(Self {
            tag: row.try_get("tag")?,
            description: row.try_get("description")?,
            media_id: row.try_get("media_id")?,
        })
    }
}

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "task_type", rename_all = "snake_case")]
enum TaskType {
    Text,
    Choice,
    Photo,
}

impl From<DomainTaskType> for TaskType {
    fn from(v: DomainTaskType) -> Self {
        match v {
            DomainTaskType::Text => TaskType::Text,
            DomainTaskType::Choice => TaskType::Choice,
            DomainTaskType::Photo => TaskType::Photo,
        }
    }
}

impl Into<DomainTaskType> for TaskType {
    fn into(self) -> DomainTaskType {
        match self {
            TaskType::Text => DomainTaskType::Text,
            TaskType::Choice => DomainTaskType::Choice,
            TaskType::Photo => DomainTaskType::Photo,
        }
    }
}

struct TaskRow {
    id: i32,
    task_type: TaskType,
    question: String,
    explanation: String,
    media_id: Option<String>,
    points: i32,
    price: i32,
    max_lvnsht_d: i32,
}

impl TaskRow {
    pub fn fetch_from_row(row: &Row) -> Result<TaskRow, tokio_postgres::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            task_type: row.try_get("task_type")?,
            question: row.try_get("question")?,
            explanation: row.try_get("explanation")?,
            media_id: row.try_get("media_id")?,
            points: row.try_get("points")?,
            price: row.try_get("price")?,
            max_lvnsht_d: row.try_get("max_lvnsht_d")?,
        })
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

                Ok(User::new(
                    UserID::new(user_row.id),
                    username,
                    FullName::new(user_row.full_name)?,
                    GroupName::new(user_row.group_name)?,
                    user_row.team_id.map(|s| TeamID::try_from(s)).transpose()?,
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
                        captain_id,
                        hint_points
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

            let rows = client
                .query(
                    r#"
                    SELECT
                        team_id,
                        task_id,
                        text,
                        points,
                        created_at
                    FROM answers
                    WHERE
                        team_id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let answer_rows = rows
                .iter()
                .map(|row| AnswerRow::fetch_from_row(&row))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut answers = Vec::new();
            for row in answer_rows {
                let answer = Answer::restore(
                    row.task_id,
                    AnswerText::new(row.text),
                    Points::new(row.points)?,
                    row.created_at,
                );
                answers.push(answer);
            }

            let rows = client
                .query(
                    r#"
                    SELECT
                        team_id,
                        track_tag,
                        started_at,
                        finished_at
                    FROM team_started_tracks
                    WHERE
                        team_id = $1
                    "#,
                    &[&id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let started_track_rows = rows
                .iter()
                .map(|row| TeamStartedTrackRow::fetch_from_row(&row))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut started_tracks = HashMap::new();
            for row in started_track_rows {
                let track_status = if let Some(finished_at) = row.finished_at {
                    TrackStatus::Finished(row.started_at, finished_at)
                } else {
                    TrackStatus::Started(row.started_at)
                };
                let tag: DomainTrackTag = row.track_tag.into();
                started_tracks.insert(tag, track_status);
            }

            let team = Team::restore(
                TeamID::try_from(team_row.id)?,
                TeamName::new(team_row.name)?,
                UserID::new(team_row.captain_id),
                member_ids,
                answers,
                started_tracks,
                Points::new(team_row.hint_points)?,
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
                        t.captain_id,
                        t.hint_points
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

            let rows = client
                .query(
                    r#"
                    SELECT
                        team_id,
                        task_id,
                        text,
                        points,
                        created_at
                    FROM answers
                    WHERE
                        team_id = $1
                    "#,
                    &[&team_row.id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let answer_rows = rows
                .iter()
                .map(|row| AnswerRow::fetch_from_row(&row))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut answers = Vec::new();
            for row in answer_rows {
                let answer = Answer::restore(
                    row.task_id,
                    AnswerText::new(row.text),
                    Points::new(row.points)?,
                    row.created_at,
                );
                answers.push(answer);
            }

            let rows = client
                .query(
                    r#"
                    SELECT
                        team_id,
                        track_tag,
                        started_at,
                        finished_at
                    FROM team_started_tracks
                    WHERE
                        team_id = $1
                    "#,
                    &[&team_row.id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let started_track_rows = rows
                .iter()
                .map(|row| TeamStartedTrackRow::fetch_from_row(&row))
                .collect::<Result<Vec<_>, tokio_postgres::Error>>()
                .map_err(|err| AppError::Internal(err.into()))?;

            let mut started_tracks = HashMap::new();
            for row in started_track_rows {
                let track_status = if let Some(finished_at) = row.finished_at {
                    TrackStatus::Finished(row.started_at, finished_at)
                } else {
                    TrackStatus::Started(row.started_at)
                };
                let tag: DomainTrackTag = row.track_tag.into();
                started_tracks.insert(tag, track_status);
            }

            let team = Team::restore(
                TeamID::try_from(team_row.id)?,
                TeamName::new(team_row.name)?,
                UserID::new(team_row.captain_id),
                member_ids,
                answers,
                started_tracks,
                Points::new(team_row.hint_points)?,
            )?;

            Ok(Some(team))
        })
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresRepository {
    async fn save_user(&self, user: User) -> Result<(), AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            tx.execute(
                r#"
                INSERT INTO 
                    users (
                        id,
                        username,
                        full_name,
                        group_name
                    )
                VALUES
                    ($1, $2, $3, $4, $5)
                ON CONFLICT (id) DO UPDATE SET
                    username = $2, 
                    full_name = $3, 
                    group_name = $4
                "#,
                &[
                    &user.id().as_i64(),
                    &user.username().clone().map(|u| u.to_string()),
                    &user.full_name().to_string(),
                    &user.group_name().to_string(),
                ],
            )
            .await
            .map_err(|err| AppError::Internal(err.into()))?;

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

            for (&track_tag, track_status) in team.started_tracks() {
                match track_status {
                    TrackStatus::Started(started_at) => {
                        tx.execute(
                            r#"
                            INSERT INTO
                                team_started_tracks (
                                    team_id,
                                    track_tag,
                                    started_at
                                )
                            VALUES
                                ($1, $2, $3)
                            ON CONFLICT (team_id, track_tag)
                            DO NOTHING
                            "#,
                            &[&team.id().as_str(), &TrackTag::from(track_tag), &started_at],
                        )
                        .await
                        .map_err(|err| AppError::Internal(err.into()))?;
                    }
                    TrackStatus::Finished(started_at, finished_at) => {
                        tx.execute(
                            r#"
                            INSERT INTO
                                team_started_tracks (
                                    team_id,
                                    track_tag,
                                    started_at,
                                    finished_at
                                )
                            VALUES
                                ($1, $2, $3, $4)
                            ON CONFLICT (team_id, track_tag)
                            DO UPDATE SET
                                finished_at = $4
                            "#,
                            &[
                                &team.id().as_str(),
                                &TrackTag::from(track_tag),
                                &started_at,
                                &finished_at,
                            ],
                        )
                        .await
                        .map_err(|err| AppError::Internal(err.into()))?;
                    }
                }
            }

            for answer in team.answers() {
                tx.execute(
                    r#"
                    INSERT INTO
                        answers (
                            team_id,
                            task_id,
                            text,
                            points,
                            created_at
                        )
                    VALUES
                        ($1, $2, $3, $4, $5)
                    ON CONFLICT (team_id, task_id)
                    DO UPDATE SET
                        text = $3,
                        points = $4,
                        created_at = $5
                    "#,
                    &[
                        &team.id().as_str(),
                        &answer.task_id(),
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

#[async_trait::async_trait]
impl TrackProvider for PostgresRepository {
    async fn track(&self, domain_tag: DomainTrackTag) -> Result<Track, AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            let tag = TrackTag::from(domain_tag);
            let row_opt = tx
                .query_opt(
                    r#"
                    SELECT
                        tag,
                        description,
                        media_id
                    FROM tracks
                    WHERE
                        tag = $1
                    "#,
                    &[&tag],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let track_row =
                    TrackRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

                let rows = tx
                    .query(
                        r#"
                        SELECT
                            id,
                            task_type,
                            question,
                            explanation,
                            media_id,
                            points,
                            price,
                            max_lvnsht_d
                        FROM tasks
                        WHERE
                            track_tag = $1
                        "#,
                        &[&tag],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut tasks = Vec::new();
                for row in rows {
                    let task_row = TaskRow::fetch_from_row(&row)
                        .map_err(|err| AppError::Internal(err.into()))?;

                    let option_rows = tx
                        .query(
                            r#"
                            SELECT option
                            FROM task_options
                            WHERE task_id = $1
                            "#,
                            &[&task_row.id],
                        )
                        .await
                        .map_err(|err| AppError::Internal(err.into()))?;

                    let mut options = Vec::new();
                    for option_row in option_rows {
                        let option = TaskOption::new(
                            option_row
                                .try_get("option")
                                .map_err(|err| AppError::Internal(err.into()))?,
                        )?;
                        options.push(option);
                    }

                    let dependencies_rows = tx
                        .query(
                            r#"
                            SELECT dependency
                            FROM   task_dependencies
                            WHERE  task_id = $1
                            "#,
                            &[&task_row.id],
                        )
                        .await
                        .map_err(|err| AppError::Internal(err.into()))?;

                    let mut dependencies = Vec::new();
                    for dependencies_row in dependencies_rows {
                        let dependency: i32 = dependencies_row
                            .try_get("dependency")
                            .map_err(|err| AppError::Internal(err.into()))?;
                        dependencies.push(dependency as TaskID);
                    }

                    let correct_answer_rows = tx
                        .query(
                            r#"
                            SELECT answer
                            FROM   task_correct_answers
                            WHERE  task_id = $1
                            "#,
                            &[&task_row.id],
                        )
                        .await
                        .map_err(|err| AppError::Internal(err.into()))?;

                    let mut correct_answers = Vec::new();
                    for row in correct_answer_rows {
                        let text = row
                            .try_get("answer")
                            .map_err(|err| AppError::Internal(err.into()))?;
                        let answer = CorrectAnswer::new(text)?;
                        correct_answers.push(answer);
                    }

                    let task = Task::new(
                        task_row.id as TaskID,
                        task_row.task_type.into(),
                        TaskText::new(task_row.question)?,
                        TaskText::new(task_row.explanation)?,
                        task_row.media_id.map(|m| MediaID::new(m)).transpose()?,
                        options,
                        dependencies,
                        correct_answers,
                        Points::new(task_row.points)?,
                        Points::new(task_row.price)?,
                        task_row.max_lvnsht_d as usize,
                    );
                    tasks.push(task);
                }

                let track = Track::new(
                    domain_tag,
                    TrackDescription::new(track_row.description)?,
                    MediaID::new(track_row.media_id)?,
                    tasks,
                );

                Ok::<_, AppError>(track)
            } else {
                Err(AppError::TrackNotFound(domain_tag))
            }
        })
    }
}

#[async_trait::async_trait]
impl TaskProvider for PostgresRepository {
    async fn task(&self, task_id: TaskID) -> Result<Task, AppError> {
        with_transaction!(self.pool, async |tx: &Transaction| {
            let row_opt = tx
                .query_opt(
                    r#"
                    SELECT
                        id,
                        task_type,
                        question,
                        explanation,
                        media_id,
                        points,
                        price,
                        max_lvnsht_d
                    FROM tasks
                    WHERE
                        id = $1
                    "#,
                    &[&task_id],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            if let Some(row) = row_opt {
                let task_row =
                    TaskRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

                let option_rows = tx
                    .query(
                        r#"
                        SELECT option
                        FROM task_options
                        WHERE task_id = $1
                        "#,
                        &[&task_row.id],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut options = Vec::new();
                for option_row in option_rows {
                    let option = TaskOption::new(
                        option_row
                            .try_get("option")
                            .map_err(|err| AppError::Internal(err.into()))?,
                    )?;
                    options.push(option);
                }

                let dependencies_rows = tx
                    .query(
                        r#"
                            SELECT dependency
                            FROM   task_dependencies
                            WHERE  task_id = $1
                            "#,
                        &[&task_row.id],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut dependencies = Vec::new();
                for dependencies_row in dependencies_rows {
                    let dependency: i32 = dependencies_row
                        .try_get("dependency")
                        .map_err(|err| AppError::Internal(err.into()))?;
                    dependencies.push(dependency as TaskID);
                }

                let correct_answer_rows = tx
                    .query(
                        r#"
                            SELECT answer
                            FROM   task_correct_answers
                            WHERE  task_id = $1
                            "#,
                        &[&task_row.id],
                    )
                    .await
                    .map_err(|err| AppError::Internal(err.into()))?;

                let mut correct_answers = Vec::new();
                for row in correct_answer_rows {
                    let text = row
                        .try_get("answer")
                        .map_err(|err| AppError::Internal(err.into()))?;
                    let answer = CorrectAnswer::new(text)?;
                    correct_answers.push(answer);
                }

                let task = Task::new(
                    task_row.id as TaskID,
                    task_row.task_type.into(),
                    TaskText::new(task_row.question)?,
                    TaskText::new(task_row.explanation)?,
                    task_row.media_id.map(|m| MediaID::new(m)).transpose()?,
                    options,
                    dependencies,
                    correct_answers,
                    Points::new(task_row.points)?,
                    Points::new(task_row.price)?,
                    task_row.max_lvnsht_d as usize,
                );
                Ok(task)
            } else {
                Err(AppError::TaskNotFound(task_id))
            }
        })
    }
}
