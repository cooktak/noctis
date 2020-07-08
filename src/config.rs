use std::env::{var, VarError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0} does not exist")]
    MissingKey(String),
    #[error("{0} is in invalid format")]
    InvalidFormat(String),
}

impl ConfigError {
    fn from_var_error(key: &str, var_error: VarError) -> Self {
        match var_error {
            VarError::NotPresent => Self::MissingKey(key.to_string()),
            VarError::NotUnicode(_) => Self::InvalidFormat(key.to_string()),
        }
    }
}

macro_rules! try_get_var {
    ($key:literal) => {
        var($key).map_err(|e| ConfigError::from_var_error($key, e))?
    };
}

#[derive(Clone)]
pub struct Database {
    pub url: String,
}

impl Database {
    fn from_env() -> Result<Self, ConfigError> {
        let url = try_get_var!("DATABASE_URL");

        Ok(Self {
            url,
        })
    }
}

#[derive(Clone)]
pub struct Config {
    pub database: Database,
    pub bind_address: String,
    pub jwt_issuer: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let database = Database::from_env()?;

        let bind_address = var("BIND_ADDRESS").unwrap_or("0.0.0.0:80".to_owned());

        let jwt_issuer = var("JWT_ISSUER").unwrap_or("cooktak".to_owned());

        use rand::thread_rng;
        use rand::Rng;
        use rand::distributions::Alphanumeric;
        let jwt_secret = var("JWT_SECRET").unwrap_or(
            thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect()
        );

        Ok(Self {
            database,
            bind_address,
            jwt_issuer,
            jwt_secret,
        })
    }
}