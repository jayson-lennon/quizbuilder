use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizdError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("template error: {0}")]
    TemplateError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),
}

impl From<reqwest::Error> for QuizdError {
    fn from(error: reqwest::Error) -> Self {
        QuizdError::ApiError(format!("{}", error))
    }
}

impl From<serde_json::Error> for QuizdError {
    fn from(error: serde_json::Error) -> Self {
        QuizdError::DeserializationError(format!("{}", error))
    }
}

impl From<handlebars::RenderError> for QuizdError {
    fn from(error: handlebars::RenderError) -> Self {
        QuizdError::TemplateError(format!("{}", error))
    }
}

impl From<handlebars::TemplateError> for QuizdError {
    fn from(error: handlebars::TemplateError) -> Self {
        QuizdError::TemplateError(format!("{}", error))
    }
}

impl From<handlebars::TemplateRenderError> for QuizdError {
    fn from(error: handlebars::TemplateRenderError) -> Self {
        QuizdError::TemplateError(format!("{}", error))
    }
}
