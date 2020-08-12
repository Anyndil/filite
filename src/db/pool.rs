use crate::config::Config;
use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;

#[tracing::instrument(level = "debug")]
pub async fn build(config: &Config) -> Result<&'static SqlitePool> {
    let mut options: SqlitePoolOptions = Default::default();

    if let Some(ms) = config.pool.max_connections {
        options = options.max_connections(ms);
    }
    if let Some(ms) = config.pool.min_connections {
        options = options.min_connections(ms);
    }

    if let Some(ct) = config.pool.connect_timeout {
        options = options.connect_timeout(Duration::from_millis(ct));
    }
    if let Some(it) = config.pool.idle_timeout {
        options = options.idle_timeout(Duration::from_millis(it));
    }

    if let Some(ml) = config.pool.max_lifetime {
        options = options.max_lifetime(Duration::from_millis(ml));
    }

    let pool = options
        .connect(&format!("sqlite://{}", config.database_url))
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(&*Box::leak(Box::new(pool)))
}
