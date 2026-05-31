CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    supplier_code TEXT NOT NULL UNIQUE,
    contact_name TEXT,
    phone TEXT,
    address TEXT,
    active INTEGER NOT NULL DEFAULT 1
);
