use std::future::Future;

use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use tracing::warn;

use crate::config::Config;

#[derive(Clone)]
pub struct Cache {
    conn: Option<ConnectionManager>,
}

impl Cache {
    pub async fn new(config: &Config) -> Self {
        let url = format!("redis://{}:{}/{}", config.redis_host, config.redis_port, config.redis_db);
        match redis::Client::open(url.as_str()) {
            Err(e) => {
                warn!(error = %e, "redis unavailable, cache disabled");
                Self { conn: None }
            }
            Ok(client) => match ConnectionManager::new(client).await {
                Err(e) => {
                    warn!(error = %e, "redis connect failed, cache disabled");
                    Self { conn: None }
                }
                Ok(conn) => Self { conn: Some(conn) },
            },
        }
    }

    pub fn unavailable() -> Self {
        Self { conn: None }
    }

    pub async fn execute<F, Fut, T>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce(&mut ConnectionManager) -> Fut,
        Fut: Future<Output = redis::RedisResult<T>>,
    {
        let conn = self.conn.as_mut()?;
        match f(conn).await {
            Ok(val) => Some(val),
            Err(e) => {
                warn!(error = %e, "cache operation failed");
                None
            }
        }
    }

    pub async fn get(&mut self, key: &str) -> Option<String> {
        let conn = self.conn.as_mut()?;
        match conn.get::<_, Option<String>>(key).await {
            Ok(val) => val,
            Err(e) => {
                warn!(error = %e, key, "cache get failed");
                None
            }
        }
    }

    pub async fn set(&mut self, key: &str, value: &str, ttl_secs: u64) -> bool {
        let Some(conn) = self.conn.as_mut() else {
            return false;
        };
        match conn.set_ex::<_, _, ()>(key, value, ttl_secs).await {
            Ok(_) => true,
            Err(e) => {
                warn!(error = %e, key, "cache set failed");
                false
            }
        }
    }

    pub async fn del(&mut self, key: &str) -> bool {
        let Some(conn) = self.conn.as_mut() else {
            return false;
        };
        match conn.del::<_, ()>(key).await {
            Ok(_) => true,
            Err(e) => {
                warn!(error = %e, key, "cache del failed");
                false
            }
        }
    }

    pub async fn bf_add(&mut self, bloom_key: &str, item: &str) -> bool {
        let Some(conn) = self.conn.as_mut() else {
            return false;
        };
        match redis::cmd("BF.ADD")
            .arg(bloom_key)
            .arg(item)
            .query_async::<bool>(conn)
            .await
        {
            Ok(val) => val,
            Err(e) => {
                warn!(error = %e, bloom_key, item, "bf_add failed");
                false
            }
        }
    }

    pub async fn bf_exists(&mut self, bloom_key: &str, item: &str) -> Result<bool, ()> {
        let Some(conn) = self.conn.as_mut() else {
            return Err(());
        };
        match redis::cmd("BF.EXISTS")
            .arg(bloom_key)
            .arg(item)
            .query_async::<bool>(conn)
            .await
        {
            Ok(val) => Ok(val),
            Err(e) => {
                warn!(error = %e, bloom_key, item, "bf_exists failed");
                Err(())
            }
        }
    }

    pub async fn set_with_bf_add(
        &mut self,
        bloom_key: &str,
        item: &str,
        cache_key: &str,
        value: &str,
        ttl_secs: u64,
    ) -> bool {
        let Some(conn) = self.conn.as_mut() else {
            return false;
        };
        if let Err(e) = redis::cmd("BF.ADD")
            .arg(bloom_key)
            .arg(item)
            .query_async::<bool>(conn)
            .await
        {
            warn!(error = %e, bloom_key, item, "set_with_bloom bf_add failed");
            return false;
        }
        let Some(conn) = self.conn.as_mut() else {
            return false;
        };
        match conn.set_ex::<_, _, ()>(cache_key, value, ttl_secs).await {
            Ok(_) => true,
            Err(e) => {
                warn!(error = %e, cache_key, "set_with_bloom set failed");
                false
            }
        }
    }
}
