use rocket::{response::content::Html, State};
use tera::Context;

use crate::{error::QuizMgmtdError, AppState};

fn validate_shortcode(shortcode: &str) -> Result<(), QuizMgmtdError> {
    let shortcode_options = libquiz::db::quiz::ShortCodeOptions::default();
    if !libquiz::db::quiz::validate_shortcode(&shortcode, &shortcode_options) {
        Err(QuizMgmtdError::InvalidShortcode)
    } else {
        Ok(())
    }
}

fn quiz_from_shortcode(
    shortcode: &str,
    app_state: &State<AppState>,
) -> Result<serde_json::Value, QuizMgmtdError> {
    let _ = validate_shortcode(shortcode)?;

    let client = reqwest::blocking::Client::new();
    let api_query = r#"{"operationName":null,"variables":{},"query":"{quizFromShortcode(shortcode:\"__SHORTCODE__\") {name shortcode dateCreated openDate closeDate duration questions { questionData options { optionData isCorrect optionType }}}}"}"#.replace("__SHORTCODE__", shortcode);
    let res = client
        .post(&app_state.api_url)
        .body(api_query.to_string())
        .header("Content-Type", "application/json")
        .send()?
        .text()?;
    let res: serde_json::Value = serde_json::from_str(&res)?;

    Ok(res["data"]["quizFromShortcode"].clone())
}

fn scores_from_shortcode(
    shortcode: &str,
    app_state: &State<AppState>,
) -> Result<serde_json::Value, QuizMgmtdError> {
    let _ = validate_shortcode(shortcode)?;

    let client = reqwest::blocking::Client::new();
    let api_query = r#"{"operationName":null,"variables":{},"query":"{quizScore(shortcode:\"__SHORTCODE__\") {totalQuestions totalCorrect submission { quizSubmissionId identity }}}"}"#.replace("__SHORTCODE__", &shortcode);
    let res = client
        .post(&app_state.api_url)
        .body(api_query.to_string())
        .header("Content-Type", "application/json")
        .send()?
        .text()?;
    let res: serde_json::Value = serde_json::from_str(&res)?;
    Ok(res["data"]["quizScore"].clone())
}

#[rocket::get("/quiz/questions/<shortcode>")]
pub fn get_questions(
    shortcode: String,
    app_state: State<AppState>,
) -> Result<Html<String>, QuizMgmtdError> {
    let quiz_data = quiz_from_shortcode(&shortcode, &app_state)?;

    let mut context = Context::new();
    context.insert("quiz", &quiz_data);

    let template = app_state
        .template_engine
        .render("quiz-questions.tera", &context)?;

    Ok(Html(template))
}

#[rocket::get("/quiz/<shortcode>")]
pub fn index(
    shortcode: String,
    app_state: State<AppState>,
) -> Result<Html<String>, QuizMgmtdError> {
    let quiz_data = quiz_from_shortcode(&shortcode, &app_state)?;
    let scores = scores_from_shortcode(&shortcode, &app_state)?;

    let mut context = Context::new();
    context.insert("quiz", &quiz_data);
    context.insert("scores", &scores);

    let template = app_state
        .template_engine
        .render("quiz-detail.tera", &context)?;

    Ok(Html(template))
}
