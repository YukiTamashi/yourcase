PRAGMA case_sensitive_like = true;

-- Rename Stores table
ALTER TABLE "Stores" RENAME TO store;

-- Rename Promoters table
ALTER TABLE "Promoters" RENAME TO promoter;

-- Rename Models table
ALTER TABLE "Models" RENAME TO model;

-- Rename Purchases table
ALTER TABLE "Purchases" RENAME TO purchase;

-- Rename Payments table
ALTER TABLE "Payments" RENAME TO payment;

-- Rename Promotions table
ALTER TABLE "Promotions" RENAME TO promotion;