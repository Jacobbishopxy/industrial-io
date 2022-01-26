//! Crud

pub mod cache;
pub mod persistence;

pub use cache::RedisClient;
pub use crud_derive::GrantCRUD;
pub use persistence::*;
