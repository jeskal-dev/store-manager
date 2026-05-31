CREATE TABLE IF NOT EXISTS supply_agreements (
    id TEXT NOT NULL PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    supplier_id TEXT NOT NULL REFERENCES suppliers(id),
    cost TEXT,
    active INTEGER NOT NULL DEFAULT 1
);
