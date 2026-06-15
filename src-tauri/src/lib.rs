pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use infrastructure::di;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            di::setup_app(app).expect("failed to initialize application");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // analytics
            presentation::analytics_commands::get_dashboard_data,
            presentation::analytics_commands::get_sales_over_time,
            presentation::analytics_commands::get_top_products,
            presentation::analytics_commands::get_inventory_summary,
            presentation::analytics_commands::get_sales_by_store,
            presentation::analytics_commands::get_sales_by_payment_method,
            // product
            presentation::product_commands::create_product,
            presentation::product_commands::update_product,
            presentation::product_commands::delete_product,
            presentation::product_commands::search_products,
            // supplier
            presentation::supplier_commands::create_supplier,
            presentation::supplier_commands::update_supplier,
            presentation::supplier_commands::delete_supplier,
            presentation::supplier_commands::search_suppliers,
            // store
            presentation::store_commands::create_store,
            presentation::store_commands::update_store,
            presentation::store_commands::delete_store,
            presentation::store_commands::search_stores,
            // inventory
            presentation::inventory_commands::create_inventory,
            presentation::inventory_commands::update_inventory,
            presentation::inventory_commands::delete_inventory,
            presentation::inventory_commands::search_inventories,
            // supply_agreement
            presentation::supply_agreement_commands::create_supply_agreement,
            presentation::supply_agreement_commands::update_supply_agreement,
            presentation::supply_agreement_commands::delete_supply_agreement,
            presentation::supply_agreement_commands::search_supply_agreements,
            // purchase
            presentation::purchase_commands::search_purchases,
            presentation::purchase_commands::find_purchase_by_id,
            presentation::purchase_commands::register_purchase_operation,
            // sale
            presentation::sale_commands::search_sales,
            presentation::sale_commands::find_sale_by_id,
            presentation::sale_commands::register_sale_operation,
            // inventory_movement
            presentation::inventory_movement_commands::register_restock_movement,
            presentation::inventory_movement_commands::register_shrinkage_movement,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
