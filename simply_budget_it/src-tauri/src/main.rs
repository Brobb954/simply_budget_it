// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod data_management;
use data_management::{add_transaction, delete_transaction, fetch_transactions_and_totals};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Configure the database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    tauri::Builder::default()
        .manage(db_pool)
        .invoke_handler(tauri::generate_handler![
            add_transaction,
            delete_transaction,
            fetch_transactions_and_totals,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
