use diesel::Connection;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::config;

use super::error::DatabaseError;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_diesel_connection(config: &config::Database) -> Result<MysqlConnection, DatabaseError> {
    MysqlConnection::establish(&config.url)
    .map_err(move |e| DatabaseError::ConnectionError(e.to_string()))
}

pub fn establish_r2d2_connection(config: &config::Database) -> ConnectionManager<MysqlConnection> {
    ConnectionManager::new(&config.url)
}

pub fn build_pool(manager: ConnectionManager<MysqlConnection>) -> Result<MysqlPool, DatabaseError> {
    Pool::builder()
    .build(manager)
    .map_err(move |e| DatabaseError::PoolError(e.to_string()))
}
