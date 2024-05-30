use std::{sync::Arc, usize};

use crate::domain::users::User;
use crate::{schema::budgets, AppState};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Selectable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(belongs_to(User))]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = budgets)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
}

#[derive(Insertable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = budgets)]
pub struct NewBudget {
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
}

impl Budget {
    pub fn into_budget(self) -> Budget {
        Budget {
            id: self.id,
            name: self.name,
            description: self.description,
            user_id: self.user_id,
        }
    }
}

#[debug_handler]
pub async fn update_budget(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    let updated_budget = Budget {
        id: budget.id,
        name: budget.name,
        description: budget.description,
        user_id: budget.user_id,
    };

    let conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };

    conn.interact(move |conn| {
        diesel::update(budgets::table)
            .set(&updated_budget.clone())
            .filter(budgets::id.eq(budget.id))
            .execute(conn)
            .expect("Error updating budget");
    })
    .await
    .expect("Error updating budget");

    let budgets = budgets::table.filter(budgets::id.eq(budget.id));
    let updated: Budget = conn
        .interact(move |conn| {
            budgets
                .get_result::<Budget>(conn)
                .expect("Error finding budgets for user")
        })
        .await
        .expect("Did not find any budgets for user");

    (StatusCode::ACCEPTED, Json(updated)).into_response()
}
#[debug_handler]
pub async fn create_budget(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<NewBudget>,
) -> Response {
    let new_budget = NewBudget {
        name: budget.name,
        description: budget.description,
        user_id: budget.user_id,
    };
    let conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(budgets::table)
                .values(&new_budget)
                .get_result::<Budget>(conn)
                .expect("Could not insert budget")
        })
        .await
        .expect("Could not connect to database");

    tracing::debug!("Returning JSON response: {:?}", result);
    (StatusCode::CREATED, Json(result)).into_response()
}

#[debug_handler]
pub async fn delete_budget(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    use self::budgets::dsl::*;
    let conn = &mut match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    let budget_deleted: Result<Vec<Budget>, _> = conn
        .interact(move |conn| diesel::delete(budgets.filter(id.eq(budget.id))).load::<Budget>(conn))
        .await
        .expect("Could not find budget");
    let result = budget_deleted.as_ref().unwrap().clone().len();
    match result {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budget").into_response(),
        _ => (StatusCode::ACCEPTED, Json(budget_deleted.unwrap())).into_response(),
    }
}

#[debug_handler]
pub async fn delete_all_budgets(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    let conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };

    conn.interact(move |conn| {
        diesel::delete(budgets::table.filter(budgets::user_id.eq(budget.user_id)))
            .execute(conn)
            .expect("Error deleting all budgets for user")
    })
    .await
    .expect("Error deleting all budgets for user");

    "All budgets deleted".into_response()
}

#[debug_handler]
pub async fn get_budgets(State(state): State<Arc<AppState>>, Json(user): Json<User>) -> Response {
    info!("Looking for user budgets!");

    let conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };

    let budgets = budgets::table.filter(budgets::user_id.eq(user.id));
    let budget: Vec<Budget> = conn
        .interact(move |conn| {
            budgets
                .get_results::<Budget>(conn)
                .expect("Error finding budgets for user")
        })
        .await
        .expect("Did not find any budgets for user");

    Json(budget).into_response()
}
