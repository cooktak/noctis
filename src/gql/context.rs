use crate::database::connection::MysqlPool;
use crate::config::Config;

impl juniper::Context for Context {}

pub struct Context {
    pub database_pool: MysqlPool,
    pub config: Config,
}

impl Context {
    pub fn new(database_pool: MysqlPool, config: Config) -> Self {
        Context {
            config,
            database_pool,
        }
    }
}