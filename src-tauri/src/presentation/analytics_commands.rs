//! Tauri commands for dashboard analytics / chart data.
//!
//! Each command delegates to the analytics interactor and maps errors to
//! user-facing strings for the frontend.

use tauri::State;
use uuid::Uuid;

use crate::application::dtos::analytics::{
    DashboardData, DashboardInput, DateRangeInput, DailySales, InventorySummary,
    PaymentMethodSales, StoreSales, TopProduct, TopProductsInput,
};
use crate::application::use_cases::analytics::ForAnalyticsUseCases;
use crate::infrastructure::di::AppState;

/// Return all dashboard charts in a single call.
///
/// This is the primary command for the main dashboard page.
#[tauri::command]
pub async fn get_dashboard_data(
    state: State<'_, AppState>,
    input: DashboardInput,
) -> Result<DashboardData, String> {
    state
        .use_cases
        .analytics
        .get_dashboard_data(input)
        .await
        .map_err(|e| e.to_string())
}

/// Daily sales revenue and count over a date range.
#[tauri::command]
pub async fn get_sales_over_time(
    state: State<'_, AppState>,
    input: DateRangeInput,
) -> Result<Vec<DailySales>, String> {
    state
        .use_cases
        .analytics
        .get_sales_over_time(input)
        .await
        .map_err(|e| e.to_string())
}

/// Top N products by revenue in a date range.
#[tauri::command]
pub async fn get_top_products(
    state: State<'_, AppState>,
    input: TopProductsInput,
) -> Result<Vec<TopProduct>, String> {
    state
        .use_cases
        .analytics
        .get_top_products(input)
        .await
        .map_err(|e| e.to_string())
}

/// Inventory health summary (total, low stock, out of stock, total value).
#[tauri::command]
pub async fn get_inventory_summary(
    state: State<'_, AppState>,
    store_id: Option<String>,
) -> Result<InventorySummary, String> {
    let sid = match store_id {
        Some(s) if !s.is_empty() => Some(Uuid::parse_str(&s).map_err(|e| e.to_string())?),
        _ => None,
    };
    state
        .use_cases
        .analytics
        .get_inventory_summary(sid)
        .await
        .map_err(|e| e.to_string())
}

/// Sales data broken down by store.
#[tauri::command]
pub async fn get_sales_by_store(
    state: State<'_, AppState>,
    input: DateRangeInput,
) -> Result<Vec<StoreSales>, String> {
    state
        .use_cases
        .analytics
        .get_sales_by_store(input)
        .await
        .map_err(|e| e.to_string())
}

/// Sales data broken down by payment method.
#[tauri::command]
pub async fn get_sales_by_payment_method(
    state: State<'_, AppState>,
    input: DateRangeInput,
) -> Result<Vec<PaymentMethodSales>, String> {
    state
        .use_cases
        .analytics
        .get_sales_by_payment(input)
        .await
        .map_err(|e| e.to_string())
}
