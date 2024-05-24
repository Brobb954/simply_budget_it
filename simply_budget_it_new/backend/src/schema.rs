// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
}

diesel::table! {
    budgets (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        user_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;

    transactions (id) {
        id -> Int4,
        description -> Nullable<Text>,
        transaction_type -> TransactionType,
        amount -> Numeric,
        transaction_date -> Nullable<Date>,
        created_at -> Timestamp,
        budget_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(budgets -> users (user_id));
diesel::joinable!(transactions -> budgets (budget_id));

diesel::allow_tables_to_appear_in_same_query!(
    budgets,
    transactions,
    users,
);
