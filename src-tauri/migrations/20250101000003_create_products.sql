CREATE TABLE IF NOT EXISTS products (
    id TEXT NOT NULL PRIMARY KEY,
    product_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    initial_price TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1
);
