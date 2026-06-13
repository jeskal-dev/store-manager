use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::inventory::Inventory,
    events::inventory::{InventoryMovementRecorded, LowStockDetected, StockOutDetected},
    repositories::{
        inventory::InventoryRepository, inventory_movement::InventoryMovementRepository,
    },
    services::inventory_movement::{self, MovementResult},
};

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::logger::Logger;

use super::super::dtos::inventory_movement::{
    InventoryMovementOutput, RegisterPurchaseInput, RegisterRestockInput, RegisterSaleInput,
    RegisterShrinkageInput,
};

#[async_trait]
pub trait ForInventoryMovementUseCases {
    async fn register_purchase(
        &self,
        input: RegisterPurchaseInput,
    ) -> Result<InventoryMovementOutput>;
    async fn register_sale(&self, input: RegisterSaleInput) -> Result<InventoryMovementOutput>;
    async fn register_restock(
        &self,
        input: RegisterRestockInput,
    ) -> Result<InventoryMovementOutput>;
    async fn register_shrinkage(
        &self,
        input: RegisterShrinkageInput,
    ) -> Result<InventoryMovementOutput>;
}

pub struct ForInventoryMovementInteractor<I: InventoryRepository, M: InventoryMovementRepository> {
    inventory_repo: I,
    movement_repo: M,
    event_bus: EventBus,
    logger: Box<dyn Logger>,
}

impl<I: InventoryRepository, M: InventoryMovementRepository> ForInventoryMovementInteractor<I, M> {
    pub fn new(
        inventory_repo: I,
        movement_repo: M,
        event_bus: EventBus,
        logger: Box<dyn Logger>,
    ) -> Self {
        Self {
            inventory_repo,
            movement_repo,
            event_bus,
            logger,
        }
    }

    async fn execute_movement(
        &self,
        inventory_id: Uuid,
        movement_fn: impl FnOnce(&Inventory) -> Result<MovementResult>,
    ) -> Result<InventoryMovementOutput> {
        let inventory = self
            .inventory_repo
            .find_by_id(inventory_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found: {}", inventory_id))?;

        if !inventory.is_active() {
            anyhow::bail!("Inventory is inactive: {}", inventory_id);
        }

        let result = movement_fn(&inventory)?;

        self.movement_repo
            .save_with_inventory_update(&result.movement, &result.updated_inventory)
            .await?;

        let movement_event = InventoryMovementRecorded::build_movement_event(
            &result.movement,
            &result.updated_inventory,
        );

        if let Err(e) = self
            .event_bus
            .publish(DomainEvent::InventoryMovementRecorded(movement_event))
            .await
        {
            self.logger.error(&format!(
                "[InventoryMovement] Failed to publish movement event: {}",
                e
            ));
        }

        if result.requires_restock {
            let low_stock = LowStockDetected {
                inventory_id: result.updated_inventory.id,
                store_id: result.updated_inventory.store_id,
                product_id: result.updated_inventory.product_id,
                current_quantity: result.updated_inventory.quantity.value(),
                min_stock: result.updated_inventory.min_stock.value(),
                occurred_at: result.movement.date,
            };
            if let Err(e) = self
                .event_bus
                .publish(DomainEvent::LowStockDetected(low_stock))
                .await
            {
                self.logger.error(&format!(
                    "[InventoryMovement] Failed to publish low-stock event: {}",
                    e
                ));
            }
        }

        if result.is_out_of_stock {
            let stock_out = StockOutDetected {
                inventory_id: result.updated_inventory.id,
                store_id: result.updated_inventory.store_id,
                product_id: result.updated_inventory.product_id,
                occurred_at: result.movement.date,
            };
            if let Err(e) = self
                .event_bus
                .publish(DomainEvent::StockOutDetected(stock_out))
                .await
            {
                self.logger.error(&format!(
                    "[InventoryMovement] Failed to publish stock-out event: {}",
                    e
                ));
            }
        }

        Ok(InventoryMovementOutput::new(
            &result.movement,
            result.requires_restock,
            result.is_out_of_stock,
        ))
    }
}

#[async_trait]
impl<I: InventoryRepository + Sync + Send, M: InventoryMovementRepository + Sync + Send>
    ForInventoryMovementUseCases for ForInventoryMovementInteractor<I, M>
{
    async fn register_purchase(
        &self,
        input: RegisterPurchaseInput,
    ) -> Result<InventoryMovementOutput> {
        let inventory_id = Uuid::parse_str(&input.inventory_id)?;
        let purchase_item_id = Uuid::parse_str(&input.purchase_item_id)?;

        if input.quantity <= 0 {
            anyhow::bail!("Purchase quantity must be positive");
        }

        self.execute_movement(inventory_id, |inventory| {
            inventory_movement::calculate_purchase(
                inventory,
                input.quantity,
                input.description.clone(),
                purchase_item_id,
            )
        })
        .await
    }

    async fn register_sale(&self, input: RegisterSaleInput) -> Result<InventoryMovementOutput> {
        let inventory_id = Uuid::parse_str(&input.inventory_id)?;
        let sale_item_id = Uuid::parse_str(&input.sale_item_id)?;

        if input.quantity <= 0 {
            anyhow::bail!("Sale quantity must be positive");
        }

        self.execute_movement(inventory_id, |inventory| {
            inventory_movement::calculate_sale(
                inventory,
                input.quantity,
                input.description.clone(),
                sale_item_id,
            )
        })
        .await
    }

    async fn register_restock(
        &self,
        input: RegisterRestockInput,
    ) -> Result<InventoryMovementOutput> {
        let inventory_id = Uuid::parse_str(&input.inventory_id)?;

        if input.quantity <= 0 {
            anyhow::bail!("Restock quantity must be positive");
        }

        self.execute_movement(inventory_id, |inventory| {
            inventory_movement::calculate_restock(
                inventory,
                input.quantity,
                input.description.clone(),
            )
        })
        .await
    }

    async fn register_shrinkage(
        &self,
        input: RegisterShrinkageInput,
    ) -> Result<InventoryMovementOutput> {
        let inventory_id = Uuid::parse_str(&input.inventory_id)?;

        if input.quantity <= 0 {
            anyhow::bail!("Shrinkage quantity must be positive");
        }

        self.execute_movement(inventory_id, |inventory| {
            inventory_movement::calculate_shrinkage(
                inventory,
                input.quantity,
                input.description.clone(),
            )
        })
        .await
    }
}
