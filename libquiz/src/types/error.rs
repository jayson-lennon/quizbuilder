use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibquizError {
    #[error("Quiz option error: {0}")]
    QuizOption(String),
}
