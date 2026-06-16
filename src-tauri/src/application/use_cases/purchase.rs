use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    aggregates::purchase::Purchase,
    entities::{inventory::Inventory, purchase_item::PurchaseItem},
    events::purchase::PurchaseCompleted,
    repositories::{
        inventory::InventoryRepository, inventory_movement::InventoryMovementRepository,
        purchase::PurchaseRepository,
    },
    services::inventory_movement::{self, MovementResult},
};

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::{criteria::{Criteria, PaginatedResult}, logger::Logger};

use super::super::dtos::{
    operations::{PurchaseOperationOutput, RegisterPurchaseOperationInput},
    shared::CriteriaInput,
};

#[async_trait]
pub trait ForPurchaseUseCases {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Purchase>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Purchase>;
    async fn register_purchase(
        &self,
        input: RegisterPurchaseOperationInput,
    ) -> Result<PurchaseOperationOutput>;
}

pub struct ForPurchaseInteractor<
    R: PurchaseRepository,
    I: InventoryRepository,
    M: InventoryMovementRepository,
> {
    repo: R,
    inventory_repo: I,
    movement_repo: M,
    event_bus: EventBus,
    logger: Box<dyn Logger>,
}

impl<R: PurchaseRepository, I: InventoryRepository, M: InventoryMovementRepository>
    ForPurchaseInteractor<R, I, M>
{
    pub fn new(
        repo: R,
        inventory_repo: I,
        movement_repo: M,
        event_bus: EventBus,
        logger: Box<dyn Logger>,
    ) -> Self {
        Self {
            repo,
            inventory_repo,
            movement_repo,
            event_bus,
            logger,
        }
    }

    async fn resolve_inventory(
        &self,
        product_id: Uuid,
        store_id: Uuid,
    ) -> Result<Inventory> {
        let inventory = self
            .inventory_repo
            .find_by_product_and_store(product_id, store_id)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No inventory found for product {} in store {}",
                    product_id,
                    store_id
                )
            })?;

        if !inventory.is_active() {
            anyhow::bail!(
                "Inventory {} is inactive (product {})",
                inventory.id,
                product_id
            );
        }

        Ok(inventory)
    }

    async fn handle_item_movement(
        &self,
        inventory_id: Uuid,
        movement_fn: impl FnOnce(&Inventory) -> Result<MovementResult>,
    ) -> Result<MovementResult> {
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

        self.publish_movement_events(&result).await;

        Ok(result)
    }

    async fn publish_movement_events(&self, result: &MovementResult) {
        self.publish_event(DomainEvent::InventoryMovementRecorded(
            result.movement_recorded_event(),
        ))
        .await;

        if let Some(event) = result.low_stock_event() {
            self.publish_event(DomainEvent::LowStockDetected(event))
                .await;
        }

        if let Some(event) = result.stock_out_event() {
            self.publish_event(DomainEvent::StockOutDetected(event))
                .await;
        }
    }

    async fn publish_event(&self, event: DomainEvent) {
        if let Err(e) = self.event_bus.publish(event).await {
            self.logger
                .error(&format!("[Purchase] Failed to publish event: {}", e));
        }
    }
}

#[async_trait]
impl<
        R: PurchaseRepository + Sync + Send,
        I: InventoryRepository + Sync + Send,
        M: InventoryMovementRepository + Sync + Send,
    > ForPurchaseUseCases for ForPurchaseInteractor<R, I, M>
{
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Purchase>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Purchase> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Purchase not found"))
    }

    async fn register_purchase(
        &self,
        input: RegisterPurchaseOperationInput,
    ) -> Result<PurchaseOperationOutput> {
        let store_id = Uuid::parse_str(&input.store_id)?;
        let supplier_id = input
            .supplier_id
            .as_ref()
            .map(|id| Uuid::parse_str(id))
            .transpose()?;

        if input.items.is_empty() {
            anyhow::bail!("Purchase must have at least one item");
        }

        let purchase_id = Uuid::new_v4();

        let mut purchase_items: Vec<PurchaseItem> = Vec::with_capacity(input.items.len());

        for item_input in &input.items {
            let product_id = Uuid::parse_str(&item_input.product_id)?;
            let unit_cost = item_input.unit_cost.parse::<Decimal>().map_err(|e| {
                anyhow::anyhow!("Invalid unit_cost '{}': {}", item_input.unit_cost, e)
            })?;

            let inventory = self.resolve_inventory(product_id, store_id).await?;

            let purchase_item =
                PurchaseItem::new(purchase_id, inventory.id, item_input.quantity, unit_cost)?;

            purchase_items.push(purchase_item);
        }

        let total_cost = Purchase::calculate_total_cost(&purchase_items)?;
        let purchase = Purchase::restore(
            purchase_id,
            supplier_id,
            store_id,
            total_cost,
            Utc::now(),
            input.description,
            input.purchase_code,
            purchase_items,
        )?;

        self.repo.create(&purchase).await?;

        let mut movements_registered = 0usize;
        for item in &purchase.items {
            match self
                .handle_item_movement(item.inventory_id, |inv| {
                    inventory_movement::calculate_purchase(
                        inv,
                        item.quantity.value(),
                        None,
                        item.id,
                    )
                })
                .await
            {
                Ok(_) => movements_registered += 1,
                Err(e) => {
                    self.logger.error(&format!(
                        "[Purchase] Failed to register movement for item {}: {}",
                        item.id, e
                    ));
                }
            }
        }

        self.publish_event(DomainEvent::PurchaseCompleted(
            PurchaseCompleted::from_purchase(&purchase),
        ))
        .await;

        Ok(PurchaseOperationOutput::from_purchase(
            &purchase,
            movements_registered,
        ))
    }
}
