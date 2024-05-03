pub use crate::db::DB;
pub use crate::storage::{Storage, StorageError};

pub mod prelude {
    pub use super::ArkyDB;
    pub use crate::db::{DBError, DB};
    pub use crate::storage::{Storage, StorageError};
    pub use crate::storages::rocksdb::{RocksDB, RocksDBConfig};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArkyDB<S: Storage> {
    storage: S,
}
impl<S: Storage> ArkyDB<S> {
    pub fn init(storage: &S) -> &S::Database {
        storage.use_db().unwrap()
    }
}
