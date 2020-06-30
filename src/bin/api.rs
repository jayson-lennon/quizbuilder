use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in env");

    let db_pool = PgPool::builder().max_size(10).build(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .route("/api/test", web::get().to(libquiz::route::test::test))
    });

    let host = env::var("API_HOST").expect("API_HOST not set in env");
    let port = env::var("API_PORT").expect("API_PORT not set in env");

    server = server.bind(format!("{}:{}", host, port))?;

    server.run().await?;

    Ok(())
}
