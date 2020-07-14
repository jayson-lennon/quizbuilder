pub mod config;
pub mod quiz;
pub mod quiz_answer;
pub mod quiz_option;
pub mod quiz_question;
pub mod quiz_score;
pub mod quiz_submission;

mod init;
pub use init::new_pool;
