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
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, usize};
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
    use self::budgets::dsl::*;
    let updated_budget = Budget {
        id: budget.id,
        name: budget.name,
        description: budget.description,
        user_id: budget.user_id,
    };

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
    let updated = diesel::update(budgets)
        .set(&updated_budget.clone())
        .filter(id.eq(budget.id))
        .execute(&mut conn)
        .await;
    match updated.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        _ => (StatusCode::ACCEPTED).into_response(),
    }
}
#[debug_handler]
pub async fn create_budget(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<NewBudget>,
) -> Response {
    use self::budgets::dsl::*;
    let new_budget = NewBudget {
        name: budget.name,
        description: budget.description,
        user_id: budget.user_id,
    };
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
    let result = diesel::insert_into(budgets)
        .values(&new_budget)
        .execute(&mut conn)
        .await;

    match result.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        _ => (StatusCode::ACCEPTED).into_response(),
    };

    (StatusCode::CREATED).into_response()
}

#[debug_handler]
pub async fn delete_budget(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    use self::budgets::dsl::*;
    let mut conn = &mut match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    let budget_deleted = diesel::delete(budgets)
        .filter(id.eq(budget.id))
        .execute(&mut conn)
        .await;
    match budget_deleted.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budget").into_response(),
        _ => (StatusCode::ACCEPTED).into_response(),
    }
}

#[debug_handler]
pub async fn delete_all_budgets(
    State(state): State<Arc<AppState>>,
    Json(budget): Json<Budget>,
) -> Response {
    use self::budgets::dsl::*;
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
    let deleted = diesel::delete(budgets)
        .filter(user_id.eq(budget.user_id))
        .execute(&mut conn)
        .await;
    match deleted.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budgets").into_response(),
        _ => "All budgets deleted".into_response(),
    }
}

#[debug_handler]
pub async fn get_budgets(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Json<Vec<Budget>> {
    info!("Looking for user budgets!");

    use self::budgets::dsl::*;
    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            panic!("Should always recieve connection");
        }
    };

    let budgets_to_get = budgets.filter(user_id.eq(user.id));
    let returned_budgets: Vec<Budget> = budgets_to_get
        .get_results::<Budget>(&mut conn)
        .await
        .unwrap();

    Json(returned_budgets)
}
