//! Crud

pub mod cache;
pub mod persistence;

pub use cache::RedisClient;
pub use crud_derive::GrantCRUD;
pub use persistence::{MongoClient, MongoClientFactory};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::Serialize;

pub trait IDMutator {
    fn mutate_id(&mut self, oid: ObjectId) -> Result<()>;
}

#[async_trait]
pub trait CRUD<TYPE>: MongoClientFactory
where
    TYPE: Send + Sync + Serialize + Clone + IDMutator,
{
    async fn create<'a>(&'a self, db: &str, collection: &str, mut value: TYPE) -> Result<TYPE>
    where
        TYPE: 'a,
    {
        let insert = self
            .collection::<TYPE>(db, collection)
            .insert_one(value.clone(), None)
            .await?;
        let oid = insert
            .inserted_id
            .as_object_id()
            .ok_or(anyhow!("Invalid ObjectId"))?;
        value.mutate_id(oid)?;
        Ok(value)
    }
}
