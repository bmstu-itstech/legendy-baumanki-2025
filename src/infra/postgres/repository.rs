use deadpool_postgres::Pool;
use tokio_postgres::{Client, GenericClient};
use tokio_postgres::{Row, Transaction};

use crate::app::error::AppError;
use crate::app::ports::{
    IsRegisteredUserProvider, IsTeamExistsProvider, TeamByMemberProvider, TeamProvider,
    TeamRepository, UserProvider, UserRepository,
};
use crate::domain::error::DomainError;
use crate::domain::models::{FullName, GroupName, Team, TeamID, TeamName, User, UserID, Username};
use crate::{with_client, with_transaction};

pub struct PostgresRepository {
    pool: Pool,
}

impl PostgresRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

pub struct UserRow {
    id: i64,
    username: Option<String>,
    full_name: String,
    group_name: String,
}

impl UserRow {
    pub fn fetch_from_row(row: &Row) -> Result<UserRow, tokio_postgres::Error> {
        Ok(UserRow {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            full_name: row.try_get("full_name")?,
            group_name: row.try_get("group_name")?,
        })
    }
}

pub struct TeamRow {
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

#[async_trait::async_trait]
impl UserProvider for PostgresRepository {
    async fn user(&self, id: UserID) -> Result<User, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row = client
                .query_one(
                    r#"
                SELECT
                    id,
                    username,
                    full_name,
                    group_name
                FROM users
                WHERE 
                    id = $1
                "#,
                    &[&id.as_i64()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;

            let user_row =
                UserRow::fetch_from_row(&row).map_err(|err| AppError::Internal(err.into()))?;

            let username = user_row
                .username
                .map(|s| Username::new(s))
                .transpose()
                .map_err(AppError::DomainError)?;

            Ok(User::new(
                UserID::new(user_row.id),
                username,
                FullName::new(user_row.full_name).map_err(AppError::DomainError)?,
                GroupName::new(user_row.group_name).map_err(AppError::DomainError)?,
            ))
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
    async fn team(&self, id: TeamID) -> Result<Team, AppError> {
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
                return Err(AppError::DomainError(DomainError::TeamNotFound(
                    id.to_string(),
                )));
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
                TeamID::try_from(team_row.id).map_err(AppError::DomainError)?,
                TeamName::new(team_row.name).map_err(AppError::DomainError)?,
                UserID::new(team_row.captain_id),
                member_ids,
            )
            .map_err(AppError::DomainError)?;

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
                TeamID::try_from(team_row.id).map_err(AppError::DomainError)?,
                TeamName::new(team_row.name).map_err(AppError::DomainError)?,
                UserID::new(team_row.captain_id),
                member_ids,
            )
            .map_err(AppError::DomainError)?;

            Ok(Some(team))
        })
    }
}

#[async_trait::async_trait]
impl IsTeamExistsProvider for PostgresRepository {
    async fn is_team_exists(&self, team_id: &TeamID) -> Result<bool, AppError> {
        with_client!(self.pool, async |client: &Client| {
            let row = client
                .query_opt(
                    r#"
                SELECT 1
                FROM teams
                WHERE
                    id = $1
                LIMIT 1
                "#,
                    &[&team_id.as_str()],
                )
                .await
                .map_err(|err| AppError::Internal(err.into()))?;
            Ok(row.is_some())
        })
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresRepository {
    async fn save(&self, user: User) -> Result<(), AppError> {
        with_client!(self.pool, async |client: &Client| {
            client
                .execute(
                    r#"
                INSERT INTO 
                    users (
                        id,
                        username,
                        full_name,
                        group_name
                    )
                VALUES
                    ($1, $2, $3, $4)
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
            Ok(())
        })
    }
}

#[async_trait::async_trait]
impl TeamRepository for PostgresRepository {
    async fn save(&self, team: Team) -> Result<(), AppError> {
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

    async fn delete(&self, team_id: &TeamID) -> Result<(), AppError> {
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
