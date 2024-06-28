use std::{net::SocketAddr, sync::Arc};

use crate::{
    config::config,
    domain::{
        budgets::{create_budget, update_budget},
        users::create_user,
    },
};
use crate::{domain::budgets::get_budgets, errors::internal_error};
use axum::{
    routing::{delete, get, post},
    Router,
};
use clerk_rs::{clerk::Clerk, validators::axum::ClerkLayer, ClerkConfiguration};
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use domain::routehandlers::delete_handler;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import modules
mod config;
mod domain;
mod errors;
mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct AppState {
    pub client: Clerk,
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

    // Initialize Clerk client
    let content = std::fs::read_to_string("./Secrets.toml").unwrap();
    let secrets: toml::Value = toml::from_str(&content).unwrap();
    let clerk_secret_key = secrets["CLERK_SECRET_KEY"].to_string();
    let clerk_config: ClerkConfiguration =
        ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
    let client = Clerk::new(clerk_config.clone());

    // Create shared state
    let shared_state = Arc::new(AppState { client, pool });

    // Set app  router
    let app = Router::new()
        .route("/delete/:type", delete(delete_handler))
        .route("/create", post(create_budget))
        .route("/update", post(update_budget))
        .route("/get", get(get_budgets))
        .route("/users", post(create_user))
        .layer(ClerkLayer::new(clerk_config, None, true))
        .with_state(shared_state);

    // Get server address and create server
    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    let socket_addr: SocketAddr = address.parse().unwrap();

    // Log server listening address
    tracing::debug!("Listening on http://{}", socket_addr);

    // Start Axum server
    let tcp_listener = TcpListener::bind(&socket_addr).await.unwrap();

    axum::serve(tcp_listener, app)
        .await
        .map_err(internal_error)
        .unwrap();
}

// Create function to initialize tracing for logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Create function to run migrations
async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}
