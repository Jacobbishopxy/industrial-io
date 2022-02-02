//! Crud

pub mod cache;
pub mod persistence;

pub use cache::RedisClient;
pub use crud_derive::CRUD;
pub use persistence::*;
