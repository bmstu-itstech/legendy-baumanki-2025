use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::{CharactersProvider, MediaProvider};
use crate::app::usecases::dto::CharacterDTO;
use crate::domain::models::CharacterName;

#[derive(Clone)]
pub struct GetCharacter {
    characters_provider: Arc<dyn CharactersProvider>,
    media_provider: Arc<dyn MediaProvider>,
}

impl GetCharacter {
    pub fn new(provider: Arc<dyn CharactersProvider>, media_provider: Arc<dyn MediaProvider>) -> Self {
        GetCharacter { characters_provider: provider, media_provider }
    }

    pub async fn character(&self, name: &CharacterName) -> Result<CharacterDTO, AppError> {
        match self.characters_provider.character_by_name(name).await? {
            None => Err(AppError::CharacterNotFound(name.clone())),
            Some(character) => {
                let image = self.media_provider.media(character.media_id()).await?;
                Ok(CharacterDTO{
                    id: character.id().clone(),
                    name: character.name().clone(),
                    quote: character.quote().clone(),
                    facts: character.facts().clone(),
                    legacy: character.legacy().clone(),
                    image_id: image.file_id().clone(),
                })
            }
        }
    }
}
