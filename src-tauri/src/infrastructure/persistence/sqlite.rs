use std::fs;
use std::path::Path;

use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn create_pool(app_data_dir: &Path) -> Result<SqlitePool> {
    fs::create_dir_all(app_data_dir)?;
    let db_path = app_data_dir.join("store_manager.db");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path.to_str().unwrap())
        .await?;

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    MIGRATOR.run(pool).await?;
    Ok(())
}
