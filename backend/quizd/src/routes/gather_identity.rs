use crate::{
    config::{IDENTITY_COOKIE, QUIZ_START_TIME_COOKIE},
    AppState, QuizdError,
};
use chrono::{SecondsFormat, Utc};
use rocket::{
    http::{Cookie, Cookies},
    request::{Form, FromForm},
    response::content::Html,
    response::Redirect,
    uri, State,
};
use tera::Context;

#[rocket::get("/q/<shortcode>")]
pub fn get_identity(
    shortcode: String,
    app_state: State<AppState>,
) -> Result<Html<String>, QuizdError> {
    let mut context = Context::new();
    context.insert("quiz_shortcode", &shortcode);

    let template = app_state
        .template_engine
        .render("landing.html.tera", &context)?;

    Ok(Html(template))
}

#[derive(FromForm)]
pub struct IdentityInfo {
    pub quiz_shortcode: String,
    pub identity: String,
}

#[rocket::post("/submit_identity", data = "<submission>")]
pub fn submit_identity(submission: Form<IdentityInfo>, mut cookies: Cookies) -> Redirect {
    cookies.add(Cookie::new(
        IDENTITY_COOKIE,
        submission.identity.to_string(),
    ));

    let quiz_start_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let mut quiz_start_cookie = Cookie::new(QUIZ_START_TIME_COOKIE, quiz_start_time);
    quiz_start_cookie.set_secure(true);

    cookies.add_private(quiz_start_cookie);

    let shortcode = &submission.quiz_shortcode;

    Redirect::to(uri!(super::take_quiz::get_quiz: shortcode))
}
