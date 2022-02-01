//! Persistence service.

use std::borrow::Cow;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_document};
use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct MongoClient {
    client: mongodb::Client,
    pub database: String,
    pub collection: String,
}

impl MongoClient {
    pub async fn new(uri: &str, database: &str, collection: &str) -> anyhow::Result<Self> {
        let co = mongodb::options::ClientOptions::parse(uri).await?;

        let client = mongodb::Client::with_options(co)?;

        Ok(MongoClient {
            client,
            database: database.to_string(),
            collection: collection.to_string(),
        })
    }

    pub async fn show_dbs(&self) -> anyhow::Result<Vec<String>> {
        let dbs = self.client.list_database_names(None, None).await?;
        Ok(dbs)
    }
}

pub trait MongoClientFactory: Send + Sync {
    fn database(&self) -> Cow<str>;

    fn set_database(&mut self, database: &str);

    fn collection(&self) -> Cow<str>;

    fn set_collection(&mut self, collection: &str);

    fn coll<T>(&self) -> mongodb::Collection<T>;
}

impl MongoClientFactory for MongoClient {
    fn database(&self) -> Cow<str> {
        Cow::Borrowed(&self.database)
    }

    fn set_database(&mut self, database: &str) {
        self.database = database.to_string();
    }

    fn collection(&self) -> Cow<str> {
        Cow::Borrowed(&self.collection)
    }

    fn set_collection(&mut self, collection: &str) {
        self.collection = collection.to_string();
    }

    fn coll<T>(&self) -> mongodb::Collection<T> {
        self.client
            .database(&self.database)
            .collection(&self.collection)
    }
}

pub trait IDMutator {
    fn id(&self) -> Option<ObjectId>;

    fn remove_id(&mut self);

    fn mutate_id(&mut self, oid: ObjectId) -> Result<()>;
}

#[async_trait]
pub trait MongoCRUD<TYPE>: MongoClientFactory
where
    TYPE: Send + Sync + Clone + Serialize + DeserializeOwned + Unpin + IDMutator,
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

    /// Read many documents by ids
    async fn read_many<'a>(&'a self, ids: Vec<ObjectId>) -> Result<Vec<TYPE>>
    where
        TYPE: 'a,
    {
        let filter = doc! { "_id": { "$in": ids } };
        self.coll::<TYPE>()
            .find(filter, None)
            .await?
            .map(|v| v.map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()
            .await
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
        let update = doc! {"$set": to_document(&value).unwrap()};
        self.coll::<TYPE>().update_one(filter, update, None).await?;
        Ok(value)
    }

    /// Delete an existing document
    async fn delete<'a>(&'a self, id: ObjectId) -> Result<()>
    where
        TYPE: 'a,
    {
        let filter = doc! {"_id": id};
        self.coll::<TYPE>().delete_one(filter, None).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test_persistence {
    use super::*;

    #[tokio::test]
    async fn test_show_dbs() {
        let uri = "mongodb://root:secret@localhost:27017";

        let client = MongoClient::new(uri, "test", "dev").await;
        assert!(client.is_ok());

        let client = client.unwrap();

        let dbs = client.show_dbs().await;
        assert!(dbs.is_ok());
    }
}
