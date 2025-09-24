use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;
use crate::utils::uuid::new_pseudo_uuid;
use crate::{not_empty_string_impl, pseudo_uuid_impl};

use super::user::UserID;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamID(String);
pseudo_uuid_impl!(TeamID, 6);

#[derive(Debug, Clone)]
pub struct TeamName(String);
not_empty_string_impl!(TeamName);

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
