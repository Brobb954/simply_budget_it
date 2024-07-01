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
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::{Pg, PgValue};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Deserialize;

#[derive(Queryable, Clone, Identifiable, Associations, Selectable, Debug, PartialEq, Deserialize)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub description: Option<String>,
    pub transaction_type: TransactionType,
    pub amount: f32,
    pub transaction_date: Option<NaiveDateTime>,
    pub budget_id: i32,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub description: Option<&'a String>,
    pub transaction_type: &'a TransactionType,
    pub amount: &'a BigDecimal,
    pub transaction_date: Option<&'a NaiveDate>,
    pub budget_id: &'a i32,
}

#[derive(Debug, PartialEq, Clone, Deserialize, FromSqlRow, AsExpression, Eq)]
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
   Json(delete_trans): Json<Vec<Transaction>>,
) -> Response {
    use self::transactions::dsl::*;
    let delete_transactions = delete_trans;
    let conn = &mut state.pool.get().await;
    for transaction in delete_transactions.into_iter() {
        match diesel::delete(transactions)
            .filter(id.eq(transaction.id))
            .execute(conn.as_mut().unwrap()).await{
           Ok( 0_usize) => return "Failed to delete".into_response(),
            _ => continue,
        };
    }
    (StatusCode::ACCEPTED).into_response()
}

#[debug_handler]
pub async fn delete_all_transactions(
    state: State<Arc<AppState>>,
    budget: Json<Budget>,
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
