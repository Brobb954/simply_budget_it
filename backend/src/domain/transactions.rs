use diesel_async::RunQueryDsl;
use std::io::Write;
use std::sync::Arc;

use crate::domain::budgets::Budget;
use crate::schema::{self, transactions};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Json};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::pg::{Pg, PgValue};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Serialize, Deserialize};

#[derive(
    Queryable,
    Clone,
    Identifiable,
    Associations,
    Selectable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    AsChangeset,
)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub description: Option<String>,
    pub transaction_type: TransactionType,
    pub amount: BigDecimal,
    pub transaction_date: Option<NaiveDate>,
    pub budget_id: i32,
}

#[derive(Insertable, Associations, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub description: Option<String>,
    pub transaction_type: TransactionType,
    pub amount: BigDecimal,
    pub transaction_date: Option<NaiveDate>,
    pub budget_id: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = schema::sql_types::TransactionType)]
pub enum TransactionType {
    Income,
    Expense,
}

impl ToSql<schema::sql_types::TransactionType, Pg> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TransactionType::Income => out.write_all(b"Income")?,
            TransactionType::Expense => out.write_all(b"Expense")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::TransactionType, Pg> for TransactionType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Income" => Ok(TransactionType::Income),
            b"Expense" => Ok(TransactionType::Expense),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[debug_handler]
pub async fn delete_transactions(
    state: State<Arc<AppState>>,
    Json(delete_transactions): Json<Vec<Transaction>>,
) -> Response {
    use self::transactions::dsl::*;
    let mut conn = state.pool.get().await;
    for transaction in delete_transactions.into_iter() {
        match diesel::delete(transactions)
            .filter(id.eq(transaction.id))
            .execute(conn.as_mut().unwrap())
            .await
        {
            Ok(0_usize) => return "Failed to delete".into_response(),
            _ => continue,
        };
    }
    (StatusCode::ACCEPTED).into_response()
}

#[debug_handler]
pub async fn delete_all_transactions(
    state: State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    use self::transactions::dsl::*;
    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };

    let deleted_transactions = diesel::delete(transactions.filter(budget_id.eq(budget.user_id)))
        .execute(&mut conn)
        .await;

    match deleted_transactions.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budget").into_response(),
        _ => "All transactions deleted".into_response(),
    }
}

#[debug_handler]
pub async fn create_transactions(
    state: State<Arc<AppState>>,
    Json(new_transactions): Json<Vec<NewTransaction>>,
) -> Response {
    use self::transactions::dsl::*;
    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    for transaction in new_transactions.into_iter() {
        match diesel::insert_into(transactions)
            .values(&transaction)
            .execute(&mut conn)
            .await
        {
            Ok(0_usize) => return "Failed to delete".into_response(),
            _ => continue,
        }
    }
    "All transactions inserted successfully".into_response()
}

#[debug_handler]
pub async fn update_transaction(
    state: State<Arc<AppState>>,
    Json(transaction): Json<Transaction>,
) -> Response {
    use self::transactions::dsl::*;
    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    let updated_transaction = diesel::update(transactions)
        .set(&transaction)
        .execute(&mut conn)
        .await;
    match updated_transaction.unwrap() {
        0_usize => "Failed to update tx".into_response(),
        _ => "Updated transaction successfully".into_response(),
    }
}


#[debug_handler]
pub async fn get_transactions(
    state: State<Arc<AppState>>,
    Json(budget): Json<Budget>
) {
print!("{:?}", budget);
    let mut _conn =  state.pool.get().await; 
}
