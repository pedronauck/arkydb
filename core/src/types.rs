pub use crate::data::Data;
pub use crate::id::{EdgeID, NodeID};
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub type DynResult<T> = Result<T, Box<dyn Error>>;
pub type NodesTree<T> = HashMap<NodeID, T>;
pub type IndexesTree = HashMap<String, HashMap<String, Vec<NodeID>>>;
pub type EdgesTree<T> = HashMap<(NodeID, NodeID), T>;
pub type EntitiesTree<T> = HashMap<String, T>;
