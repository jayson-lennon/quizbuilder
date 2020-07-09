#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate log;

use tera::Tera;

pub mod config;

pub mod error;
pub use error::QuizdError;

pub mod routes;

pub struct AppState {
    pub api_url: String,
    pub template_engine: Tera,
}
