//! Persistence service.

use mongodb::options::ClientOptions;

#[derive(Clone)]
pub struct MongoClient {
    client: mongodb::Client,
}

pub trait MongoClientFactory: Send + Sync {
    fn collection<T>(&self, db: &str, name: &str) -> mongodb::Collection<T>;
}

impl MongoClient {
    pub async fn new(uri: &str, name: &str) -> anyhow::Result<Self> {
        let mut co = ClientOptions::parse(uri).await?;
        co.app_name = Some(name.to_string());

        let client = mongodb::Client::with_options(co)?;
        Ok(MongoClient { client })
    }

    pub async fn show_dbs(&self) -> anyhow::Result<Vec<String>> {
        let dbs = self.client.list_database_names(None, None).await?;
        Ok(dbs)
    }
}

impl MongoClientFactory for MongoClient {
    fn collection<T>(&self, db: &str, name: &str) -> mongodb::Collection<T> {
        self.client.database(db).collection(name)
    }
}

#[cfg(test)]
mod test_persistence {
    use super::*;

    #[tokio::test]
    async fn test_show_dbs() {
        let uri = "mongodb://localhost:27017";

        let client = MongoClient::new(uri, "test").await;
        assert!(client.is_ok());

        let client = client.unwrap();

        let dbs = client.show_dbs().await;
        assert!(dbs.is_ok());
    }
}
