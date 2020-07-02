#![feature(decl_macro, proc_macro_hygiene)]

use handlebars::Handlebars;

pub mod error;
pub use error::QuizdError;

pub mod handlebars_helpers;
pub mod routes;

pub struct AppState {
    pub api_url: String,
    pub template_engine: Handlebars<'static>,
}
