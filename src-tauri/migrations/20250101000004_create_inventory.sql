CREATE TABLE IF NOT EXISTS inventory (
    id TEXT NOT NULL PRIMARY KEY,
    inventory_code TEXT NOT NULL UNIQUE,
    store_id TEXT NOT NULL REFERENCES stores(id),
    product_id TEXT NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL,
    price_local TEXT NOT NULL,
    min_stock INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    last_updated TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1
);
