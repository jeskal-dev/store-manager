use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use tauri::Manager;

pub async fn init_pool(db_path: &PathBuf) -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("migrations").run(&pool).await?;

    Ok(pool)
}

pub fn setup_database(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("store_manager.db");
    let pool = tauri::async_runtime::block_on(init_pool(&db_path))?;
    app.manage(pool);
    Ok(())
}
