-- Your SQL goes here
CREATE TYPE TRANSACTIONTYPE AS ENUM ('debit','credit');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    total_transaction INTEGER NOT NULL,
    type_transaction TRANSACTIONTYPE NOT NULL,
    description VARCHAR(225) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
