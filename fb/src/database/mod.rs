use sqlx::{PgPool, SqlitePool};

use crate::config::{Config, DatabaseBackend};
pub use crate::models::exception::Exception;
pub use crate::models::user::User;

#[derive(Clone)]
enum Pool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let pool = match config.db_backend {
            DatabaseBackend::Postgres => {
                let url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    std::env::var("DB_USER").expect("DB_USER"),
                    std::env::var("DB_PASSWORD").expect("DB_PASSWORD"),
                    config.db_host,
                    config.db_port,
                    config.db_name,
                );
                Pool::Postgres(PgPool::connect(&url).await?)
            }
            DatabaseBackend::Sqlite => Pool::Sqlite(SqlitePool::connect(&config.db_path).await?),
        };
        Ok(Self { pool })
    }

    pub async fn get_exception_list(&self) -> Result<Vec<Exception>, sqlx::Error> {
        match &self.pool {
            Pool::Postgres(p) => {
                sqlx::query_as::<_, Exception>(
                    "SELECT n, result, created_at, modified_at FROM exceptions",
                )
                .fetch_all(p)
                .await
            }
            Pool::Sqlite(p) => {
                sqlx::query_as::<_, Exception>(
                    "SELECT n, result, created_at, modified_at FROM exceptions",
                )
                .fetch_all(p)
                .await
            }
        }
    }

    pub async fn upsert_exception(
        &self,
        n: i64,
        result: String,
    ) -> Result<(Exception, bool), sqlx::Error> {
        match &self.pool {
            Pool::Postgres(p) => {
                let exists = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM exceptions WHERE n = $1)",
                )
                .bind(n)
                .fetch_one(p)
                .await?;

                let exc = Exception::new(n, result);
                let exception = sqlx::query_as::<_, Exception>(
                    "INSERT INTO exceptions (n, result, created_at, modified_at)
                     VALUES ($1, $2, $3, $4)
                     ON CONFLICT (n) DO UPDATE
                     SET result = EXCLUDED.result, modified_at = EXCLUDED.modified_at
                     RETURNING n, result, created_at, modified_at",
                )
                .bind(exc.n)
                .bind(&exc.result)
                .bind(exc.created_at)
                .bind(exc.modified_at)
                .fetch_one(p)
                .await?;

                Ok((exception, !exists))
            }
            Pool::Sqlite(p) => {
                let exists = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM exceptions WHERE n = ?)",
                )
                .bind(n)
                .fetch_one(p)
                .await?;

                let exc = Exception::new(n, result);
                let exception = sqlx::query_as::<_, Exception>(
                    "INSERT INTO exceptions (n, result, created_at, modified_at)
                     VALUES (?, ?, ?, ?)
                     ON CONFLICT(n) DO UPDATE
                     SET result = excluded.result, modified_at = excluded.modified_at
                     RETURNING n, result, created_at, modified_at",
                )
                .bind(exc.n)
                .bind(&exc.result)
                .bind(exc.created_at)
                .bind(exc.modified_at)
                .fetch_one(p)
                .await?;

                Ok((exception, !exists))
            }
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        match &self.pool {
            Pool::Postgres(p) => {
                sqlx::query_as::<_, User>(
                    "SELECT id, email, password_hash FROM users WHERE email = $1",
                )
                .bind(email)
                .fetch_optional(p)
                .await
            }
            Pool::Sqlite(p) => {
                sqlx::query_as::<_, User>(
                    "SELECT id, email, password_hash FROM users WHERE email = ?",
                )
                .bind(email)
                .fetch_optional(p)
                .await
            }
        }
    }
}
