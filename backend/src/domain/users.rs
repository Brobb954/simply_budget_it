use crate::{schema::users, AppState};
use diesel_async::RunQueryDsl;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Queryable, Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct UserId {
    pub user_id: String,
}

#[debug_handler]
pub async fn create_user(
state: State<Arc<AppState>>,
    Json(user): Json<NewUser>,
) -> Response {
    use self::users::dsl::*;
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
    let created_user = diesel::insert_into(users).values(&user).execute(&mut conn).await;

    match created_user.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budget").into_response(),
        _ => (StatusCode::ACCEPTED).into_response(),
    }
}

#[debug_handler]
pub async fn update_user(
    state: State<Arc<AppState>>,
    Json(user): Json<User>
) -> Response {
  //  use self::users::dsl::*;
    let mut _conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    format!("this is {:#?}", user).into_response()
}


#[debug_handler]
pub async fn get_user(
    state: State<Arc<AppState>>,
    Json(user_id): Json<UserId>
) -> Response {
    let mut _conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to the database",
            )
                .into_response()
        }
    };
    format!("this is {:#?}", user_id.user_id).into_response()
}
