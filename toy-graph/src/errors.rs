//! Error handling

use anyhow::Result;
use thiserror::Error;

pub type TGResult<T> = Result<T>;

#[derive(Error, Debug)]
pub enum TGError {
    #[error("Invalid argument: {0}")]
    Parse(String),

    #[error("ID not found")]
    IDNotFound,

    #[error("Invalid object id")]
    InvalidID,
}
