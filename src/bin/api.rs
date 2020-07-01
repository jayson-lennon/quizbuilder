#![feature(decl_macro, proc_macro_hygiene)]

use rocket::config::Environment;
use rocket::{response::content, State};

use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use structopt::StructOpt;

#[rocket::get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<libquiz::schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<libquiz::schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<libquiz::schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<libquiz::schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

/// A simple tool to test frontend code with faked API requests
#[derive(StructOpt, Debug)]
#[structopt(name = "spa-host")]
struct Opt {
    /// Port to use for hosting.
    #[structopt(short = "p", long, default_value = "8000", env = "QUIZ_API_PORT")]
    api_port: u16,

    /// Bind address
    #[structopt(short = "h", long, default_value = "localhost", env = "QUIZ_API_HOST")]
    api_host: String,

    /// Database connection pool size
    #[structopt(long, default_value = "10", env = "QUIZ_DB_POOL_SIZE")]
    db_pool_size: u32,

    /// Database url
    #[structopt(
        short = "d",
        long,
        default_value = "postgres://postgres@localhost/quiz",
        env = "QUIZ_DATABASE_URL"
    )]
    database_url: String,
}

fn main() {
    dotenv().ok();

    let opt = Opt::from_args();

    let db_pool = smol::run(libquiz::db::new_pool(&opt.database_url, opt.db_pool_size))
        .expect("failed to init db pool");

    let rocket_config = rocket::Config::build(Environment::Development)
        .port(opt.api_port)
        .address(&opt.api_host)
        .finalize()
        .expect("Invalid server configuration");

    let juniper_context = libquiz::schema::Context {
        db_pool: db_pool.clone(),
    };

    rocket::custom(rocket_config)
        .manage(db_pool)
        .manage(libquiz::schema::new())
        .manage(juniper_context)
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
