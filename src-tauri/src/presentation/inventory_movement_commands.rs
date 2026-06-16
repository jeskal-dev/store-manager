use tauri::State;

use crate::application::dtos::inventory_movement::{
    InventoryMovementOutput, RegisterRestockInput, RegisterShrinkageInput,
};
use crate::application::use_cases::inventory_movement::ForInventoryMovementUseCases;
use crate::infrastructure::di::AppState;

#[tauri::command]
pub async fn register_restock_movement(
    state: State<'_, AppState>,
    input: RegisterRestockInput,
) -> Result<InventoryMovementOutput, String> {
    state
        .use_cases
        .inventory_movement
        .register_restock(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register_shrinkage_movement(
    state: State<'_, AppState>,
    input: RegisterShrinkageInput,
) -> Result<InventoryMovementOutput, String> {
    state
        .use_cases
        .inventory_movement
        .register_shrinkage(input)
        .await
        .map_err(|e| e.to_string())
}
