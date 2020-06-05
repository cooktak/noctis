use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database Connection Error: {0}")]
    ConnectionError(String),
    #[error("Database Migration Error: {0}")]
    MigrationError(String),
    #[error("Database Pool Error: {0}")]
    PoolError(String),
}
