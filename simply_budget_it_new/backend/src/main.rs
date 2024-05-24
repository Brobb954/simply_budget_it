pub mod models;
pub mod schema;
use self::models::NewTransaction;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::env;

fn main() {

    let pool = get_connection_pool();

    let transaction = NewTransaction {
        description: "Test".to_string(),
        amount: 100.0,
        transaction_type: todo!(),
        transaction_date: todo!(),
    };

    add_transaction(&pool, transaction).expect("Should add transaction to database")
    
}

// Make sure to embed diesel migrations at top of main.rs

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder().test_on_check_out(true).build(manager).expect("Should create a connection pool")
}

pub fn add_transaction(pool: &Pool<ConnectionManager<PgConnection>>, transaction: Transaction) -> Result<(), Error> {

    Ok(())

}
