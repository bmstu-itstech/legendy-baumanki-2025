use crate::domain::error::DomainError;
use crate::domain::models::MediaID;
use crate::utils::uuid::new_pseudo_uuid;
use crate::{not_empty_string_impl, pseudo_uuid_impl};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterID(String);
pseudo_uuid_impl!(CharacterID, 4);

#[derive(Debug, Clone)]
pub struct CharacterName(String);
not_empty_string_impl!(CharacterName);

#[derive(Clone)]
pub struct CharacterQuote(String);
not_empty_string_impl!(CharacterQuote);

#[derive(Clone)]
pub struct CharacterFact(String);
not_empty_string_impl!(CharacterFact);

#[derive(Clone)]
pub struct CharacterLegacy(String);
not_empty_string_impl!(CharacterLegacy);

pub type SerialNumber = u32;

pub struct Character {
    id: CharacterID,
    index: SerialNumber,
    name: CharacterName,
    quote: CharacterQuote,
    facts: Vec<CharacterFact>,
    legacy: CharacterLegacy,
    media_id: MediaID,
}

impl Character {
    pub fn new(
        name: CharacterName,
        index: SerialNumber,
        quote: CharacterQuote,
        facts: Vec<CharacterFact>,
        legacy: CharacterLegacy,
        media_id: MediaID,
    ) -> Self {
        Self {
            id: CharacterID::new(),
            index,
            name,
            quote,
            facts,
            legacy,
            media_id,
        }
    }

    pub fn restore(
        id: CharacterID,
        index: SerialNumber,
        name: CharacterName,
        quote: CharacterQuote,
        facts: Vec<CharacterFact>,
        legacy: CharacterLegacy,
        media_id: MediaID,
    ) -> Self {
        Self {
            id,
            index,
            name,
            quote,
            facts,
            legacy,
            media_id,
        }
    }

    pub fn id(&self) -> &CharacterID {
        &self.id
    }

    pub fn index(&self) -> SerialNumber {
        self.index
    }

    pub fn name(&self) -> &CharacterName {
        &self.name
    }

    pub fn quote(&self) -> &CharacterQuote {
        &self.quote
    }

    pub fn facts(&self) -> &Vec<CharacterFact> {
        &self.facts
    }

    pub fn legacy(&self) -> &CharacterLegacy {
        &self.legacy
    }

    pub fn media_id(&self) -> &MediaID {
        &self.media_id
    }
}
