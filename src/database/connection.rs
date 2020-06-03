use std::error::Error;

use diesel::{Connection, ConnectionResult};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};

use crate::config;

use super::error::DatabaseError;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_diesel_connection(config: &config::Database) -> Result<MysqlConnection, DatabaseError> {
    let database_url = &config.url;
    MysqlConnection::establish(database_url)
    .map_err(move |e| DatabaseError::ConnectionError(e.to_string()))
}

pub fn establish_r2d2_connection(config: &config::Database) -> ConnectionManager<MysqlConnection> {
    let database_url = &config.url;
    ConnectionManager::new(database_url)
}

pub fn build_pool(manager: ConnectionManager<MysqlConnection>) -> Result<MysqlPool, DatabaseError> {
    Pool::builder()
    .build(manager)
    .map_err(move |e| DatabaseError::PoolError(e.to_string()))
}
