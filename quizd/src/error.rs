use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizdError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("template error: {0}")]
    TemplateError(String),
}
