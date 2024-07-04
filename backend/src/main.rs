use crate::domain::{
    budgets::{create_budget, update_budget},
    users::create_user,
};
use crate::{domain::budgets::get_budgets, errors::internal_error};
use axum::{
    routing::{delete, get, post},
    Router,
};
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use config::config;
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use futures_util::{future::BoxFuture, FutureExt};
use std::{net::SocketAddr, sync::Arc};

use domain::{
    budgets::{delete_all_budgets, delete_budget},
    transactions::{
        create_transactions, delete_all_transactions, delete_transactions, get_transactions,
        update_transaction,
    },
    users::{get_user, update_user},
};
use rustls::*;
use tokio::net::TcpListener;
use tracing::*;

// Import modules
mod config;
mod domain;
mod errors;
mod schema;

pub static MIGRATIONS: diesel_async_migrations::EmbeddedMigrations =
    diesel_async_migrations::embed_migrations!();

pub struct AppState {
    pub client: Clerk,
    pub pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    println!("Tracing");
    tracing_subscriber::fmt::init();
    info!("Print this");
    println!("Enter Config");
    let mut mgr_config = ManagerConfig::default();
    mgr_config.custom_setup = Box::new(establish_connection);
    let config = config().await;

    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
        config.db_url().to_string(),
        mgr_config,
    );
    let pool = Pool::builder(mgr).build().expect("Should Build Pool");

    println!("Pool Built");
    // Run migrations
    let _ = run_migrations(&pool).await;
    println!("Migrations Run");
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
        .route("/delete/dab", delete(delete_all_budgets))
        .route("/delete/dat", delete(delete_all_transactions))
        .route("/delete/dt", delete(delete_transactions))
        .route("/delete/db", delete(delete_budget))
        .route("/create/budget", post(create_budget))
        .route("/create/transactions", post(create_transactions))
        .route("/create/users", post(create_user))
        .route("/update/budget", post(update_budget))
        .route("/update/transactions", post(update_transaction))
        .route("/update/user", post(update_user))
        .route("/get/budgets", post(get_budgets))
        .route("/get/transactions", get(get_transactions))
        .route("/get/user", post(get_user))
        //        .layer(ClerkLayer::new(clerk_config, None, true))
        .with_state(shared_state);

    // Get server address and create server
    let host = "127.0.0.1";
    let port = "8080";

    let address = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = address.parse().unwrap();

    // Log server listening address
    println!("Listening on http://{}", socket_addr);

    // Start Axum server
    let tcp_listener = TcpListener::bind(&socket_addr).await.unwrap();

    axum::serve(tcp_listener, app)
        .await
        .map_err(internal_error)
        .unwrap();
}

// Create function to initialize tracing for logging
//fn init_tracing() {
//   tracing_subscriber::registry()
//     .with(
//       tracing_subscriber::EnvFilter::try_from_default_env()
//             .unwrap_or_else(|_| "debug".into()),
//    )
//    .with(tracing_subscriber::fmt::layer())
//   .init();
//}

// Create function to run migrations
async fn run_migrations(pool: &Pool<AsyncPgConnection>) -> anyhow::Result<()> {
    let mut conn = pool.get().await.unwrap();
    MIGRATIONS.run_pending_migrations(&mut conn).await?;
    Ok(())
}
fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_certs())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

fn root_certs() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().expect("Certs not loadable!");
    let certs: Vec<_> = certs.into_iter().map(|cert| cert.0).collect();
    roots.add_parsable_certificates(&certs);
    roots
}
