use rocket::{response::content::Html, State};
use tera::Context;

use crate::{error::QuizMgmtdError, AppState};

#[rocket::get("/quizlist")]
pub fn get(app_state: State<AppState>) -> Result<Html<String>, QuizMgmtdError> {
    let client = reqwest::blocking::Client::new();

    let api_query =
        r#"{"operationName":null,"variables":{},"query":"{getAllQuizzes {shortcode name}}"}"#;
    let quiz_data: serde_json::Value = {
        let res = client
            .post(&app_state.api_url)
            .body(api_query.to_string())
            .header("Content-Type", "application/json")
            .send()?
            .text()?;
        let res: serde_json::Value = serde_json::from_str(&res)?;
        res["data"]["getAllQuizzes"].clone()
    };

    let mut context = Context::new();
    context.insert("quizzes", &quiz_data);

    let template = app_state
        .template_engine
        .render("quizlist.tera", &context)?;

    Ok(Html(template))
}
