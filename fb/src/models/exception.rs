use chrono::{DateTime, Utc};
use fb_macros::Cacheable;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Cacheable, Serialize, FromRow)]
pub struct Exception {
    pub n: i64,
    pub result: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Exception {
    pub fn new(n: i64, result: String) -> Self {
        let now = Utc::now();
        Exception {
            n,
            result,
            created_at: now,
            modified_at: now,
        }
    }

    pub fn update(&mut self) {
        self.modified_at = Utc::now();
    }
}
