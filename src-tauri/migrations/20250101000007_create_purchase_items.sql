CREATE TABLE IF NOT EXISTS purchase_items (
    id TEXT NOT NULL PRIMARY KEY,
    purchase_id TEXT NOT NULL REFERENCES purchases(id),
    inventory_id TEXT NOT NULL REFERENCES inventory(id),
    quantity INTEGER NOT NULL,
    unit_cost TEXT NOT NULL,
    subtotal TEXT NOT NULL
);
