pub use arkycore::types::{Deserialize, NodeID, Serialize};
use arkycore::utils;
pub use arkymacros_schema::schema;
use thiserror::Error as ThisError;

pub mod prelude {
    pub use super::{schema, Node, NodeID};
}

#[derive(Debug, ThisError, PartialEq)]
pub enum NodeError {
    #[error("Failed to serialize node")]
    SerializeError,
    #[error("Failed to deserialize node")]
    DeserializeError,
}

pub trait Node
where
    Self: Sized + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static,
{
    fn key(&self) -> NodeID;
    fn entity(&self) -> String {
        utils::format_entity("WeakNode")
    }
    fn new<T: Node>(node: T) -> T {
        node
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, NodeError> {
        let node: Self = bincode::deserialize(bytes).map_err(|_| NodeError::DeserializeError)?;
        Ok(node)
    }
    fn to_bytes(&self) -> Result<Vec<u8>, NodeError> {
        let bytes = bincode::serialize(self).map_err(|_| NodeError::SerializeError)?;
        Ok(bytes)
    }
}
