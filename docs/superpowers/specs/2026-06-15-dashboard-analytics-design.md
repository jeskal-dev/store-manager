# Dashboard Analytics — Design Spec

## Overview

Add a read-only analytics module to the store-manager backend that provides
aggregated data for chart components on the initial dashboard. The frontend
calls a **single** `get_dashboard_data` command (or individual metric commands)
to receive pre-aggregated data — no client-side computation of totals or counts.

## Charts / Metrics (v1)

| # | Chart | Data | Aggregation | SQL Source |
|---|-------|------|-------------|------------|
| 1 | **Revenue over time** | daily total + count, last 30 days | `DATE(sale_date)` | `sales` |
| 2 | **Top 5 products** | revenue + units sold, last 30 days | `SUM(subtotal)`, `SUM(quantity)` | `sale_items → inventory → products` |
| 3 | **Inventory alerts** | low-stock count, out-of-stock count, total value | `COUNT(*) + CASE` | `inventory` |
| 4 | **Sales by store** | revenue + count per store, last 30 days | `GROUP BY store_id` | `sales → stores` |
| 5 | **Sales by payment method** | revenue + count per method, last 30 days | `GROUP BY payment_method` | `sales` |

All metrics support an **optional `store_id` filter** (defaults to all stores).

---

## Layer-by-Layer Changes

### 1. Domain / Repository

**`SaleRepository` (trait)** — add 4 query methods:
```rust
async fn get_daily_sales(store_id: Option<Uuid>, start: DateTime, end: DateTime) -> Result<Vec<DailySales>>;
async fn get_top_products(store_id: Option<Uuid>, start: DateTime, end: DateTime, limit: u32) -> Result<Vec<TopProduct>>;
async fn get_sales_by_store(start: DateTime, end: DateTime) -> Result<Vec<StoreSales>>;
async fn get_sales_by_payment_method(store_id: Option<Uuid>, start: DateTime, end: DateTime) -> Result<Vec<PaymentMethodSales>>;
```

**`InventoryRepository` (trait)** — add 1 query method:
```rust
async fn get_inventory_summary(store_id: Option<Uuid>) -> Result<InventorySummary>;
```

**Sqlite implementations** use plain `sqlx::query_as` with `FromRow` structs.
No new tables, no migrations.

### 2. Application Layer

**New DTOs module** `application/dtos/analytics.rs` — value types only:
```rust
pub struct DailySales { pub date: NaiveDate, pub total: Decimal, pub count: i64 }
pub struct TopProduct { pub product_id: Uuid, pub product_code: String, pub name: String, pub total_revenue: Decimal, pub units_sold: i64 }
pub struct InventorySummary { pub total_products: i64, pub low_stock: i64, pub out_of_stock: i64, pub total_value: Decimal }
pub struct StoreSales { pub store_id: Uuid, pub store_code: String, pub name: String, pub total_revenue: Decimal, pub sales_count: i64 }
pub struct PaymentMethodSales { pub method: String, pub total_revenue: Decimal, pub count: i64 }
pub struct DashboardData { ... }
pub struct DashboardInput { pub store_id: Option<Uuid>, pub days: u32 }
pub struct DateRangeInput { pub store_id: Option<Uuid>, pub start: DateTime, pub end: DateTime }
pub struct TopProductsInput { pub store_id: Option<Uuid>, pub start: DateTime, pub end: DateTime, pub limit: u32 }
```

**New use case** `application/use_cases/analytics.rs`:
```rust
#[async_trait]
pub trait ForAnalyticsUseCases {
    async fn get_dashboard_data(&self, input: DashboardInput) -> Result<DashboardData>;
    async fn get_sales_over_time(&self, input: DateRangeInput) -> Result<Vec<DailySales>>;
    async fn get_top_products(&self, input: TopProductsInput) -> Result<Vec<TopProduct>>;
    async fn get_inventory_summary(&self, store_id: Option<Uuid>) -> Result<InventorySummary>;
    async fn get_sales_by_store(&self, input: DateRangeInput) -> Result<Vec<StoreSales>>;
    async fn get_sales_by_payment(&self, input: DateRangeInput) -> Result<Vec<PaymentMethodSales>>;
}
```

### 3. Presentation / Tauri Commands

**New file** `presentation/analytics_commands.rs` — 6 commands:
- `get_dashboard_data`
- `get_sales_over_time`
- `get_top_products`
- `get_inventory_summary`
- `get_sales_by_store`
- `get_sales_by_payment_method`

All accept `State<AppState>` and return `Result<T, String>`.

### 4. Dependency Injection

`infrastructure/di.rs`:
- Add `analytics: ForAnalyticsInteractor<...>` to `UseCases`
- Instantiate with existing repos

### 5. Command Registration

`lib.rs` — register 6 new `#[tauri::command]` handlers.

---

## Frontend Integration

Types auto-generated via `cargo tauri-typegen generate`.
Frontend calls `invoke('get_dashboard_data', { input: { store_id: null, days: 30 } })`.

---

## Not in v1

- Profit margin (requires purchase cost join)
- Export to CSV/PDF
- Custom date range (future: `days: u32` only for now)
- Real-time push via WebSocket / events
