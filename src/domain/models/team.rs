use crate::domain::error::DomainError;
use crate::utils::short_uuid::new_short_uuid;
use serde::{Deserialize, Serialize};

use super::user::UserID;

const TEAM_ID_LENGTH: usize = 6; // Вероятность пересечения порядка 10^(-7) или 0.00001%

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamID(String);

impl TeamID {
    pub fn new() -> Self {
        Self(new_short_uuid(6))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<String> for TeamID {
    type Error = DomainError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != TEAM_ID_LENGTH {
            return Err(DomainError::InvalidValue(format!(
                "expected TeamID length = {}, got {}",
                TEAM_ID_LENGTH,
                s.len()
            )));
        }
        if !s.chars().into_iter().all(|c| c.is_alphanumeric()) {
            return Err(DomainError::InvalidValue(format!(
                "invalid TeamID: expected alphanumeric character, got {}",
                s
            )));
        }
        Ok(Self(s))
    }
}

#[derive(Debug, Clone)]
pub struct TeamName(String);

impl TeamName {
    pub fn new(s: String) -> Result<Self, DomainError> {
        if s == "" {
            return Err(DomainError::InvalidValue(
                "invalid TeamName: expected not empty string".to_string(),
            ));
        }
        Ok(Self(s.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub const MIN_COMPLETED_TEAM_SIZE: usize = 5;
pub const MAX_TEAM_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct Team {
    id: TeamID,
    name: TeamName,
    captain_id: UserID,
    member_ids: Vec<UserID>,
}

impl Team {
    pub fn new(name: TeamName, captain_id: UserID) -> Self {
        let id = TeamID::new();
        let member_ids = vec![captain_id];
        Self {
            id,
            name,
            captain_id,
            member_ids,
        }
    }

    pub fn restore(
        id: TeamID,
        name: TeamName,
        captain_id: UserID,
        member_ids: Vec<UserID>,
    ) -> Result<Self, DomainError> {
        if !member_ids
            .iter()
            .find(|&member_id| *member_id == captain_id)
            .is_some()
        {
            return Err(DomainError::InvalidValue(format!(
                "captain {:?} is not in team {:?}",
                captain_id,
                member_ids.as_slice()
            )));
        }
        if member_ids.len() > MAX_TEAM_SIZE {
            return Err(DomainError::InvalidValue(format!(
                "team is too large: {} > {}",
                member_ids.len(),
                MAX_TEAM_SIZE
            )));
        }
        Ok(Self {
            id,
            name,
            captain_id,
            member_ids,
        })
    }

    pub fn add_member(&mut self, member_id: UserID) -> Result<(), DomainError> {
        if self.member_ids.len() + 1 > MAX_TEAM_SIZE {
            return Err(DomainError::TeamIsFull(self.member_ids.len()));
        }
        if self.member_ids.contains(&member_id) {
            return Err(DomainError::UserAlreadyInTeam(member_id.as_i64()));
        }
        self.member_ids.push(member_id);
        Ok(())
    }

    pub fn remove_member(mut self, member_id: UserID) -> Result<Option<Self>, DomainError> {
        if !self.member_ids.contains(&member_id) {
            return Err(DomainError::UserNotFound(member_id.as_i64()));
        }
        if member_id == self.captain_id {
            if self.member_ids.len() > 1 {
                self.member_ids.retain(|id| *id != member_id);
                self.captain_id = *self.member_ids.first().unwrap();
                Ok(Some(self))
            } else {
                Ok(None)
            }
        } else {
            self.member_ids.retain(|id| *id != member_id);
            Ok(Some(self))
        }
    }

    pub fn id(&self) -> &TeamID {
        &self.id
    }

    pub fn name(&self) -> &TeamName {
        &self.name
    }

    pub fn captain_id(&self) -> UserID {
        self.captain_id
    }

    pub fn member_ids(&self) -> &Vec<UserID> {
        &self.member_ids
    }

    pub fn is_completed(&self) -> bool {
        self.member_ids.len() >= MIN_COMPLETED_TEAM_SIZE
    }
}
