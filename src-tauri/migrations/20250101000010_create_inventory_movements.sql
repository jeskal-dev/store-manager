CREATE TABLE IF NOT EXISTS inventory_movements (
    id TEXT NOT NULL PRIMARY KEY,
    inventory_id TEXT NOT NULL REFERENCES inventory(id),
    movement_type TEXT NOT NULL,
    old_quantity INTEGER NOT NULL,
    new_quantity INTEGER NOT NULL,
    date TEXT NOT NULL,
    description TEXT,
    purchase_item_id TEXT REFERENCES purchase_items(id),
    sale_item_id TEXT REFERENCES sale_items(id)
);
