use tauri::State;
use uuid::Uuid;

use crate::application::dtos::shared::CriteriaInput;
use crate::application::dtos::supply_agreement::{
    CreateSupplyAgreementInput, SupplyAgreementOutput, UpdateSupplyAgreementInput,
};
use crate::application::use_cases::supply_agreement::ForSupplyAgreementUseCases;
use crate::infrastructure::di::AppState;
use crate::shared::criteria::PaginatedResult;

#[tauri::command]
pub async fn create_supply_agreement(
    state: State<'_, AppState>,
    input: CreateSupplyAgreementInput,
) -> Result<SupplyAgreementOutput, String> {
    state
        .use_cases
        .supply_agreement
        .create(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_supply_agreement(
    state: State<'_, AppState>,
    id: String,
    input: UpdateSupplyAgreementInput,
) -> Result<SupplyAgreementOutput, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .supply_agreement
        .update(uuid, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_supply_agreement(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .use_cases
        .supply_agreement
        .delete(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_supply_agreements(
    state: State<'_, AppState>,
    input: CriteriaInput,
) -> Result<PaginatedResult<SupplyAgreementOutput>, String> {
    state
        .use_cases
        .supply_agreement
        .search(input)
        .await
        .map_err(|e| e.to_string())
}
