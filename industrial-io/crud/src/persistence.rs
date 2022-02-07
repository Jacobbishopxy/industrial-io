//! Persistence service.

use std::borrow::Cow;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_document};
use mongodb::{options::IndexOptions as MongoIndexOptions, IndexModel as MongoIndexModel};
use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct MongoClient {
    client: mongodb::Client,
    pub database: String,
    pub collection: String,
}

impl MongoClient {
    pub async fn new<U, T>(uri: U, database: T, collection: T) -> Result<Self>
    where
        U: AsRef<str>,
        T: Into<String>,
    {
        let co = mongodb::options::ClientOptions::parse(uri.as_ref()).await?;

        let client = mongodb::Client::with_options(co)?;

        Ok(MongoClient {
            client,
            database: database.into(),
            collection: collection.into(),
        })
    }

    pub fn set_database<T: Into<String>>(&mut self, database: T) {
        self.database = database.into();
    }

    pub fn set_collection<T: Into<String>>(&mut self, collection: T) {
        self.collection = collection.into();
    }

    /// show databases name
    pub async fn show_dbs(&self) -> Result<Vec<String>> {
        let dbs = self.client.list_database_names(None, None).await?;
        Ok(dbs)
    }

    /// show collections name in a database
    pub async fn show_collections(&self) -> Result<Vec<String>> {
        let collections = self
            .client
            .database(&self.database)
            .list_collection_names(None)
            .await?;
        Ok(collections)
    }

    // TODO: turn `IndexOptions` into `mongodb::MongoIndexModel`

    /// list all indexes in a collection
    /// T is the type of the document
    pub async fn list_indexes<T>(&self) -> Result<Vec<MongoIndexModel>> {
        self.schema::<T>()
            .list_indexes(None)
            .await?
            .map(|v| v.map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()
            .await
    }

    /// create index
    /// T is the type of the document
    pub async fn create_index<T>(&self, index: MongoIndexModel) -> Result<String> {
        let result = self.schema::<T>().create_index(index, None).await?;
        Ok(result.index_name)
    }

    /// drop index
    /// T is the type of the document
    pub async fn drop_index<T>(&self, index_name: &str) -> Result<()> {
        self.schema::<T>().drop_index(index_name, None).await?;
        Ok(())
    }
}

pub trait MongoClientFactory: Send + Sync {
    /// get database
    fn database(&self) -> Cow<str>;

    /// set database
    fn set_database(&mut self, database: &str);

    /// get collection
    fn collection(&self) -> Cow<str>;

    /// set collection
    fn set_collection(&mut self, collection: &str);

    /// get typed collection
    fn schema<T>(&self) -> mongodb::Collection<T>;
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

    fn schema<T>(&self) -> mongodb::Collection<T> {
        self.client
            .database(&self.database)
            .collection(&self.collection)
    }
}

#[derive(Debug, Clone)]
pub enum Dir {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct SingleIndex {
    pub name: String,
    pub dir: Dir,
    pub unique: bool,
    pub text: bool,
}

impl SingleIndex {
    pub fn new(name: &str, dir: Dir, unique: bool, text: bool) -> Self {
        SingleIndex {
            name: name.to_string(),
            dir,
            unique,
            text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleIndexOptions(pub Vec<SingleIndex>);

#[derive(Debug, Clone)]
pub struct CompoundIndexOptions {
    pub names: Vec<String>,
    pub dir: Dir,
    pub unique: bool,
    pub text: bool,
}

impl CompoundIndexOptions {
    pub fn new(names: &[&str], dir: Dir, unique: bool, text: bool) -> Self {
        CompoundIndexOptions {
            names: names.iter().map(|v| v.to_string()).collect(),
            dir,
            unique,
            text,
        }
    }
}

/// index options represent indexes in a collection, the default `_id` index is not included.
#[derive(Debug)]
pub enum IndexOptions {
    Single(SingleIndexOptions),
    Compound(CompoundIndexOptions),
    None,
}

// TODO: turn `IndexOptions` into `Vec<mongodb::MongoIndexModel>`
fn generate_mongo_index_module(indexes: &IndexOptions) -> Vec<MongoIndexModel> {
    match indexes {
        IndexOptions::Single(s) => {
            s.0.iter()
                .map(|si| {
                    let dir: i32 = match si.dir {
                        Dir::Asc => 1,
                        Dir::Desc => -1,
                    };
                    let unique = si.unique;
                    // let text = si.text;

                    MongoIndexModel::builder()
                        .keys(doc! { si.name.to_owned() : dir })
                        .options(MongoIndexOptions::builder().unique(unique).build())
                        .build()
                })
                .collect()
        }
        IndexOptions::Compound(c) => {
            let dir = match c.dir {
                Dir::Asc => 1,
                Dir::Desc => -1,
            };
            let unique = c.unique;
            // let text = c.text;

            let im = MongoIndexModel::builder()
                // TODO: {name1: dir1, name2: dir2}
                .keys(doc! { c.names.join(",") : dir })
                .options(MongoIndexOptions::builder().unique(unique).build())
                .build();

            vec![im]
        }
        IndexOptions::None => vec![],
    }
}

/// BaseCRUD trait
///
/// A Rust struct that implements this trait is a schema of MongoDB's collection.
/// According to the `crud` crate, any struct who derived `CRUD` will automatically implement this trait.
pub trait BaseCRUD {
    fn get_id(&self) -> Option<ObjectId>;

    fn remove_id(&mut self);

    fn mutate_id(&mut self, oid: ObjectId) -> Result<()>;

    /// show `Vec<indexOptions>`
    fn show_indexes(&self) -> IndexOptions;
}

/// MongoCRUD trait
///
/// According to `crud` crate, any struct who derived `CRUD` will automatically implement this trait.
/// In other words, `MongoClient` can use methods in this trait to persist `TYPE` data.
#[async_trait]
pub trait MongoCRUD<TYPE>: MongoClientFactory
where
    TYPE: Send + Sync + Clone + Serialize + DeserializeOwned + Unpin + BaseCRUD,
{
    /// Create a new document
    async fn create<'a>(&'a self, mut value: TYPE) -> Result<TYPE>
    where
        TYPE: 'a,
    {
        // in case of `id` field exists, we need to remove it
        value.remove_id();
        let insert = self
            .schema::<TYPE>()
            .insert_one(value.clone(), None)
            .await?;
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
        let result = self.schema::<TYPE>().find_one(filter, None).await?;
        Ok(result)
    }

    /// Read many documents by ids
    async fn read_many<'a>(&'a self, ids: Vec<ObjectId>) -> Result<Vec<TYPE>>
    where
        TYPE: 'a,
    {
        let filter = doc! { "_id": { "$in": ids } };
        self.schema::<TYPE>()
            .find(filter, None)
            .await?
            .map(|v| v.map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()
            .await
    }

    /// Read all documents
    async fn read_all<'a>(&'a self) -> Result<Vec<TYPE>>
    where
        TYPE: 'a,
    {
        self.schema::<TYPE>()
            .find(None, None)
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
            .get_id()
            .ok_or_else(|| anyhow!("No `id` field was found!"))?;
        let filter = doc! {"_id": oid};
        let update = doc! {"$set": to_document(&value).unwrap()};
        self.schema::<TYPE>()
            .update_one(filter, update, None)
            .await?;
        Ok(value)
    }

    /// Delete an existing document
    async fn delete<'a>(&'a self, id: ObjectId) -> Result<Option<TYPE>>
    where
        TYPE: 'a,
    {
        let filter = doc! {"_id": id};
        let result = self
            .schema::<TYPE>()
            .find_one_and_delete(filter, None)
            .await?;
        Ok(result)
    }
}
