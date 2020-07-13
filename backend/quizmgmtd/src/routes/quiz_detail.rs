use rocket::{response::content::Html, State};
use tera::Context;

use crate::{error::QuizMgmtdError, AppState};

#[rocket::get("/quiz/<shortcode>")]
pub fn get(shortcode: String, app_state: State<AppState>) -> Result<Html<String>, QuizMgmtdError> {
    let client = reqwest::blocking::Client::new();

    let shortcode_options = libquiz::db::quiz::ShortCodeOptions::default();
    if !libquiz::db::quiz::validate_shortcode(&shortcode, &shortcode_options) {
        return Err(QuizMgmtdError::InvalidShortcode);
    }

    let api_query = r#"{"operationName":null,"variables":{},"query":"{quizFromShortcode(shortcode:\"__SHORTCODE__\") {name shortcode dateCreated openDate closeDate duration questions { questionData options { optionData isCorrect optionType }}}}"}"#.replace("__SHORTCODE__", &shortcode);
    let quiz_data: serde_json::Value = {
        let res = client
            .post(&app_state.api_url)
            .body(api_query.to_string())
            .header("Content-Type", "application/json")
            .send()?
            .text()?;
        let res: serde_json::Value = serde_json::from_str(&res)?;
        res["data"]["quizFromShortcode"].clone()
    };

    let mut context = Context::new();
    context.insert("quiz", &quiz_data);

    let template = app_state
        .template_engine
        .render("quiz-detail.html.tera", &context)?;

    Ok(Html(template))
}
