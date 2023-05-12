-- Stores
CREATE TABLE Stores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Promoters
CREATE TABLE Promoters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    bank_id TEXT,
    store_id INTEGER NOT NULL,
    active INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (store_id) REFERENCES Stores(id)
);

-- Models
CREATE TABLE Models (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Purchases
CREATE TABLE Purchases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date INTEGER NOT NULL,
    description TEXT NOT NULL,
    value INTEGER NOT NULL,
    debt INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);

-- Payments
CREATE TABLE Payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date INTEGER NOT NULL,
    value INTEGER NOT NULL,
    net_received INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);

-- Promotions
CREATE TABLE Promotions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date INTEGER NOT NULL,
    paid INTEGER NOT NULL DEFAULT 0,
    model_id INTEGER NOT NULL,
    promoter_id INTEGER NOT NULL,
    FOREIGN KEY (model_id) REFERENCES Models(id),
    FOREIGN KEY (promoter_id) REFERENCES Promoters(id)
);
