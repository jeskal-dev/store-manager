use tauri::State;
use uuid::Uuid;

use crate::application::dtos::shared::CriteriaInput;
use crate::application::dtos::supplier::{CreateSupplierInput, UpdateSupplierInput};
use crate::application::use_cases::supplier::ForSupplierUseCases;
use crate::domain::entities::supplier::Supplier;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn create_supplier(
    state: State<'_, AppState>,
    input: CreateSupplierInput,
) -> Result<Supplier, String> {
    state
        .use_cases
        .supplier
        .create(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_supplier(
    state: State<'_, AppState>,
    id: String,
    input: UpdateSupplierInput,
) -> Result<Supplier, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .supplier
        .update(uuid, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_supplier(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .supplier
        .delete(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_suppliers(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<Supplier>, String> {
    state
        .use_cases
        .supplier
        .search(input)
        .await
        .map_err(|e| e.to_string())
}
