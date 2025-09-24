use std::sync::Arc;

use crate::app::error::AppError;
use crate::app::ports::CharactersProvider;
use crate::domain::models::CharacterName;

#[derive(Clone)]
pub struct GetCharacterNames {
    provider: Arc<dyn CharactersProvider>,
}

impl GetCharacterNames {
    pub fn new(provider: Arc<dyn CharactersProvider>) -> Self {
        GetCharacterNames { provider }
    }
    
    pub async fn characters(&self) -> Result<Vec<CharacterName>, AppError> {
        let characters = self.provider.characters().await?;
        Ok(characters.into_iter().map(|c| c.name().clone()).collect())
    }
}
