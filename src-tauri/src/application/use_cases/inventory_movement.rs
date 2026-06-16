use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::inventory::Inventory,
    repositories::{
        inventory::InventoryRepository, inventory_movement::InventoryMovementRepository,
    },
    services::inventory_movement::{self, MovementResult},
};

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::logger::Logger;

use super::super::dtos::inventory_movement::{
    InventoryMovementOutput, RegisterRestockInput, RegisterShrinkageInput,
};

#[async_trait]
pub trait ForInventoryMovementUseCases {
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

        self.publish_event(DomainEvent::InventoryMovementRecorded(
            result.movement_recorded_event(),
        ))
        .await;

        if let Some(event) = result.low_stock_event() {
            self.publish_event(DomainEvent::LowStockDetected(event)).await;
        }

        if let Some(event) = result.stock_out_event() {
            self.publish_event(DomainEvent::StockOutDetected(event)).await;
        }

        Ok(InventoryMovementOutput::new(
            &result.movement,
            result.requires_restock,
            result.is_out_of_stock,
        ))
    }

    async fn publish_event(&self, event: DomainEvent) {
        if let Err(e) = self.event_bus.publish(event).await {
            self.logger.error(&format!(
                "[InventoryMovement] Failed to publish event: {}",
                e
            ));
        }
    }
}

#[async_trait]
impl<I: InventoryRepository + Sync + Send, M: InventoryMovementRepository + Sync + Send>
    ForInventoryMovementUseCases for ForInventoryMovementInteractor<I, M>
{
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
