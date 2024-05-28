use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use deadpool_diesel::postgres::{Runtime, Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config;
use crate::errors::internal_error;

// Import modules

mod schema;
mod config;
mod errors;
mod domain;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct AppState {
   pub pool: Pool,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    init_tracing();

    // Load config settings
    let config = config().await;

    // Set up DB pool 
    let manager = Manager::new(config.db_url().to_string(), Runtime::Tokio1);
    let pool = Pool::builder(manager).max_size(100).build().unwrap();

    // Run migrations
    run_migrations(&pool).await;
    // Create shared state
    let shared_state = Arc::new(AppState { pool });
    // Set app  router
    let app = Router::new().route("/", get(root)).with_state(shared_state);

    // Get server address and create server
    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    let socket_addr: SocketAddr = address.parse().unwrap();

    // Log server listening address
    tracing::info!("Listening on http://{}", socket_addr);
    println!{"Listening on http://{}", socket_addr};

    // Start Axum server
    axum::Server::bind(&socket_addr).serve(app.into_make_service()).await.map_err(internal_error).unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}
// Create function to initialize tracing for logging
fn init_tracing() {
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "Users=debug".into()),).with(tracing_subscriber::fmt::layer()).init();
}

// Create function to run migrations
async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_|())).await.unwrap().unwrap();
}