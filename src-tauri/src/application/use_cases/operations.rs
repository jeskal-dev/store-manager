use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    aggregates::purchase::Purchase,
    aggregates::sale::Sales,
    entities::purchase_item::PurchaseItem,
    entities::sale_item::SaleItem,
    events::inventory::{InventoryMovementRecorded, LowStockDetected, StockOutDetected},
    repositories::{
        inventory::InventoryRepository, inventory_movement::InventoryMovementRepository,
        purchase::PurchaseRepository, sale::SalesRepository,
    },
    services::inventory_movement::{self, MovementResult},
    value_objects::sale::PaymentMethod,
};

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::logger::Logger;

use super::super::dtos::operations::{
    PurchaseOperationOutput, RegisterPurchaseOperationInput, RegisterSaleOperationInput,
    SaleOperationOutput,
};

#[async_trait]
pub trait ForOperationsUseCases {
    async fn register_purchase(
        &self,
        input: RegisterPurchaseOperationInput,
    ) -> Result<PurchaseOperationOutput>;

    async fn register_sale(
        &self,
        input: RegisterSaleOperationInput,
    ) -> Result<SaleOperationOutput>;
}

pub struct ForOperationsInteractor<
    I: InventoryRepository,
    M: InventoryMovementRepository,
    P: PurchaseRepository,
    S: SalesRepository,
> {
    inventory_repo: I,
    movement_repo: M,
    purchase_repo: P,
    sale_repo: S,
    event_bus: EventBus,
    logger: Box<dyn Logger>,
}

impl<I: InventoryRepository, M: InventoryMovementRepository, P: PurchaseRepository, S: SalesRepository>
    ForOperationsInteractor<I, M, P, S>
{
    pub fn new(
        inventory_repo: I,
        movement_repo: M,
        purchase_repo: P,
        sale_repo: S,
        event_bus: EventBus,
        logger: Box<dyn Logger>,
    ) -> Self {
        Self {
            inventory_repo,
            movement_repo,
            purchase_repo,
            sale_repo,
            event_bus,
            logger,
        }
    }

    /// Resolve a product_id to an inventory record in the given store.
    async fn resolve_inventory(
        &self,
        product_id: Uuid,
        store_id: Uuid,
    ) -> Result<crate::domain::entities::inventory::Inventory> {
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

    /// Register inventory movement + publish events for a single purchase item.
    async fn register_item_movement(
        &self,
        purchase_item: &PurchaseItem,
    ) -> Result<MovementResult> {
        let inventory = self
            .inventory_repo
            .find_by_id(purchase_item.inventory_id)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("Inventory not found: {}", purchase_item.inventory_id)
            })?;

        if !inventory.is_active() {
            anyhow::bail!("Inventory is inactive: {}", inventory.id);
        }

        let result = inventory_movement::calculate_purchase(
            &inventory,
            purchase_item.quantity.value(),
            None,
            purchase_item.id,
        )?;

        self.movement_repo
            .save_with_inventory_update(&result.movement, &result.updated_inventory)
            .await?;

        // Publish movement event
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
                "[Operations] Failed to publish movement event: {}",
                e
            ));
        }

        // Publish low-stock event
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
                    "[Operations] Failed to publish low-stock event: {}",
                    e
                ));
            }
        }

        // Publish stock-out event
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
                    "[Operations] Failed to publish stock-out event: {}",
                    e
                ));
            }
        }

        Ok(result)
    }

    /// Register inventory movement + publish events for a single sale item.
    async fn register_sale_movement(
        &self,
        sale_item: &SaleItem,
    ) -> Result<MovementResult> {
        let inventory = self
            .inventory_repo
            .find_by_id(sale_item.inventory_id)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("Inventory not found: {}", sale_item.inventory_id)
            })?;

        if !inventory.is_active() {
            anyhow::bail!("Inventory is inactive: {}", inventory.id);
        }

        let result = inventory_movement::calculate_sale(
            &inventory,
            sale_item.quantity.value(),
            None,
            sale_item.id,
        )?;

        self.movement_repo
            .save_with_inventory_update(&result.movement, &result.updated_inventory)
            .await?;

        // Publish movement event
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
                "[Operations] Failed to publish movement event: {}",
                e
            ));
        }

        // Publish low-stock event
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
                    "[Operations] Failed to publish low-stock event: {}",
                    e
                ));
            }
        }

        // Publish stock-out event
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
                    "[Operations] Failed to publish stock-out event: {}",
                    e
                ));
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl<
        I: InventoryRepository + Sync + Send,
        M: InventoryMovementRepository + Sync + Send,
        P: PurchaseRepository + Sync + Send,
        S: SalesRepository + Sync + Send,
    > ForOperationsUseCases for ForOperationsInteractor<I, M, P, S>
{
    async fn register_purchase(
        &self,
        input: RegisterPurchaseOperationInput,
    ) -> Result<PurchaseOperationOutput> {
        // 1. Parse inputs
        let store_id = Uuid::parse_str(&input.store_id)?;
        let supplier_id = match &input.supplier_id {
            Some(id) => Some(Uuid::parse_str(id)?),
            None => None,
        };

        // 2. Validate at least one item
        if input.items.is_empty() {
            anyhow::bail!("Purchase must have at least one item");
        }

        // 3. Generate purchase_id upfront so items reference it correctly
        let purchase_id = Uuid::new_v4();

        // 4. Resolve each item: product_id → inventory, then create PurchaseItem
        let mut purchase_items: Vec<PurchaseItem> = Vec::with_capacity(input.items.len());

        for item_input in &input.items {
            let product_id = Uuid::parse_str(&item_input.product_id)?;
            let unit_cost = item_input.unit_cost.parse::<Decimal>().map_err(|e| {
                anyhow::anyhow!("Invalid unit_cost '{}': {}", item_input.unit_cost, e)
            })?;

            let inventory = self.resolve_inventory(product_id, store_id).await?;

            let purchase_item = PurchaseItem::new(
                purchase_id,
                inventory.id,
                item_input.quantity,
                unit_cost,
            )?;

            purchase_items.push(purchase_item);
        }

        // 5. Build the Purchase entity
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

        // 6. Save purchase to database (header + items in transaction)
        self.purchase_repo.create(&purchase).await?;

        // 7. Register inventory movement for each item
        let mut movements_registered = 0usize;
        for item in &purchase.items {
            match self.register_item_movement(item).await {
                Ok(_) => movements_registered += 1,
                Err(e) => {
                    self.logger.error(&format!(
                        "[Operations] Failed to register movement for item {}: {}",
                        item.id, e
                    ));
                    // Continue with other items — don't abort the whole purchase
                }
            }
        }

        Ok(PurchaseOperationOutput::from_purchase(
            &purchase,
            movements_registered,
        ))
    }

    async fn register_sale(
        &self,
        input: RegisterSaleOperationInput,
    ) -> Result<SaleOperationOutput> {
        // 1. Parse inputs
        let store_id = Uuid::parse_str(&input.store_id)?;
        let payment_method = match input.payment_method.to_lowercase().as_str() {
            "cash" => PaymentMethod::Cash,
            "credit_card" => PaymentMethod::CreditCard,
            "bank_transfer" => PaymentMethod::BankTransfer,
            _ => PaymentMethod::Other,
        };

        // 2. Validate at least one item
        if input.items.is_empty() {
            anyhow::bail!("Sale must have at least one item");
        }

        // 3. Generate sale_id upfront so items reference it correctly
        let sale_id = Uuid::new_v4();

        // 4. Resolve each item: product_id → inventory, then create SaleItem
        let mut sale_items: Vec<SaleItem> = Vec::with_capacity(input.items.len());

        for item_input in &input.items {
            let product_id = Uuid::parse_str(&item_input.product_id)?;
            let unit_price = item_input.unit_price.parse::<Decimal>().map_err(|e| {
                anyhow::anyhow!("Invalid unit_price '{}': {}", item_input.unit_price, e)
            })?;

            let inventory = self.resolve_inventory(product_id, store_id).await?;

            let sale_item = SaleItem::new(
                sale_id,
                inventory.id,
                item_input.quantity,
                unit_price,
            )?;

            sale_items.push(sale_item);
        }

        // 5. Build the Sales entity
        let total = Sales::calculate_total(&sale_items)?;
        let sale = Sales::restore(
            sale_id,
            store_id,
            input.sale_code,
            total,
            payment_method,
            Utc::now(),
            sale_items,
        )?;

        // 6. Save sale to database (header + items in transaction)
        self.sale_repo.create(&sale).await?;

        // 7. Register inventory movement for each item
        let mut movements_registered = 0usize;
        for item in &sale.items {
            match self.register_sale_movement(item).await {
                Ok(_) => movements_registered += 1,
                Err(e) => {
                    self.logger.error(&format!(
                        "[Operations] Failed to register movement for item {}: {}",
                        item.id, e
                    ));
                    // Continue with other items — don't abort the whole sale
                }
            }
        }

        Ok(SaleOperationOutput::from_sale(
            &sale,
            movements_registered,
        ))
    }
}
