use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

#[actix_rt::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in env");

    let db_pool = PgPool::builder().max_size(10).build(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .configure(libquiz::handlers::register)
            .default_service(web::to(|| async { "404 " }))
    });

    let host = env::var("API_HOST").expect("API_HOST not set in env");
    let port = env::var("API_PORT").expect("API_PORT not set in env");

    server = server.bind(format!("{}:{}", host, port))?;

    server.run().await?;

    Ok(())
}
