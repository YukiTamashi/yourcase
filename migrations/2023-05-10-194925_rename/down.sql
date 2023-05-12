PRAGMA case_sensitive_like = true;

-- Rename store table back to Stores
ALTER TABLE store RENAME TO "Stores";

-- Rename promoter table back to Promoters
ALTER TABLE promoter RENAME TO "Promoters";

-- Rename model table back to Models
ALTER TABLE model RENAME TO "Models";

-- Rename purchase table back to Purchases
ALTER TABLE purchase RENAME TO "Purchases";

-- Rename payment table back to Payments
ALTER TABLE payment RENAME TO "Payments";

-- Rename promotion table back to Promotions
ALTER TABLE promotion RENAME TO "Promotions";
