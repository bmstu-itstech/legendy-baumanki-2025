use crate::domain::models::Points;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;
use crate::domain::models::{Answer, TaskID, TrackTag};
use crate::utils::uuid::new_pseudo_uuid;
use crate::{not_empty_string_impl, pseudo_uuid_impl};

use super::user::UserID;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamID(String);
pseudo_uuid_impl!(TeamID, 6);

#[derive(Debug, Clone)]
pub struct TeamName(String);
not_empty_string_impl!(TeamName);

pub const MAX_TEAM_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub enum TrackStatus {
    Started(DateTime<Utc>),
    Finished(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Clone)]
pub struct Team {
    id: TeamID,
    name: TeamName,
    captain_id: UserID,
    member_ids: Vec<UserID>,
    answers: HashMap<TaskID, Answer>,
    started_tracks: HashMap<TrackTag, TrackStatus>,
    hint_points: Points,
}

impl Team {
    pub fn restore(
        id: TeamID,
        name: TeamName,
        captain_id: UserID,
        member_ids: Vec<UserID>,
        answers: Vec<Answer>,
        started_tracks: HashMap<TrackTag, TrackStatus>,
        hint_points: Points,
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
        let answers_map = HashMap::from_iter(answers.into_iter().map(|a| (a.task_id(), a)));
        Ok(Self {
            id,
            name,
            captain_id,
            member_ids,
            answers: answers_map,
            started_tracks,
            hint_points,
        })
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

    // Страшный костыль, когда одиночные игроки это команды с одним игроком...
    pub fn is_solo(&self) -> bool {
        self.member_ids.len() == 1
    }
    
    pub fn available_tracks(&self) -> &'static [TrackTag] {
        if self.is_solo() {
            &[ TrackTag::Universitet ]
        } else {
            &[ TrackTag::Muzhestvo, TrackTag::Volya, TrackTag::Trud, TrackTag::Uporstvo ]
        }
    }
    
    pub fn hint_points(&self) -> Points {
        self.hint_points
    }
    
    pub fn answers(&self) -> Vec<&Answer> {
        self.answers.values().collect()
    }
    
    pub fn started_tracks(&self) -> &HashMap<TrackTag, TrackStatus> {
        &self.started_tracks
    }
    
    pub fn start_track(&mut self, tag: TrackTag) -> Result<(), DomainError> {
        if self.started_tracks.contains_key(&tag) {
            return Err(DomainError::TrackCanNotBeStarted(tag))
        }
        self.started_tracks.insert(tag, TrackStatus::Started(Utc::now()));
        Ok(())
    }
    
    pub fn finish_track(&mut self, tag: TrackTag) -> Result<(), DomainError> {
        match self.started_tracks.get(&tag) {
            None => Err(DomainError::TrackCanNotBeFinished(tag)),
            Some(TrackStatus::Finished(_, _)) => {
                Err(DomainError::TrackCanNotBeFinished(tag))
            },
            Some(TrackStatus::Started(start)) => {
                self.started_tracks.insert(tag, TrackStatus::Finished(start.clone(), Utc::now()));
                Ok(())
            }
        }
    }
    
    pub fn save_answer(&mut self, answer: Answer) {
        self.answers.insert(answer.task_id(), answer);
    }
    
    pub fn track_status(&self, tag: TrackTag) -> Result<&TrackStatus, DomainError> {
        self.started_tracks.get(&tag).ok_or(DomainError::TrackNotStarted(tag))
    }
    
    pub fn track_is_started(&self, tag: TrackTag) -> bool {
        self.started_tracks.contains_key(&tag)
    }
    
    pub fn is_captain(&self, user_id: UserID) -> bool {
        self.captain_id == user_id
    }
}
