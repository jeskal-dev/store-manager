use tauri::State;
use uuid::Uuid;

use crate::application::dtos::operations::{RegisterSaleOperationInput, SaleOperationOutput};
use crate::application::dtos::shared::CriteriaInput;
use crate::application::use_cases::sale::ForSaleUseCases;
use crate::domain::aggregates::sale::Sale;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn search_sales(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<Sale>, String> {
    state
        .use_cases
        .sale
        .search(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_sale_by_id(state: State<'_, AppState>, id: String) -> Result<Sale, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .sale
        .find_by_id(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register_sale_operation(
    state: State<'_, AppState>,
    input: RegisterSaleOperationInput,
) -> Result<SaleOperationOutput, String> {
    state
        .use_cases
        .sale
        .register_sale(input)
        .await
        .map_err(|e| e.to_string())
}
