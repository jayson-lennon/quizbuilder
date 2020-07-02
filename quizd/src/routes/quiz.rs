use crate::{AppState, QuizdError};
use rocket::response::content::Html;
use rocket::State;

#[rocket::get("/q/<shortcode>")]
pub fn get(shortcode: String, app_state: State<AppState>) -> Result<Html<String>, QuizdError> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(&app_state.api_url)
        .body(r#"{"query":"{\n  quizFromShortcode(shortcode:\"hay\") {\n    quizId\n    dateCreated\n    openDate\n    closeDate\n    duration\n    questions {\n      quizQuestionId\n      quizId\n      position\n      questionData\n      options {\n        quizOptionId\n        optionData\n        position\n      }\n    }\n  }\n}","variables":null}"#)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let res: serde_json::Value = serde_json::from_str(&res)?;
    let template = app_state.template_engine.render("index", &res)?;
    Ok(Html(template))
}
