use std::sync::Arc;

use axum::{routing::get, Extension};
use axum::Router;

use crate::AppState;

pub fn app_router(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    Router::new().route("/",get(root)).layer(Extension(shared_state))
}


async fn root() -> &'static str {
    "Hello, World!"
}