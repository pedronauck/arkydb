use crate::db::DB;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq)]
pub enum StorageError {
    #[error("Use database error: {0}")]
    DbError(#[from] crate::db::DBError),
}

pub trait Storage {
    type Config;
    type Database: DB;

    /**
     * General methods
     */
    fn new(config: Self::Config) -> Self;
    fn set_config(&mut self, config: &Self::Config);
    fn use_db(&self) -> Result<&Self::Database, StorageError>;
}
