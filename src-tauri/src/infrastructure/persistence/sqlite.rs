use std::fs;

use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tauri::Manager;

use crate::infrastructure::persistence::repositories::inventory::SqliteInventoryRepository;
use crate::infrastructure::persistence::repositories::product::SqliteProductRepository;
use crate::infrastructure::persistence::repositories::purchase::SqlitePurchaseRepository;
use crate::infrastructure::persistence::repositories::sale::SqliteSalesRepository;
use crate::infrastructure::persistence::repositories::store::SqliteStoreRepository;
use crate::infrastructure::persistence::repositories::supplier::SqliteSupplierRepository;
use crate::infrastructure::persistence::repositories::supply_agreement::SqliteSupplyAgreementRepository;

pub struct Repositories {
    pub store: SqliteStoreRepository,
    pub product: SqliteProductRepository,
    pub supplier: SqliteSupplierRepository,
    pub inventory: SqliteInventoryRepository,
    pub supply_agreement: SqliteSupplyAgreementRepository,
    pub purchase: SqlitePurchaseRepository,
    pub sales: SqliteSalesRepository,
}

pub struct UseCases {}

pub struct AppState {
    pub repos: Repositories,
    pub use_cases: UseCases,
}

pub fn setup_app(app: &mut tauri::App) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let pool = rt.block_on(create_pool(app))?;
    rt.block_on(run_migrations(&pool))?;

    let repos = Repositories {
        store: SqliteStoreRepository::new(pool.clone()),
        product: SqliteProductRepository::new(pool.clone()),
        supplier: SqliteSupplierRepository::new(pool.clone()),
        inventory: SqliteInventoryRepository::new(pool.clone()),
        supply_agreement: SqliteSupplyAgreementRepository::new(pool.clone()),
        purchase: SqlitePurchaseRepository::new(pool.clone()),
        sales: SqliteSalesRepository::new(pool),
    };

    let use_cases = UseCases {};

    app.manage(AppState { repos, use_cases });

    Ok(())
}

async fn create_pool(app: &tauri::App) -> Result<SqlitePool> {
    let app_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("store_manager.db");
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

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    MIGRATOR.run(pool).await?;
    Ok(())
}
