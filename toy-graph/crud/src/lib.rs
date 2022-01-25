//! Crud

pub mod cache;
pub mod persistence;

pub use cache::RedisClient;
pub use crud_derive::GrantCRUD;
pub use persistence::{MongoClient, MongoClientFactory};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_document};
use serde::{de::DeserializeOwned, Serialize};

pub trait IDMutator {
    fn id(&self) -> Option<ObjectId>;

    fn remove_id(&mut self);

    fn mutate_id(&mut self, oid: ObjectId) -> Result<()>;
}

#[async_trait]
pub trait CRUD<TYPE>: MongoClientFactory
where
    TYPE: Send + Sync + Serialize + DeserializeOwned + Unpin + Clone + IDMutator,
{
    /// Create a new document
    async fn create<'a>(&'a self, mut value: TYPE) -> Result<TYPE>
    where
        TYPE: 'a,
    {
        // in case of `id` field exists, we need to remove it
        value.remove_id();
        let insert = self.coll::<TYPE>().insert_one(value.clone(), None).await?;
        let oid = insert.inserted_id.as_object_id().unwrap();
        value.mutate_id(oid)?;
        Ok(value)
    }

    /// Read a document by id
    async fn read<'a>(&'a self, id: ObjectId) -> Result<Option<TYPE>>
    where
        TYPE: 'a,
    {
        let filter = doc! { "_id": id };
        let result = self.coll::<TYPE>().find_one(filter, None).await?;
        Ok(result)
    }

    /// Update an existing document
    async fn update<'a>(&'a self, value: TYPE) -> Result<TYPE>
    where
        TYPE: 'a,
    {
        let oid = value
            .id()
            .ok_or_else(|| anyhow!("No `id` field was found!"))?;
        let filter = doc! {"_id": oid};
        let update = doc! {
            "$set": to_document(&value).unwrap()
        };
        self.coll::<TYPE>().update_one(filter, update, None).await?;
        Ok(value)
    }

    /// Delete an existing document
    async fn delete<'a>(&'a self, value: TYPE) -> Result<()>
    where
        TYPE: 'a,
    {
        let oid = value
            .id()
            .ok_or_else(|| anyhow!("No `id` field was found!"))?;
        let filter = doc! {"_id": oid};
        self.coll::<TYPE>().delete_one(filter, None).await?;
        Ok(())
    }
}
