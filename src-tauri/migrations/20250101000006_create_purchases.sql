CREATE TABLE IF NOT EXISTS purchases (
    id TEXT NOT NULL PRIMARY KEY,
    supplier_id TEXT REFERENCES suppliers(id),
    store_id TEXT NOT NULL REFERENCES stores(id),
    total_cost TEXT NOT NULL,
    purchase_date TEXT NOT NULL,
    description TEXT,
    purchase_code TEXT NOT NULL UNIQUE
);
