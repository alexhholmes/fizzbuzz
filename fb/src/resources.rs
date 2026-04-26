use crate::cache::Cache;
use crate::config::Config;
use crate::database::Database;

#[derive(Clone)]
pub struct Resources {
    pub db: Database,
    pub cache: Cache,
}

impl Resources {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db = Database::new(config).await?;
        let cache = Cache::new(config).await;

        Ok(Self { db, cache })
    }
}
