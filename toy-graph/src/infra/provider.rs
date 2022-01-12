//! Provider
//!
//! A provider manages different graphs. It contains two main fields: mongo_client and redis_client.
//! The former plays the role of data persistence, and the latter plays the role of data caching.

use crate::infra::{MongoClient, RedisClient};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Provider {
    mongo_client: MongoClient,
    redis_client: RedisClient,
}
