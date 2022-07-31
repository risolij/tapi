CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE CATEGORY AS ENUM ('Food', 'Business', 'Gas');

CREATE TABLE IF NOT EXISTS transactions (
    transaction_id uuid default uuid_generate_v4() PRIMARY KEY,
    user_id uuid NOT NULL,
    account_id uuid NOT NULL,
    created timestamp,
    category CATEGORY NOT NULL,
    amount real NOT NULL
);
