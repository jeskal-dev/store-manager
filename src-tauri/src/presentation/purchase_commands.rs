use tauri::State;
use uuid::Uuid;

use crate::application::dtos::operations::{
    PurchaseOperationOutput, RegisterPurchaseOperationInput,
};
use crate::application::dtos::shared::CriteriaInput;
use crate::application::use_cases::purchase::ForPurchaseUseCases;
use crate::domain::aggregates::purchase::Purchase;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn search_purchases(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<Purchase>, String> {
    state
        .use_cases
        .purchase
        .search(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_purchase_by_id(
    state: State<'_, AppState>,
    id: String,
) -> Result<Purchase, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .purchase
        .find_by_id(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register_purchase_operation(
    state: State<'_, AppState>,
    input: RegisterPurchaseOperationInput,
) -> Result<PurchaseOperationOutput, String> {
    state
        .use_cases
        .purchase
        .register_purchase(input)
        .await
        .map_err(|e| e.to_string())
}
