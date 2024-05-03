use crate::{
    core::types::{DynResult, NodeID},
    edge::EdgeItem,
    entity::EntityItem,
    node::Node,
    query::{QueryBuilder, QueryExecutor},
};
use async_trait::async_trait;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq, Clone)]
pub enum DBError {
    #[error("Failed to connect on Database: {error}")]
    ConnectError { error: String },
    #[error("Failed to get entity: {key}. Error: {error}")]
    GetEntityError { key: String, error: String },
    #[error("Entity not found: {key}")]
    EntityNotFoundError { key: String },
    #[error("Failed to insert entity: {key}. Error: {error}")]
    InsertEntityError { key: String, error: String },
    #[error("Failed to remove entity: {key}. Error: {error}")]
    RemoveEntityError { key: String, error: String },
    #[error("Failed to update entity: {key}. Error: {error}")]
    UpdateEntityError { key: String, error: String },
    #[error("Failed to get node {key}. Error: {error}")]
    GetNodeError { key: NodeID, error: String },
    #[error("Failed to insert node {key}. Error: {error}")]
    InsertNodeError { key: NodeID, error: String },
    #[error("Failed to remove node: {key}. Error: {error}")]
    RemoveNodeError { key: NodeID, error: String },
    #[error("Failed to update node: {key}. Error: {error}")]
    UpdateNodeError { key: NodeID, error: String },
    #[error("Failed to get edge: {key}. Error: {error}")]
    GetEdgeError { key: String, error: String },
    #[error("Failed to insert edge: {key}. Error: {error}")]
    InsertEdgeError { key: String, error: String },
    #[error("Failed to remove edge: {key}. Error: {error}")]
    RemoveEdgeError { key: String, error: String },
    #[error("Failed to update edge: {key}. Error: {error}")]
    UpdateEdgeError { key: String, error: String },
    #[error("Failed to execute query")]
    QueryError,
    #[error("Execution error")]
    ExecError,
}

#[async_trait]
pub trait DB {
    type Config;

    fn key(&self) -> String;
    fn new(key: String, config: &Self::Config) -> Result<Self, DBError>
    where
        Self: Sized;

    /**
     * Entity methods
     */
    async fn get_entity(&self, name: &str) -> Result<EntityItem, DBError>;
    async fn insert_entity(&self, entity: &EntityItem) -> Result<(), DBError>;
    async fn insert_entities(&self, entities: &[EntityItem]) -> Result<(), DBError>;
    async fn remove_entity(&self, entity: &EntityItem) -> Result<(), DBError>;
    async fn remove_entities(&self, entities: &[EntityItem]) -> Result<(), DBError>;
    async fn update_entity(&self, entity: &EntityItem) -> Result<(), DBError>;

    /**
     * Node methods
     */
    async fn get_node<T: Node>(&self, id: NodeID) -> Result<T, DBError>;
    async fn insert_node<T: Node + Sync>(&self, node: &T) -> Result<(), DBError>;
    async fn insert_nodes<T: Node>(&self, nodes: &[T]) -> Result<(), DBError>;
    async fn remove_node<T: Node>(&self, node: &T) -> Result<(), DBError>;
    async fn remove_nodes<T: Node>(&self, nodes: &[T]) -> Result<(), DBError>;
    async fn update_node<T: Node>(&self, node: &T) -> Result<(), DBError>;

    /**
     * Edge methods
     */
    async fn get_edge(&self, from: NodeID, to: NodeID) -> Result<EdgeItem, DBError>;
    async fn insert_edge(&self, edge: &EdgeItem) -> Result<(), DBError>;
    async fn insert_edges(&self, edges: &[EdgeItem]) -> Result<(), DBError>;
    async fn remove_edge(&self, edge: &EdgeItem) -> Result<(), DBError>;
    async fn remove_edges(&self, edges: &[EdgeItem]) -> Result<(), DBError>;
    async fn update_edge(&self, edge: &EdgeItem) -> Result<(), DBError>;

    /**
     * Query methods
     */
    fn query(&self) -> QueryBuilder<Self>
    where
        Self: Sized,
    {
        QueryBuilder::new(self)
    }

    async fn exec<R>(&self, query: QueryExecutor<'async_trait, Self>) -> DynResult<R>
    where
        Self: Sized + Send + Sync,
    {
        // TODO: handle error
        query.exec::<R>()
    }
}
