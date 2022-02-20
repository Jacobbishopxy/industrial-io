//! Provider
//!
//! Used for implementing domain specific logic for CRUD operations.

use crud::{MongoClient, MongoClientFactory, RedisClient};

pub struct Provider {
    pub cache_client: RedisClient,
    pub persistence_client: MongoClient,
}

#[derive(Default)]
pub struct ProviderBuilder {
    pub cache_client: Option<RedisClient>,
    pub persistence_client: Option<MongoClient>,
}

impl Provider {
    pub fn create() -> ProviderBuilder {
        ProviderBuilder {
            cache_client: None,
            persistence_client: None,
        }
    }
}

impl MongoClientFactory for Provider {
    fn client(&self) -> &MongoClient {
        &self.persistence_client
    }
}

impl ProviderBuilder {
    pub async fn cache_uri<U: AsRef<str>>(&mut self, uri: U) -> anyhow::Result<&mut Self> {
        let client = RedisClient::new(uri.as_ref()).await?;
        self.cache_client = Some(client);
        Ok(self)
    }

    pub async fn persistence_uri<U: AsRef<str>>(&mut self, uri: U) -> anyhow::Result<&mut Self> {
        let client = MongoClient::new(uri.as_ref(), "", "").await?;
        self.persistence_client = Some(client);
        Ok(self)
    }

    pub fn persistence_database<T: Into<String>>(
        &mut self,
        database: T,
    ) -> anyhow::Result<&mut Self> {
        let client = self
            .persistence_client
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Persistence client not set"))?;
        client.set_database(database);
        Ok(self)
    }

    pub fn persistence_collection<T: Into<String>>(
        &mut self,
        collection: T,
    ) -> anyhow::Result<&mut Self> {
        let client = self
            .persistence_client
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Persistence client not set"))?;
        client.set_collection(collection);
        Ok(self)
    }

    pub fn build(&mut self) -> anyhow::Result<Provider> {
        let cache_client = self
            .cache_client
            .to_owned()
            .ok_or_else(|| anyhow::anyhow!("Cache client not set"))?;
        let persistence_client = self
            .persistence_client
            .to_owned()
            .ok_or_else(|| anyhow::anyhow!("Persistence client not set"))?;
        if persistence_client.database.is_empty() {
            return Err(anyhow::anyhow!("Persistence database not set"));
        }
        if persistence_client.collection.is_empty() {
            return Err(anyhow::anyhow!("Persistence collection not set"));
        }

        Ok(Provider {
            cache_client,
            persistence_client,
        })
    }
}

#[cfg(test)]
mod test_provider {
    use super::*;

    const CACHE_URI: &str = "redis://localhost:6379";
    const PERSISTENCE_URI: &str = "mongodb://root:secret@localhost:27017";
    const PERSISTENCE_DATABASE: &str = "test";
    const PERSISTENCE_COLLECTION: &str = "test";

    #[tokio::test]
    async fn provider_create_and_build_is_ok() {
        let rp = Provider::create()
            .cache_uri(CACHE_URI)
            .await
            .unwrap()
            .persistence_uri(PERSISTENCE_URI)
            .await
            .unwrap()
            .persistence_database(PERSISTENCE_DATABASE)
            .unwrap()
            .persistence_collection(PERSISTENCE_COLLECTION)
            .unwrap()
            .build();

        assert!(rp.is_ok());
    }
}
