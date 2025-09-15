use serde::{Deserialize, Serialize};
use crate::domain::error::DomainError;

pub const MEDIA_ID_MAX_LENGTH: usize = 64;

#[derive(Clone, Copy)]
pub enum MediaType {
    Image,
    VideoNote,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaID(String);

impl MediaID {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id == "" {
            return Err(DomainError::InvalidValue("invalid media id: not empty string".to_string()));
        }
        if id.len() > MEDIA_ID_MAX_LENGTH {
            return Err(DomainError::InvalidValue(format!(
                "invalid media id: expected length <= {MEDIA_ID_MAX_LENGTH}, got {}",
                id.len()
            )));
        }
        Ok(Self(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(
    Clone,
    Debug,
)]
pub struct FileID(String);

impl FileID {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub struct Media {
    id: MediaID,
    file_id: FileID,
    media_type: MediaType,
}

impl Media {
    pub fn new(id: MediaID, file_id: FileID, media_type: MediaType) -> Self {
        Self {
            id,
            file_id,
            media_type,
        }
    }
    
    pub fn image(id: MediaID, file_id: FileID) -> Self {
        Self::new(id, file_id, MediaType::Image)
    }
    
    pub fn video_note(id: MediaID, file_id: FileID) -> Self {
        Self::new(id, file_id, MediaType::VideoNote)
    }

    pub fn id(&self) -> &MediaID {
        &self.id
    }

    pub fn file_id(&self) -> &FileID {
        &self.file_id
    }

    pub fn media_type(&self) -> MediaType {
        self.media_type
    }
}
