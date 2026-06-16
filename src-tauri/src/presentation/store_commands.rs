use tauri::State;
use uuid::Uuid;

use crate::application::dtos::shared::CriteriaInput;
use crate::application::dtos::store::{CreateStoreInput, UpdateStoreInput};
use crate::application::use_cases::store::ForStoreUseCases;
use crate::domain::entities::store::Store;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn create_store(
    state: State<'_, AppState>,
    input: CreateStoreInput,
) -> Result<Store, String> {
    state
        .use_cases
        .store
        .create(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_store(
    state: State<'_, AppState>,
    id: String,
    input: UpdateStoreInput,
) -> Result<Store, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .store
        .update(uuid, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_store(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .store
        .delete(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_stores(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<Store>, String> {
    state
        .use_cases
        .store
        .search(input)
        .await
        .map_err(|e| e.to_string())
}
