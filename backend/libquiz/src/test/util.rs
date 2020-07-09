#![cfg(test)]

use sqlx::postgres::{PgConnection, PgPool};
use sqlx::Connect;

pub const TEST_DB_URL: &str = "postres://postgres@localhost/quiz_test";

pub fn new_connection() -> PgConnection {
    smol::run(PgConnection::connect(TEST_DB_URL)).expect("failed to connect to database")
}

pub fn new_pool() -> PgPool {
    smol::run(PgPool::builder().max_size(1).build(TEST_DB_URL))
        .expect("failed to create connection pool")
}
