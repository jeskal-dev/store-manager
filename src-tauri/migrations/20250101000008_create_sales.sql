CREATE TABLE IF NOT EXISTS sales (
    id TEXT NOT NULL PRIMARY KEY,
    store_id TEXT NOT NULL REFERENCES stores(id),
    sale_code TEXT NOT NULL UNIQUE,
    total TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    sale_date TEXT NOT NULL
);
