use tauri::State;
use uuid::Uuid;

use crate::application::dtos::inventory::{CreateInventoryInput, InventoryOutput, UpdateInventoryInput};
use crate::application::dtos::shared::CriteriaInput;
use crate::application::use_cases::inventory::ForInventoryUseCases;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn create_inventory(
    state: State<'_, AppState>,
    input: CreateInventoryInput,
) -> Result<InventoryOutput, String> {
    state
        .use_cases
        .inventory
        .create(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_inventory(
    state: State<'_, AppState>,
    id: String,
    input: UpdateInventoryInput,
) -> Result<InventoryOutput, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .inventory
        .update(uuid, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_inventory(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .inventory
        .delete(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_inventories(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<InventoryOutput>, String> {
    state
        .use_cases
        .inventory
        .search(input)
        .await
        .map_err(|e| e.to_string())
}
