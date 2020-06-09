use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Given token not found on database")]
    NotFound,
    #[error("Unknown Error: {0}")]
    Unknown(String),
}
