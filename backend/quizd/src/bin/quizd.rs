#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use quizd::AppState;
use rocket::config::Environment;
use std::path::PathBuf;
use structopt::StructOpt;
use tera::Tera;

#[derive(StructOpt, Debug)]
#[structopt(name = "quizd")]
struct Opt {
    /// URL for QuizApi
    #[structopt(
        short = "a",
        long = "api",
        default_value = "http://localhost:8001/graphql",
        env = "QUIZ_API_URL"
    )]
    api_url: String,

    /// Port to use for hosting.
    #[structopt(short = "p", long, default_value = "8000", env = "QUIZ_WEB_PORT")]
    port: u16,

    /// Bind address
    #[structopt(short = "h", long, default_value = "localhost", env = "QUIZ_WEB_HOST")]
    host: String,

    /// Template directory
    #[structopt(
        short = "t",
        long,
        default_value = "templates",
        env = "QUIZ_WEB_TEMPLATE_DIR"
    )]
    templates: PathBuf,

    /// Supress logging
    #[structopt(long)]
    silent: bool,
}

fn init_template_engine(directory: PathBuf) -> Result<Tera, tera::Error> {
    Tera::new(
        directory
            .join("**")
            .as_path()
            .to_str()
            .expect("Failed to convert path to string"),
    )
}

fn main() {
    use quizd::routes;

    dotenv().ok();

    let opt = Opt::from_args();

    let log_level = match opt.silent {
        true => rocket::config::LoggingLevel::Off,
        false => rocket::config::LoggingLevel::Normal,
    };

    let template_engine =
        init_template_engine(opt.templates).expect("failed to init template engine");

    let app_state = AppState {
        api_url: opt.api_url,
        template_engine,
    };

    let rocket_config = rocket::Config::build(Environment::Development)
        .port(opt.port)
        .address(&opt.host)
        .log_level(log_level)
        .finalize()
        .expect("Invalid server configuration");

    rocket::custom(rocket_config)
        .manage(app_state)
        .mount(
            "/",
            rocket::routes![
                routes::gather_identity::get_identity,
                routes::gather_identity::submit_identity,
                routes::take_quiz::get_quiz,
                routes::submit_quiz::submit,
            ],
        )
        .launch();
}
