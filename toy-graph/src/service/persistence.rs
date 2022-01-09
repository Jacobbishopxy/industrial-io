//! Persistence service.

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct MongoClient {
    client: mongodb::Client,
}

#[allow(dead_code)]
impl MongoClient {
    pub async fn new(uri: &str) -> anyhow::Result<Self> {
        let client = mongodb::Client::with_uri_str(uri).await?;
        Ok(MongoClient { client })
    }

    pub async fn show_dbs(&self) -> anyhow::Result<Vec<String>> {
        let dbs = self.client.list_database_names(None, None).await?;
        Ok(dbs)
    }
}

#[cfg(test)]
mod test_persistence {
    use super::*;

    #[tokio::test]
    async fn test_show_dbs() {
        let uri = "mongodb://localhost:27017";

        let client = MongoClient::new(uri).await;
        assert!(client.is_ok());

        let client = client.unwrap();

        let dbs = client.show_dbs().await;
        assert!(dbs.is_ok());
    }
}
