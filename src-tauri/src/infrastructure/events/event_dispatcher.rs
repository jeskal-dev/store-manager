use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::broadcast::error::RecvError;

use crate::infrastructure::events::event_bus::{DomainEvent, EventBus};
use crate::shared::logger::Logger;

/// Background event dispatcher.
///
/// Subscribes to the `EventBus` and routes domain events to:
/// - Tauri frontend via `app.emit()` for real-time UI updates
/// - (future) other handlers like audit logs, notifications, etc.
///
/// Spawns a task on Tauri's async runtime that lives for the app's lifetime.
pub struct EventDispatcher;

impl EventDispatcher {
    /// Spawn a background task that listens for events and forwards them.
    ///
    /// - `handle` — Tauri `AppHandle` for emitting events to the frontend
    /// - `event_bus` — the shared event bus to subscribe to
    /// - `logger` — logger for errors and audit
    pub fn spawn(handle: AppHandle, event_bus: EventBus, logger: Box<dyn Logger>) {
        tauri::async_runtime::spawn(async move {
            let mut rx = event_bus.subscriber();

            loop {
                match rx.recv().await {
                    Ok(event) => {
                        Self::handle_event(&handle, &*logger, event).await;
                    }
                    Err(RecvError::Closed) => {
                        logger.error("[EventDispatcher] EventBus closed, shutting down");
                        break;
                    }
                    Err(RecvError::Lagged(n)) => {
                        logger.warn(&format!(
                            "[EventDispatcher] Lagged behind by {} events, skipping",
                            n
                        ));
                        // Continue receiving — don't crash on lag
                    }
                }
            }
        });
    }

    async fn handle_event(handle: &AppHandle, logger: &dyn Logger, event: DomainEvent) {
        match event {
            DomainEvent::InventoryMovementRecorded(evt) => {
                logger.info(&format!(
                    "[EventDispatcher] Inventory movement recorded: {} (type: {})",
                    evt.movement_id, evt.movement_type
                ));

                if let Err(e) = handle.emit("inventory:movement", &evt) {
                    logger.error(&format!(
                        "[EventDispatcher] Failed to emit inventory:movement: {}",
                        e
                    ));
                }
            }
            DomainEvent::LowStockDetected(evt) => {
                logger.warn(&format!(
                    "[EventDispatcher] Low stock detected: inventory={}, qty={}, min={}",
                    evt.inventory_id, evt.current_quantity, evt.min_stock
                ));

                if let Err(e) = handle.emit("inventory:stock-low", &evt) {
                    logger.error(&format!(
                        "[EventDispatcher] Failed to emit inventory:stock-low: {}",
                        e
                    ));
                }
            }
            DomainEvent::StockOutDetected(evt) => {
                logger.warn(&format!(
                    "[EventDispatcher] Stock out detected: inventory={}",
                    evt.inventory_id
                ));

                if let Err(e) = handle.emit("inventory:stock-out", &evt) {
                    logger.error(&format!(
                        "[EventDispatcher] Failed to emit inventory:stock-out: {}",
                        e
                    ));
                }
            }
            DomainEvent::PurchaseCompleted(evt) => {
                logger.info(&format!(
                    "[EventDispatcher] Purchase completed: {}",
                    evt.purchase_id
                ));

                if let Err(e) = handle.emit("purchase:completed", &evt) {
                    logger.error(&format!(
                        "[EventDispatcher] Failed to emit purchase:completed: {}",
                        e
                    ));
                }
            }
            DomainEvent::SaleCompleted(evt) => {
                logger.info(&format!(
                    "[EventDispatcher] Sale completed: {}",
                    evt.sale_id
                ));

                if let Err(e) = handle.emit("sale:completed", &evt) {
                    logger.error(&format!(
                        "[EventDispatcher] Failed to emit sale:completed: {}",
                        e
                    ));
                }
            }
        }
    }
}
