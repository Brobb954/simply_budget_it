use serde::{Deserialize, Serialize};

use tauri::State;

#[tauri::command]
pub async fn add_transaction(
    pool: State<'_, sqlx::PgPool>,
    transaction: Transaction,
) -> Result<(), String> {
    sqlx::query!(
        "INSERT INTO transactions (transaction_type, description, amount) VALUES ($1, $2, $3)",
        transaction.transaction_type,
        transaction.description,
        transaction.amount
    )
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
pub async fn fetch_transactions_and_totals(
    pool: State<'_, sqlx::PgPool>,
) -> Result<TransactionResponse, String> {
    let transactions = sqlx::query_as!(Transaction, "SELECT * FROM transactions")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    let total_income: f32 = transactions
        .iter()
        .filter(|t| t.transaction_type == "income")
        .map(|t| t.amount)
        .sum();

    let total_expense: f32 = transactions
        .iter()
        .filter(|t| t.transaction_type == "expense")
        .map(|t| t.amount)
        .sum();

    let balance = total_income - total_expense;

    let response = TransactionResponse {
        transactions,
        totals: Totals {
            total_income,
            total_expense,
            balance,
        },
    };
    Ok(response)
}

#[tauri::command]
pub async fn delete_transaction(
    pool: State<'_, sqlx::PgPool>,
    transaction: Transaction,
) -> Result<(), String> {
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub transaction_type: String,
    pub description: String,
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transactions: Vec<Transaction>,
    pub totals: Totals,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Totals {
    pub total_income: f32,
    pub total_expense: f32,
    pub balance: f32,
}
