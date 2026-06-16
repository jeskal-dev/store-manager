use anyhow::Result;
use tokio::sync::broadcast;

use crate::domain::events::inventory::{
    InventoryMovementRecorded, LowStockDetected, StockOutDetected,
};
use crate::domain::events::purchase::PurchaseCompleted;
use crate::domain::events::sale::SaleCompleted;

#[derive(Debug, Clone)]
pub enum DomainEvent {
    InventoryMovementRecorded(InventoryMovementRecorded),
    LowStockDetected(LowStockDetected),
    StockOutDetected(StockOutDetected),
    PurchaseCompleted(PurchaseCompleted),
    SaleCompleted(SaleCompleted),
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<DomainEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publisher(&self) -> broadcast::Sender<DomainEvent> {
        self.sender.clone()
    }

    pub fn subscriber(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }

    pub async fn publish(&self, event: DomainEvent) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}

// TODO: Add background event dispatcher task:
// 1. Subscribe to EventBus
// 2. Route events to handlers (update stock, check low stock, emit to frontend)
// 3. Log errors but do not crash
