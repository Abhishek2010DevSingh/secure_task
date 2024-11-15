use anyhow::Context;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::config::CONFIG;

pub async fn get_database_object() -> anyhow::Result<Pool<Sqlite>> {
    let database = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&CONFIG.database_url)
        .await
        .context("Failed to connect database")?;

    sqlx::migrate!()
        .run(&database)
        .await
        .context("Error at Running database migration")?;

    Ok(database)
}
