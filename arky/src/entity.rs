use arkycore::types::{Deserialize, IndexesTree, Serialize};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq)]
pub enum EntityError {
    #[error("Failed to serialize entity")]
    SerializeError,
    #[error("Failed to deserialize entity")]
    DeserializeError,
    #[error("Failed to insert entity: {key}. Error: {error}")]
    InsertEntityError { key: String, error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityItem {
    pub name: String,
    pub indexes: IndexesTree,
}

impl EntityItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            indexes: IndexesTree::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EntityError> {
        let entity: Self =
            bincode::deserialize(bytes).map_err(|_| EntityError::DeserializeError)?;
        Ok(entity)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, EntityError> {
        let bytes = bincode::serialize(self).map_err(|_| EntityError::SerializeError)?;
        Ok(bytes)
    }
}
