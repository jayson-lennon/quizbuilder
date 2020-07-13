#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate log;

use tera::Tera;

pub mod error;
pub mod routes;

pub struct AppState {
    pub api_url: String,
    pub template_engine: Tera,
}
