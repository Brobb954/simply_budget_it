-- Add migration script here
CREATE TABLE IF NOT EXISTS transactions (
    ID SERIAL PRIMARY KEY NOT NULL,
    Transaction_Type TEXT NOT NULL,
    Description TEXT NOT NULL,
    Amount REAL NOT NULL
)