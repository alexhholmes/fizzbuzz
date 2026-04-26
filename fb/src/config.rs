use std::fmt;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use tracing_appender::rolling::Rotation;

#[derive(Parser)]
pub struct Config {
    #[arg(long, env = "ADDR", default_value = "0.0.0.0")]
    pub addr: String,

    #[arg(long, env = "PORT", default_value_t = 3000)]
    pub port: u16,

    #[arg(long, env = "DB_BACKEND", default_value = "postgres")]
    pub db_backend: DatabaseBackend,

    #[arg(long, env = "DB_NAME")]
    pub db_name: String,

    #[arg(long, env = "DB_HOST", default_value = "localhost")]
    pub db_host: String,

    #[arg(long, env = "DB_PORT", default_value_t = 5432)]
    pub db_port: u16,

    #[arg(long, env = "DB_PATH", default_value = "db.sqlite")]
    pub db_path: String,

    #[arg(long, env = "REDIS_HOST", default_value = "localhost")]
    pub redis_host: String,

    #[arg(long, env = "REDIS_PORT", default_value_t = 6379)]
    pub redis_port: u16,

    #[arg(long, env = "REDIS_DB", default_value_t = 0)]
    pub redis_db: u8,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: LogLevel,

    #[arg(long, env = "LOG_ROTATION", default_value = "daily")]
    pub log_rotation: LogRotation,

    #[arg(long, env = "LOG_PREFIX", default_value = "api")]
    pub log_prefix: String,

    #[arg(long, env = "LOG_SUFFIX", default_value = "log")]
    pub log_suffix: String,

    #[arg(long, env = "LOG_MAX_FILES", default_value_t = 7)]
    pub log_max_files: usize,
}

#[derive(Clone, ValueEnum)]
pub enum DatabaseBackend {
    #[value(name = "postgres")]
    Postgres,
    #[value(name = "sqlite")]
    Sqlite,
}

#[derive(Clone, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().unwrap().get_name().fmt(f)
    }
}

#[derive(Clone)]
pub enum LogRotation {
    Daily,
    Hourly,
    Minutely,
    Never,
}

impl From<LogRotation> for Rotation {
    fn from(r: LogRotation) -> Self {
        match r {
            LogRotation::Daily => Self::DAILY,
            LogRotation::Hourly => Self::HOURLY,
            LogRotation::Minutely => Self::MINUTELY,
            LogRotation::Never => Self::NEVER,
        }
    }
}

impl ValueEnum for LogRotation {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Daily, Self::Hourly, Self::Minutely, Self::Never]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Daily => PossibleValue::new("daily").alias("day"),
            Self::Hourly => PossibleValue::new("hourly").alias("hour"),
            Self::Minutely => PossibleValue::new("minutely").alias("minute"),
            Self::Never => PossibleValue::new("never"),
        })
    }
}
