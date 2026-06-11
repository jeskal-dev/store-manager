pub mod inventory;
pub mod product;
pub mod purchase;
pub mod sale;
pub mod store;
pub mod supplier;
pub mod supply_agreement;

pub use inventory::InventoryRepository;
pub use product::ProductRepository;
pub use purchase::PurchaseRepository;
pub use sale::SalesRepository;
pub use store::StoreRepository;
pub use supplier::SupplierRepository;
pub use supply_agreement::SupplyAgreementRepository;

use async_trait::async_trait;
use uuid::Uuid;

use anyhow::Result;

use crate::shared::criteria::{Criteria, PaginatedResult};

#[async_trait]
pub trait Repository<T: Send + Sync>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>>;
    async fn search(&self, criteria: Criteria<T>) -> Result<PaginatedResult<T>>;
    async fn create(&self, entity: &T) -> Result<()>;
    async fn update(&self, entity: &T) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
