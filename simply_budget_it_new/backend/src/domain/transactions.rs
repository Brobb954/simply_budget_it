use std::io::Write;

use crate::domain::budgets::Budget;
use crate::schema::{self, transactions};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::{Pg, PgValue};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq)]
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

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
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
