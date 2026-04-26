//! Runs all database migrations against the configured SQL database instance.
//!
//! Requires the `DATABASE_URL` environment variable to be set. Intended to run
//! as a one-shot container before the API starts.

use sqlx::PgPool;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db = PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("db connect");

    sqlx::migrate!("../migrations")
        .run(&db)
        .await
        .expect("migrations");

    info!("migrations applied");
}
