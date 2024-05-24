use chrono::{NaiveDate, NaiveDateTime};
use diesel::{ deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, prelude::*, serialize::{self, IsNull, Output, ToSql}, sql_types::{Numeric, SqlType}};
use crate::schema::{transactions, users, budgets};


#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub description: Option<String>,
    pub transaction_type: TransactionType,
    pub amount: f32,
    pub transaction_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub budget_id: i32,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(Budget))]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub description: &'a String,
    pub transaction_type: &'a TransactionType,
    pub amount: &'a i32,
    pub transaction_date: Option<&'a NaiveDate>,
    pub budget_id: &'a i32,
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "transaction_type"))]
pub struct TransactionsType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = TransactionsType)]
enum TransactionType {
    Income,
    Expense,
}

impl ToSql<TransactionsType, Pg> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TransactionType::Income => out.write_all(b"Income")?,
            TransactionType::Expense => out.write_all(b"Expense")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<TransactionsType, Pg> for TransactionType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Income" => Ok(TransactionType::Income),
            b"Expense" => Ok(TransactionType::Expense),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = budgets)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = budgets)]
pub struct NewBudget<'a> {
    pub name: &'a String,
    pub description: Option<&'a String>,
    pub created_at: Option<&'a NaiveDateTime>,
    pub user_id: &'a i32,
}


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

