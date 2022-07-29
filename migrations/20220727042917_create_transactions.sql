CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS transactions (
    transaction_id uuid default uuid_generate_v4() PRIMARY KEY,
    user_id uuid NOT NULL,
    account_id uuid NOT NULL,
    created timestamp,
    amount real NOT NULL
);
