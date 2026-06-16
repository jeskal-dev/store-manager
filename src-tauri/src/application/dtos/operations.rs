use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::aggregates::purchase::Purchase;
use crate::domain::aggregates::sale::Sale;

/// Input for a single purchase item.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseItemInput {
    /// Product ID to find its inventory record in the store.
    #[validate(length(min = 1, message = "Product ID is required"))]
    pub product_id: String,

    /// Positive quantity to add to stock.
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,

    /// Unit cost (must be non-negative).
    #[validate(length(min = 1, message = "Unit cost is required"))]
    pub unit_cost: String,
}

/// Input to register a full purchase operation.
///
/// Creates the purchase record AND registers inventory movements for each item.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPurchaseOperationInput {
    /// Store where the purchase is made.
    #[validate(length(min = 1, message = "Store ID is required"))]
    pub store_id: String,

    /// Supplier ID (optional for cash purchases).
    pub supplier_id: Option<String>,

    /// Unique purchase code.
    #[validate(length(min = 1, max = 50, message = "Purchase code must be 1-50 chars"))]
    pub purchase_code: String,

    /// Optional description.
    #[validate(length(max = 500, message = "Description must be at most 500 chars"))]
    pub description: Option<String>,

    /// At least one item required.
    #[validate(length(min = 1, message = "Purchase must have at least one item"))]
    pub items: Vec<PurchaseItemInput>,
}

/// Output after successfully registering a purchase.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseOperationOutput {
    pub purchase_id: String,
    pub purchase_code: String,
    pub store_id: String,
    pub supplier_id: Option<String>,
    pub total_cost: String,
    pub purchase_date: String,
    pub description: Option<String>,
    pub items_count: usize,
    pub movements_registered: usize,
}

impl PurchaseOperationOutput {
    pub fn from_purchase(purchase: &Purchase, movements_registered: usize) -> Self {
        Self {
            purchase_id: purchase.id.to_string(),
            purchase_code: purchase.purchase_code.value().to_string(),
            store_id: purchase.store_id.to_string(),
            supplier_id: purchase.supplier_id.map(|id| id.to_string()),
            total_cost: purchase.total_cost.formatted(),
            purchase_date: purchase.purchase_date.to_rfc3339(),
            description: purchase.description.as_ref().map(|d| d.value().to_string()),
            items_count: purchase.items.len(),
            movements_registered,
        }
    }
}

// ---------------------------------------------------------------------------
// Sale operation DTOs
// ---------------------------------------------------------------------------

/// Input for a single sale item.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleItemInput {
    /// Product ID to find its inventory record in the store.
    #[validate(length(min = 1, message = "Product ID is required"))]
    pub product_id: String,

    /// Positive quantity to remove from stock.
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,

    /// Unit selling price (must be non-negative).
    #[validate(length(min = 1, message = "Unit price is required"))]
    pub unit_price: String,
}

/// Input to register a full sale operation.
///
/// Creates the sale record AND registers inventory movements for each item.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSaleOperationInput {
    /// Store where the sale is made.
    #[validate(length(min = 1, message = "Store ID is required"))]
    pub store_id: String,

    /// Unique sale code.
    #[validate(length(min = 1, max = 50, message = "Sale code must be 1-50 chars"))]
    pub sale_code: String,

    /// Payment method for this sale (cash, credit_card, bank_transfer, other).
    #[validate(length(min = 1, message = "Payment method is required"))]
    pub payment_method: String,

    /// At least one item required.
    #[validate(length(min = 1, message = "Sale must have at least one item"))]
    pub items: Vec<SaleItemInput>,
}

/// Output after successfully registering a sale.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaleOperationOutput {
    pub sale_id: String,
    pub sale_code: String,
    pub store_id: String,
    pub total: String,
    pub payment_method: String,
    pub sale_date: String,
    pub items_count: usize,
    pub movements_registered: usize,
}

impl SaleOperationOutput {
    pub fn from_sale(sale: &Sale, movements_registered: usize) -> Self {
        Self {
            sale_id: sale.id.to_string(),
            sale_code: sale.sale_code.value().to_string(),
            store_id: sale.store_id.to_string(),
            total: sale.total.formatted(),
            payment_method: sale.payment_method.to_string(),
            sale_date: sale.sale_date.to_rfc3339(),
            items_count: sale.items.len(),
            movements_registered,
        }
    }
}
