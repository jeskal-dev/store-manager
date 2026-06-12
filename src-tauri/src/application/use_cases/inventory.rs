use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::repositories::inventory::InventoryRepository;
use crate::domain::value_objects::common::Quantity;
use crate::domain::{entities::inventory::Inventory, value_objects::common::Code};

use super::super::dtos::inventory::{CreateInventoryInput, UpdateInventoryInput};

#[async_trait]
pub trait ForInventoryUseCases {
    async fn create(&self, input: CreateInventoryInput) -> Result<Inventory>;
    async fn update(&self, id: Uuid, input: UpdateInventoryInput) -> Result<Inventory>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct ForInventoryInteractor<R: InventoryRepository> {
    repo: R,
}

impl<R: InventoryRepository> ForInventoryInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: InventoryRepository + Sync> ForInventoryUseCases for ForInventoryInteractor<R> {
    async fn create(&self, input: CreateInventoryInput) -> Result<Inventory> {
        let price_local = input.parse_price_local()?;
        let store_id = Uuid::parse_str(&input.store_id)?;
        let product_id = Uuid::parse_str(&input.product_id)?;

        if self
            .repo
            .find_by_product_and_store(product_id, store_id)
            .await?
            .is_some()
        {
            anyhow::bail!("Inventory already exists for this product and store");
        }

        let inventory = Inventory::new(
            input.inventory_code,
            store_id,
            product_id,
            input.quantity,
            price_local,
            input.min_stock,
            input.status,
            input.active,
        )?;

        self.repo.create(&inventory).await?;
        Ok(inventory)
    }
    async fn update(&self, id: Uuid, input: UpdateInventoryInput) -> Result<Inventory> {
        let mut inventory = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;

        if let Some(ref code) = input.inventory_code {
            inventory.inventory_code = Code::new(code.clone())?;
        }
        if let Some(price) = input.parse_price_local()? {
            inventory.update_price(price)?;
        }
        if let Some(min_stock) = input.min_stock {
            inventory.min_stock = Quantity::new(min_stock)?;
            inventory.calculate_status();
        }
        if let Some(active) = input.active {
            if active {
                inventory.activate();
            } else {
                inventory.deactivate();
            }
        }

        self.repo.update(&inventory).await?;
        Ok(inventory)
    }
    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
