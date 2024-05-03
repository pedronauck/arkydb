use std::sync::Arc;

use async_trait::async_trait;
use rocksdb::{
    BoundColumnFamily, ColumnFamilyDescriptor, DBWithThreadMode, MultiThreaded,
    Options as RocksDBOptions,
};

use crate::{
    core::types::NodeID,
    db::{DBError, DB},
    edge::EdgeItem,
    entity::EntityItem,
    node::Node,
    storage::{Storage, StorageError},
};

#[derive(Debug)]
pub struct Database {
    key: String,
    instance: DBWithThreadMode<MultiThreaded>,
}

static NODES_CF: &str = "nodes";
static EDGES_CF: &str = "edges";
static ENTITIES_CF: &str = "entities";

impl Database {
    fn create_db_instance(
        config: &RocksDBConfig,
    ) -> Result<DBWithThreadMode<MultiThreaded>, rocksdb::Error> {
        let mut dbs_opts = RocksDBOptions::default();
        dbs_opts.create_if_missing(config.create_if_missing);
        dbs_opts.set_error_if_exists(config.set_error_if_exists);
        dbs_opts.create_missing_column_families(config.create_missing_column_families);

        let nodes = ColumnFamilyDescriptor::new(NODES_CF, dbs_opts.clone());
        let edges = ColumnFamilyDescriptor::new(EDGES_CF, dbs_opts.clone());
        let entities = ColumnFamilyDescriptor::new(ENTITIES_CF, dbs_opts.clone());
        let cfs = vec![nodes, edges, entities];
        DBWithThreadMode::open_cf_descriptors(&dbs_opts, &config.path, cfs)
    }

    fn _get_entity(
        &self,
        name: &str,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<EntityItem, DBError> {
        self.instance
            .get_cf(handle, name.to_string())
            .map_err(|e| DBError::GetEntityError {
                key: name.to_string(),
                error: e.to_string(),
            })?
            .ok_or_else(|| DBError::GetEntityError {
                key: name.to_string(),
                error: "Entity not found".to_string(),
            })
            .and_then(|node_bytes| {
                EntityItem::from_bytes(&node_bytes).map_err(|e| DBError::GetEntityError {
                    key: name.to_string(),
                    error: e.to_string(),
                })
            })
    }

    fn _entity_to_bytes_with_error(&self, entity: &EntityItem) -> Result<Vec<u8>, DBError> {
        entity.to_bytes().map_err(|e| DBError::InsertEntityError {
            key: entity.name.to_string(),
            error: e.to_string(),
        })
    }

    fn _insert_entity(
        &self,
        entity: &EntityItem,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<(), DBError> {
        let entity_serialized = self._entity_to_bytes_with_error(entity)?;
        self.instance
            .put_cf(handle, entity.name.to_string(), entity_serialized)
            .map_err(|e| DBError::InsertEntityError {
                key: entity.name.to_string(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    fn _insert_entity_if_needed<T: Node>(&self, node: &T) {
        let entities = self.instance.cf_handle(ENTITIES_CF).unwrap();
        let entity_name = node.entity();
        let entity = self._get_entity(entity_name.as_str(), &entities);
        if let Err(_) = entity {
            let new_entity = EntityItem::new(entity_name);
            self._insert_entity(&new_entity, &entities).unwrap();
        }
    }

    fn _get_node<T: Node>(
        &self,
        id: NodeID,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<T, DBError> {
        self.instance
            .get_cf(handle, id.to_string())
            .map_err(|e| DBError::GetNodeError {
                key: id,
                error: e.to_string(),
            })?
            .ok_or_else(|| DBError::GetNodeError {
                key: id,
                error: "Node not found".to_string(),
            })
            .and_then(|node_bytes| {
                T::from_bytes(&node_bytes).map_err(|e| DBError::GetNodeError {
                    key: id,
                    error: e.to_string(),
                })
            })
    }

    fn _node_to_bytes_with_error<T: Node>(&self, node: &T) -> Result<Vec<u8>, DBError> {
        node.to_bytes().map_err(|e| DBError::InsertNodeError {
            key: node.key(),
            error: e.to_string(),
        })
    }

    fn _insert_node<T: Node>(
        &self,
        node: &T,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<(), DBError> {
        let node_serialized = self._node_to_bytes_with_error(node)?;
        self.instance
            .put_cf(handle, node.key().to_string(), node_serialized)
            .map_err(|e| DBError::InsertNodeError {
                key: node.key(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    fn _get_edge(
        &self,
        from: NodeID,
        to: NodeID,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<EdgeItem, DBError> {
        let id = &EdgeItem::format_key(from, to);
        self.instance
            .get_cf(handle, id)
            .map_err(|e| DBError::GetEdgeError {
                key: id.to_string(),
                error: e.to_string(),
            })?
            .ok_or_else(|| DBError::GetEdgeError {
                key: id.to_string(),
                error: "Edge not found".to_string(),
            })
            .and_then(|edge_bytes| {
                EdgeItem::from_bytes(&edge_bytes).map_err(|e| DBError::GetEdgeError {
                    key: id.to_string(),
                    error: e.to_string(),
                })
            })
    }

    fn _edge_to_bytes_with_error(&self, edge: &EdgeItem) -> Result<Vec<u8>, DBError> {
        edge.to_bytes().map_err(|e| DBError::InsertEdgeError {
            key: edge.key(),
            error: e.to_string(),
        })
    }

    fn _insert_edge(
        &self,
        edge: &EdgeItem,
        handle: &Arc<BoundColumnFamily<'_>>,
    ) -> Result<(), DBError> {
        let edge_serialized = self._edge_to_bytes_with_error(edge)?;
        self.instance
            .put_cf(handle, edge.key(), edge_serialized)
            .map_err(|e| DBError::InsertEdgeError {
                key: edge.key(),
                error: e.to_string(),
            })?;

        Ok(())
    }
}

#[async_trait]
impl DB for Database {
    type Config = RocksDBConfig;

    fn key(&self) -> String {
        self.key.clone()
    }

    fn new(key: String, config: &Self::Config) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Database::create_db_instance(&config)
            .map(|instance| Database { key, instance })
            .map_err(|e| DBError::ConnectError {
                error: e.to_string(),
            })
    }

    /**
     * Entity methods
     */
    async fn get_entity(&self, name: &str) -> Result<EntityItem, DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        self._get_entity(name, &handle)
    }

    async fn insert_entity(&self, entity: &EntityItem) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        self._insert_entity(entity, &handle)
    }

    async fn insert_entities(&self, entities: &[EntityItem]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();

        for entity in entities {
            let entity_serialized = self._entity_to_bytes_with_error(entity)?;
            batch.put_cf(&handle, entity.name.to_string(), entity_serialized);
        }

        Ok(())
    }

    async fn remove_entity(&self, entity: &EntityItem) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        self.instance
            .delete_cf(&handle, entity.name.to_string())
            .map_err(|e| DBError::RemoveEntityError {
                key: entity.name.to_string(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    async fn remove_entities(&self, entities: &[EntityItem]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();
        for entity in entities {
            batch.delete_cf(&handle, entity.name.to_string());
        }
        Ok(())
    }

    async fn update_entity(&self, entity: &EntityItem) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(ENTITIES_CF).unwrap();
        self._insert_entity(entity, &handle)
    }

    /**
     * Node methods
     */
    async fn get_node<T: Node>(&self, id: NodeID) -> Result<T, DBError> {
        let nodes = self.instance.cf_handle(NODES_CF).unwrap();
        self._get_node(id, &nodes)
    }

    async fn insert_node<T: Node + Sync>(&self, node: &T) -> Result<(), DBError> {
        let nodes = self.instance.cf_handle(NODES_CF).unwrap();
        self._insert_node(node, &nodes)?;
        self._insert_entity_if_needed(node);
        Ok(())
    }

    async fn insert_nodes<T: Node>(&self, nodes: &[T]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(NODES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();

        for node in nodes {
            let node_serialized = self._node_to_bytes_with_error(node)?;
            batch.put_cf(&handle, node.key().to_string(), node_serialized);
        }

        Ok(())
    }

    async fn remove_node<T: Node>(&self, node: &T) -> Result<(), DBError> {
        let nodes = self.instance.cf_handle(NODES_CF).unwrap();
        let res = self.instance.delete_cf(&nodes, node.key().to_string());

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(DBError::RemoveNodeError {
                key: node.key(),
                error: e.to_string(),
            }),
        }
    }

    async fn remove_nodes<T: Node>(&self, nodes: &[T]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(NODES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();

        for node in nodes {
            batch.delete_cf(&handle, node.key().to_string());
        }

        Ok(())
    }

    async fn update_node<T: Node>(&self, node: &T) -> Result<(), DBError> {
        self.insert_node(node).await
    }

    /**
     * Edge methods
     */
    async fn get_edge(&self, from: NodeID, to: NodeID) -> Result<EdgeItem, DBError> {
        let handle = self.instance.cf_handle(EDGES_CF).unwrap();
        self._get_edge(from, to, &handle)
    }

    async fn insert_edge(&self, edge: &EdgeItem) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(EDGES_CF).unwrap();
        self._insert_edge(edge, &handle)?;
        Ok(())
    }

    async fn insert_edges(&self, edges: &[EdgeItem]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(EDGES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();

        for edge in edges {
            let edge_serialized = self._edge_to_bytes_with_error(edge)?;
            let id = EdgeItem::format_key(edge.from, edge.to);
            batch.put_cf(&handle, id, edge_serialized);
        }

        Ok(())
    }

    async fn remove_edge(&self, edge: &EdgeItem) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(EDGES_CF).unwrap();
        let id = EdgeItem::format_key(edge.from, edge.to);
        self.instance
            .delete_cf(&handle, id)
            .map_err(|e| DBError::RemoveEdgeError {
                key: format!("{}-{}", edge.from, edge.to),
                error: e.to_string(),
            })?;

        Ok(())
    }

    async fn remove_edges(&self, edges: &[EdgeItem]) -> Result<(), DBError> {
        let handle = self.instance.cf_handle(EDGES_CF).unwrap();
        let mut batch = rocksdb::WriteBatch::default();

        for edge in edges {
            let id = EdgeItem::format_key(edge.from, edge.to);
            batch.delete_cf(&handle, id);
        }

        Ok(())
    }

    async fn update_edge(&self, edge: &EdgeItem) -> Result<(), DBError> {
        self.insert_edge(edge).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocksDBConfig {
    pub path: String,
    pub set_error_if_exists: bool,
    pub create_if_missing: bool,
    pub create_missing_column_families: bool,
}
impl Default for RocksDBConfig {
    fn default() -> Self {
        Self {
            path: "./data".to_string(),
            set_error_if_exists: true,
            create_if_missing: true,
            create_missing_column_families: true,
        }
    }
}

#[derive(Debug)]
pub struct RocksDB {
    db: Result<Database, DBError>,
    config: RocksDBConfig,
}
impl Storage for RocksDB {
    type Config = RocksDBConfig;
    type Database = Database;

    fn set_config(&mut self, config: &Self::Config) {
        self.config = config.clone();
    }

    fn new(config: Self::Config) -> Self {
        let db = Database::new("rocksdb".to_string(), &config);
        Self { db, config }
    }

    fn use_db(&self) -> Result<&Self::Database, StorageError> {
        match &self.db {
            Ok(db) => Ok(db),
            Err(e) => Err(StorageError::DbError(e.clone())),
        }
    }
}
