use crate::node::Node;
pub use arkycore::types::{Data, Deserialize, Serialize};
use arkycore::{id::EdgeID, types::NodeID};
use thiserror::Error as ThisError;

pub mod prelude {
    pub use super::{Data, Edge, EdgeBuilder, EdgeError, EdgeList, EdgeRef};
}

#[derive(Debug, ThisError, PartialEq, Serialize, Deserialize)]
pub enum EdgeError {
    #[error("Error when trying to get edge data: {data_type:?}")]
    EdgeDataMismatch { data_type: String },
    #[error("Cannot unlink from an empty list")]
    UnlinkFromEmptyList,
    #[error("Cannot unlink from an inexistent nodes: {0:?} -> {1:?}")]
    UnlinkFromInexistentNodes(NodeID, NodeID),
    #[error("Failed to serialize node")]
    SerializeError,
    #[error("Failed to deserialize node")]
    DeserializeError,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EdgeItem {
    pub label: String,
    pub from: NodeID,
    pub to: NodeID,
    pub data: Data,
}
impl Default for EdgeItem {
    fn default() -> Self {
        Self {
            label: "edge_item".to_string(),
            from: NodeID::default(),
            to: NodeID::default(),
            data: Data::default(),
        }
    }
}
impl EdgeItem {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EdgeError> {
        let edge: Self = bincode::deserialize(bytes).map_err(|_| EdgeError::DeserializeError)?;
        Ok(edge)
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, EdgeError> {
        let bytes = bincode::serialize(self).map_err(|_| EdgeError::SerializeError)?;
        Ok(bytes)
    }
    pub fn key(&self) -> String {
        Self::format_key(self.from, self.to)
    }
    pub fn format_key(from: NodeID, to: NodeID) -> String {
        format!("{}:{}", from, to)
    }
}

pub trait EdgeBuilder {
    fn new(label: &str) -> Self;
    fn link(&mut self, from: &impl Node, to: &impl Node, data: Data) -> &mut Self;
    fn label(&self) -> String;
    fn key(&self) -> EdgeID;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeID,
    pub label: String,
    pub item: Option<EdgeItem>,
}
impl EdgeBuilder for Edge {
    fn new(label: &str) -> Self {
        Self {
            id: EdgeID::new(),
            label: label.to_string(),
            item: None,
        }
    }
    fn link(&mut self, from: &impl Node, to: &impl Node, data: Data) -> &mut Self {
        let edge_item = EdgeItem {
            label: self.label.clone(),
            from: from.key(),
            to: to.key(),
            data,
        };
        self.item = Some(edge_item);
        self
    }
    fn label(&self) -> String {
        self.label.to_string()
    }
    fn key(&self) -> EdgeID {
        self.id.clone()
    }
}
impl Edge {
    pub fn unlink(&mut self) -> &mut Self {
        self.item = None;
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EdgeList {
    pub id: EdgeID,
    pub label: String,
    pub items: Vec<EdgeItem>,
}
impl EdgeBuilder for EdgeList {
    fn new(label: &str) -> Self {
        Self {
            id: EdgeID::new(),
            label: label.to_string(),
            items: Vec::new(),
        }
    }
    fn link(&mut self, from: &impl Node, to: &impl Node, data: Data) -> &mut Self {
        let edge_item = EdgeItem {
            label: self.label.clone(),
            from: from.key(),
            to: to.key(),
            data,
        };
        self.items.push(edge_item);
        self
    }
    fn label(&self) -> String {
        self.label.to_string()
    }
    fn key(&self) -> EdgeID {
        self.id.clone()
    }
}
impl EdgeList {
    pub fn unlink(&mut self, from: &impl Node, to: &impl Node) -> Result<&mut Self, EdgeError> {
        let mut items = self.items.clone();
        if items.is_empty() {
            return Err(EdgeError::UnlinkFromEmptyList);
        }

        let from_exist = items.iter().any(|item| item.from == from.key());
        let to_exist = items.iter().any(|item| item.to == to.key());
        if !from_exist || !to_exist {
            return Err(EdgeError::UnlinkFromInexistentNodes(from.key(), to.key()));
        }

        items.retain(|item| item.from != from.key() && item.to != to.key());
        self.items = items;
        Ok(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EdgeRef {
    pub id: EdgeID,
}
impl EdgeRef {
    pub fn new<T: EdgeBuilder>(edge: &T) -> Self {
        Self { id: edge.key() }
    }
}
