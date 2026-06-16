use tauri::State;
use uuid::Uuid;

use crate::application::dtos::product::{CreateProductInput, UpdateProductInput};
use crate::application::dtos::shared::CriteriaInput;
use crate::application::use_cases::product::ForProductUseCases;
use crate::domain::entities::product::Product;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn create_product(
    state: State<'_, AppState>,
    input: CreateProductInput,
) -> Result<Product, String> {
    state
        .use_cases
        .product
        .create(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_product(
    state: State<'_, AppState>,
    id: String,
    input: UpdateProductInput,
) -> Result<Product, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .product
        .update(uuid, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_product(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .product
        .delete(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_products(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<Product>, String> {
    state
        .use_cases
        .product
        .search(input)
        .await
        .map_err(|e| e.to_string())
}
