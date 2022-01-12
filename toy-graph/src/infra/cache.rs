//! Cache

use std::sync::Arc;

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct RedisClient {
    connection: Arc<redis::aio::Connection>,
}

#[allow(dead_code)]
impl RedisClient {
    pub async fn new(uri: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(uri)?;
        let connection = client.get_async_connection().await?;
        let connection = Arc::new(connection);

        Ok(RedisClient { connection })
    }
}
