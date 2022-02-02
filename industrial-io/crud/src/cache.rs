//! Cache

use std::sync::Arc;

#[allow(dead_code)]
#[derive(Clone)]
pub struct RedisClient {
    connection: Arc<redis::aio::Connection>,
}

impl RedisClient {
    pub async fn new<U: AsRef<str>>(uri: U) -> anyhow::Result<Self> {
        let client = redis::Client::open(uri.as_ref())?;
        let connection = client.get_async_connection().await?;
        let connection = Arc::new(connection);

        Ok(RedisClient { connection })
    }
}
