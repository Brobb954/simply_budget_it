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

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<NewUser>,
) -> Response {
    use self::users::dsl::*;
    let new_user = NewUser {
        username: user.username,
        email: user.email,
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
    let created_user = diesel::insert_into(users).values(&new_user).execute(&mut conn).await;

    match created_user.unwrap() {
        0_usize => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting budget").into_response(),
        _ => (StatusCode::ACCEPTED).into_response(),
    }
}
