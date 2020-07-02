#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use handlebars::Handlebars;
use rocket::config::Environment;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// A simple tool to test frontend code with faked API requests
#[derive(StructOpt, Debug)]
#[structopt(name = "spa-host")]
struct Opt {
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

fn init_template_engine(directory: PathBuf) -> Result<Handlebars<'static>, Box<dyn Error>> {
    let mut engine = Handlebars::new();
    engine.register_template_file("index", directory.join("index.hbs"))?;
    engine.register_template_file("derp", directory.join("derp.hbs"))?;
    Ok(engine)
}

fn main() {
    use quizd::route;

    dotenv().ok();

    let opt = Opt::from_args();

    let log_level = match opt.silent {
        true => rocket::config::LoggingLevel::Off,
        false => rocket::config::LoggingLevel::Normal,
    };

    let template_engine =
        init_template_engine(opt.templates).expect("failed to init template engine");

    let rocket_config = rocket::Config::build(Environment::Development)
        .port(opt.port)
        .address(&opt.host)
        .log_level(log_level)
        .finalize()
        .expect("Invalid server configuration");

    rocket::custom(rocket_config)
        .manage(template_engine)
        .mount("/", rocket::routes![route::quiz::get])
        .launch();
}
