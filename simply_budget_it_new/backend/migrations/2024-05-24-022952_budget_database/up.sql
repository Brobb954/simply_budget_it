-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password_hash VARCHAR NOT NULL
);

CREATE TABLE budgets (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TYPE transaction_type AS ENUM ('expense', 'income');

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    description TEXT,
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    transaction_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    budget_id INT NOT NULL REFERENCES budgets(id) ON DELETE CASCADE
);