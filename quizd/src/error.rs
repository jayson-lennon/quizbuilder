use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizdError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("template error: {0}")]
    TemplateError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("shortcode validation error")]
    InvalidShortcode,

    #[error("missing identity")]
    MissingIdentity,

    #[error("missing quiz start time")]
    MissingQuizStartTime,

    #[error("time parse error: {0}")]
    TimeParseError(String),
}

impl<'r> Responder<'r> for QuizdError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        // TODO: better logging.
        error!("{}", self);
        Response::build().status(Status::NotFound).ok()
    }
}

impl From<reqwest::Error> for QuizdError {
    fn from(error: reqwest::Error) -> Self {
        QuizdError::ApiError(format!("{}", error))
    }
}

impl From<chrono::format::ParseError> for QuizdError {
    fn from(error: chrono::format::ParseError) -> Self {
        QuizdError::TimeParseError(format!("{}", error))
    }
}

impl From<serde_json::Error> for QuizdError {
    fn from(error: serde_json::Error) -> Self {
        QuizdError::DeserializationError(format!("{}", error))
    }
}

impl From<tera::Error> for QuizdError {
    fn from(error: tera::Error) -> Self {
        QuizdError::TemplateError(format!("{}", error))
    }
}
