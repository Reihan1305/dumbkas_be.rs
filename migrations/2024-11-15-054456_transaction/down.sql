-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "transactions" CASCADE;
DROP TYPE IF EXISTS "transaction_type" CASCADE;