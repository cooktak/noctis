use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User Already Exists in Database")]
    Exists,
    #[error("Couldn't find user with given query {0}")]
    NotFound(String),
    #[error("Unknown Error: {0}")]
    Unknown(String),
}
