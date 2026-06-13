//! Dependency injection / composition root.
//!
//! Wires all infrastructure and application layers together.
//! Responsibility boundaries:
//! - `sqlite` module → pool creation + migrations only
//! - This module → repos, use cases, app state, logging, event dispatcher, and Tauri setup

use anyhow::Result;
use sqlx::SqlitePool;
use tauri::Manager;

use crate::application::use_cases::inventory::ForInventoryInteractor;
use crate::application::use_cases::inventory_movement::ForInventoryMovementInteractor;
use crate::application::use_cases::operations::ForOperationsInteractor;
use crate::application::use_cases::product::ForProductInteractor;
use crate::application::use_cases::purchase::ForPurchaseInteractor;
use crate::application::use_cases::sale::ForSaleInteractor;
use crate::application::use_cases::store::ForStoreInteractor;
use crate::application::use_cases::supplier::ForSupplierInteractor;
use crate::application::use_cases::supply_agreement::ForSupplyAgreementInteractor;
use crate::infrastructure::events::event_bus::EventBus;
use crate::infrastructure::events::event_dispatcher::EventDispatcher;
use crate::infrastructure::logging::console_logger::ConsoleLogger;
use crate::infrastructure::persistence::repositories::inventory::SqliteInventoryRepository;
use crate::infrastructure::persistence::repositories::inventory_movement::SqliteInventoryMovementRepository;
use crate::infrastructure::persistence::repositories::product::SqliteProductRepository;
use crate::infrastructure::persistence::repositories::purchase::SqlitePurchaseRepository;
use crate::infrastructure::persistence::repositories::sale::SqliteSalesRepository;
use crate::infrastructure::persistence::repositories::store::SqliteStoreRepository;
use crate::infrastructure::persistence::repositories::supplier::SqliteSupplierRepository;
use crate::infrastructure::persistence::repositories::supply_agreement::SqliteSupplyAgreementRepository;
use crate::infrastructure::persistence::sqlite;

// ---------------------------------------------------------------------------
// Use cases — the public API consumed by Tauri commands
// ---------------------------------------------------------------------------

pub struct UseCases {
    pub product: ForProductInteractor<SqliteProductRepository>,
    pub inventory: ForInventoryInteractor<SqliteInventoryRepository>,
    pub inventory_movement: ForInventoryMovementInteractor<
        SqliteInventoryRepository,
        SqliteInventoryMovementRepository,
    >,
    pub operations: ForOperationsInteractor<
        SqliteInventoryRepository,
        SqliteInventoryMovementRepository,
        SqlitePurchaseRepository,
        SqliteSalesRepository,
    >,
    pub sale: ForSaleInteractor<SqliteSalesRepository>,
    pub purchase: ForPurchaseInteractor<SqlitePurchaseRepository>,
    pub store: ForStoreInteractor<SqliteStoreRepository>,
    pub supplier: ForSupplierInteractor<SqliteSupplierRepository>,
    pub supply_agreement: ForSupplyAgreementInteractor<SqliteSupplyAgreementRepository>,
}

// ---------------------------------------------------------------------------
// App state — registered in Tauri, accessed by `#[tauri::command]` handlers
// ---------------------------------------------------------------------------

pub struct AppState {
    pub use_cases: UseCases,
    pub event_bus: EventBus,
}

// ---------------------------------------------------------------------------
// Repositories — implementation detail, NOT exposed via AppState
// ---------------------------------------------------------------------------

struct Repositories {
    store: SqliteStoreRepository,
    product: SqliteProductRepository,
    supplier: SqliteSupplierRepository,
    inventory: SqliteInventoryRepository,
    inventory_movement: SqliteInventoryMovementRepository,
    supply_agreement: SqliteSupplyAgreementRepository,
    purchase: SqlitePurchaseRepository,
    sales: SqliteSalesRepository,
}

impl Repositories {
    fn new(pool: SqlitePool) -> Self {
        Self {
            store: SqliteStoreRepository::new(pool.clone()),
            product: SqliteProductRepository::new(pool.clone()),
            supplier: SqliteSupplierRepository::new(pool.clone()),
            inventory: SqliteInventoryRepository::new(pool.clone()),
            inventory_movement: SqliteInventoryMovementRepository::new(pool.clone()),
            supply_agreement: SqliteSupplyAgreementRepository::new(pool.clone()),
            purchase: SqlitePurchaseRepository::new(pool.clone()),
            sales: SqliteSalesRepository::new(pool),
        }
    }
}

// ---------------------------------------------------------------------------
// Setup — the composition root called from Tauri's `setup` hook
// ---------------------------------------------------------------------------

pub fn setup_app(app: &mut tauri::App) -> Result<()> {
    // 1. Async runtime (needed because Tauri's setup is synchronous)
    let rt = tokio::runtime::Runtime::new()?;

    // 2. Database
    let app_data_dir = app.path().app_data_dir()?;
    let pool = rt.block_on(sqlite::create_pool(&app_data_dir))?;
    rt.block_on(sqlite::run_migrations(&pool))?;

    // 3. Logger
    let logger: Box<dyn crate::shared::logger::Logger> = Box::new(ConsoleLogger);

    // 4. Event bus
    let event_bus = EventBus::new(256);

    // 5. Repositories (internal — not stored in AppState)
    let repos = Repositories::new(pool);

    // 6. Use cases
    let use_cases = UseCases {
        product: ForProductInteractor::new(repos.product),
        inventory: ForInventoryInteractor::new(repos.inventory.clone()),
        inventory_movement: ForInventoryMovementInteractor::new(
            repos.inventory.clone(),
            repos.inventory_movement.clone(),
            event_bus.clone(),
            Box::new(ConsoleLogger),
        ),
        operations: ForOperationsInteractor::new(
            repos.inventory,
            repos.inventory_movement,
            repos.purchase.clone(),
            repos.sales.clone(),
            event_bus.clone(),
            Box::new(ConsoleLogger),
        ),
        sale: ForSaleInteractor::new(repos.sales),
        purchase: ForPurchaseInteractor::new(repos.purchase),
        store: ForStoreInteractor::new(repos.store),
        supplier: ForSupplierInteractor::new(repos.supplier),
        supply_agreement: ForSupplyAgreementInteractor::new(repos.supply_agreement),
    };

    // 7. Register managed state (only use cases + event bus exposed)
    app.manage(AppState {
        use_cases,
        event_bus: event_bus.clone(),
    });

    // 8. Spawn background event dispatcher (frontend emission, audit, etc.)
    let handle = app.handle().clone();
    EventDispatcher::spawn(handle, event_bus, logger);

    Ok(())
}
