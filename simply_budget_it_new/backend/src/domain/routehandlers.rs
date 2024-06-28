use core::panic;
use std::{path::Path, sync::Arc};

use axum::{debug_handler, extract::State, response::Response, Json};

use crate::{
    domain::{
        budgets::{delete_all_budgets, delete_budget},
        transactions::{delete_all_transactions, delete_transactions},
    },
    AppState,
};

use super::{budgets::Budget, transactions::Transaction};

#[debug_handler]
pub async fn delete_handler(
    Path(id): Path,
    State(state): State<Arc<AppState>>,
    Json(budget): Option<Json<Budget>>,
    Json(transaction): Option<Vec<Json<Transaction>>>,
) -> Response {
    let route = id.as_str();
    match (budget, transaction) {
        (Some(_), Some(_)) => {
            panic!("shouldn't happen");
        }
        (Some(budget), None) => match route {
            da => delete_all_budgets(state, budget).await,
            db => delete_budget(state, budget).await,
            dta => delete_all_transactions(state, budget).await,
            _ => {
                panic!()
            }
        },
        (None, Some(transaction)) => match route {
            dt => delete_transactions(state, transaction).await,
            _ => {
                panic!()
            }
        },
        (None, None) => {
            panic!("Shouldn't happen")
        }
    }
}
