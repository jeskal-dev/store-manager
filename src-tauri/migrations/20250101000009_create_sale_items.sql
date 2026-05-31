CREATE TABLE IF NOT EXISTS sale_items (
    id TEXT NOT NULL PRIMARY KEY,
    sale_id TEXT NOT NULL REFERENCES sales(id),
    inventory_id TEXT NOT NULL REFERENCES inventory(id),
    quantity INTEGER NOT NULL,
    unit_price TEXT NOT NULL,
    subtotal TEXT NOT NULL
);
