//! Persistence service.

use std::borrow::Cow;

#[derive(Clone)]
pub struct MongoClient {
    client: mongodb::Client,
    pub database: String,
    pub collection: String,
}

pub trait MongoClientFactory: Send + Sync {
    fn database(&self) -> Cow<str>;

    fn set_database(&mut self, database: &str);

    fn collection(&self) -> Cow<str>;

    fn set_collection(&mut self, collection: &str);

    fn coll<T>(&self) -> mongodb::Collection<T>;
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
