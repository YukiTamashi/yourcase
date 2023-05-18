CREATE TABLE stores (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

-- Promoters
CREATE TABLE promoters (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    bank_id TEXT,
    store_id INTEGER NOT NULL,
    active INTEGER NOT NULL DEFAULT 1,
    debt INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (store_id) REFERENCES Stores(id),
    UNIQUE(store_id, name)
);

-- Models
CREATE TABLE models (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

-- Purchases
CREATE TABLE purchases (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    date INTEGER NOT NULL,
    description TEXT NOT NULL,
    value INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);

-- Payments
CREATE TABLE payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    date INTEGER NOT NULL,
    value INTEGER NOT NULL,
    net_received INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);

-- Promotions
CREATE TABLE promotions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    date INTEGER NOT NULL,
    value INTEGER NOT NULL,
    model_id INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (model_id) REFERENCES Models(id),
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);
