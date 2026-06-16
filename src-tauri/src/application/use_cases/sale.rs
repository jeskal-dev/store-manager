use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    aggregates::sale::Sale,
    entities::{inventory::Inventory, sale_item::SaleItem},
    events::sale::SaleCompleted,
    repositories::{
        inventory::InventoryRepository, inventory_movement::InventoryMovementRepository,
        sale::SalesRepository,
    },
    services::inventory_movement::{self, MovementResult},
    value_objects::sale::PaymentMethod,
};

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::{
    criteria::{Criteria, PaginatedResult},
    logger::Logger,
};

use super::super::dtos::{
    operations::{RegisterSaleOperationInput, SaleOperationOutput},
    shared::CriteriaInput,
};

#[async_trait]
pub trait ForSaleUseCases {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Sale>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Sale>;
    async fn register_sale(&self, input: RegisterSaleOperationInput)
        -> Result<SaleOperationOutput>;
}

pub struct ForSaleInteractor<
    S: SalesRepository,
    I: InventoryRepository,
    M: InventoryMovementRepository,
> {
    repo: S,
    inventory_repo: I,
    movement_repo: M,
    event_bus: EventBus,
    logger: Box<dyn Logger>,
}

impl<S: SalesRepository, I: InventoryRepository, M: InventoryMovementRepository>
    ForSaleInteractor<S, I, M>
{
    pub fn new(
        repo: S,
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

    async fn resolve_inventory(&self, product_id: Uuid, store_id: Uuid) -> Result<Inventory> {
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
                .error(&format!("[Sale] Failed to publish event: {}", e));
        }
    }
}

#[async_trait]
impl<
        S: SalesRepository + Sync + Send,
        I: InventoryRepository + Sync + Send,
        M: InventoryMovementRepository + Sync + Send,
    > ForSaleUseCases for ForSaleInteractor<S, I, M>
{
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Sale>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Sale> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Sale not found"))
    }

    async fn register_sale(
        &self,
        input: RegisterSaleOperationInput,
    ) -> Result<SaleOperationOutput> {
        let store_id = Uuid::parse_str(&input.store_id)?;
        let payment_method =
            PaymentMethod::from_str(&input.payment_method).unwrap_or(PaymentMethod::Other);

        if input.items.is_empty() {
            anyhow::bail!("Sale must have at least one item");
        }

        let sale_id = Uuid::new_v4();

        let mut sale_items: Vec<SaleItem> = Vec::with_capacity(input.items.len());

        for item_input in &input.items {
            let product_id = Uuid::parse_str(&item_input.product_id)?;
            let unit_price = item_input.unit_price.parse::<Decimal>().map_err(|e| {
                anyhow::anyhow!("Invalid unit_price '{}': {}", item_input.unit_price, e)
            })?;

            let inventory = self.resolve_inventory(product_id, store_id).await?;

            let sale_item = SaleItem::new(sale_id, inventory.id, item_input.quantity, unit_price)?;

            sale_items.push(sale_item);
        }

        let total = Sale::calculate_total(&sale_items)?;
        let sale = Sale::restore(
            sale_id,
            store_id,
            input.sale_code,
            total,
            payment_method,
            Utc::now(),
            sale_items,
        )?;

        self.repo.create(&sale).await?;

        let mut movements_registered = 0usize;
        for item in &sale.items {
            match self
                .handle_item_movement(item.inventory_id, |inv| {
                    inventory_movement::calculate_sale(inv, item.quantity.value(), None, item.id)
                })
                .await
            {
                Ok(_) => movements_registered += 1,
                Err(e) => {
                    self.logger.error(&format!(
                        "[Sale] Failed to register movement for item {}: {}",
                        item.id, e
                    ));
                }
            }
        }

        self.publish_event(DomainEvent::SaleCompleted(SaleCompleted::from_sale(&sale)))
            .await;

        Ok(SaleOperationOutput::from_sale(&sale, movements_registered))
    }
}
