use diesel::{Connection, ConnectionResult};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};

use crate::config;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_diesel_connection(config: &config::Database) -> ConnectionResult<MysqlConnection> {
    let database_url = &config.url;
    MysqlConnection::establish(database_url)
}

pub fn establish_r2d2_connection(config: &config::Database) -> ConnectionManager<MysqlConnection> {
    let database_url = &config.url;
    ConnectionManager::new(database_url)
}

pub fn build_pool(manager: ConnectionManager<MysqlConnection>) -> Result<MysqlPool, PoolError> {
    Pool::builder().build(manager)
}
