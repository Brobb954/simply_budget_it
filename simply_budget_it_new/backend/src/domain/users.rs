use crate::{schema::users, AppState};
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
    let new_user = NewUser {
        username: user.username,
        email: user.email,
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
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result::<User>(conn)
                .expect("Error creating user");
        })
        .await;

    match result {
        Ok(_) => {
            tracing::debug!("Returning JSON response: {:?}", &result.unwrap());
            (StatusCode::CREATED, "User created").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response(),
    }
}
