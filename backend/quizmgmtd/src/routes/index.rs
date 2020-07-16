use rocket::{response::content::Html, State};
use tera::Context;

use crate::{error::QuizMgmtdError, AppState};

#[rocket::get("/")]
pub fn get(app_state: State<AppState>) -> Result<Html<String>, QuizMgmtdError> {
    let context = Context::new();

    let template = app_state.template_engine.render("index.tera", &context)?;

    Ok(Html(template))
}
