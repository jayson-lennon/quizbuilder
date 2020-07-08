use crate::{config::IDENTITY_COOKIE, AppState, QuizdError};
use rocket::{http::Cookies, response::content::Html, response::Redirect, uri, Responder, State};
use tera::Context;

fn generate_get_quiz_api_query(shortcode: &str) -> Result<String, QuizdError> {
    // TODO: Pull shortcode options from API and cache results.
    // NOTE: When implementing shortcode API request, the cache should be refreshed
    //       whenever shortcode validation fails. This would be an indication
    //       that the shortcode rules have changed.
    let shortcode_options = libquiz::db::quiz::ShortCodeOptions::default();
    if !libquiz::db::quiz::validate_shortcode(&shortcode, &shortcode_options) {
        return Err(QuizdError::InvalidShortcode);
    }

    let api_query = {
        let graphql_query = r#""{ quizFromShortcode(shortcode:\"_SHORTCODE_\") { quizId, openDate, closeDate, duration, questions { quizQuestionId, position, questionData, options { quizOptionId, optionData, position, optionType } } } }""#;
        let graphql_query = graphql_query.replace("_SHORTCODE_", &shortcode);
        let json_request = r#"{
                "query": _GRAPHQL_QUERY_,
                "variables": null
            }"#;
        json_request.replace("_GRAPHQL_QUERY_", &graphql_query)
    };

    Ok(api_query)
}

fn submit_get_quiz_api_query(
    client: &reqwest::blocking::Client,
    api_url: &str,
    api_query: &str,
) -> Result<serde_json::Value, QuizdError> {
    // TODO: Cache result of quiz API response.
    // NOTE: Quizzes are immutable once they have been opened. However,
    //       the "closeDate" of a quiz should be respected.
    let res = client
        .post(api_url)
        .body(api_query.to_string())
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let res: serde_json::Value = serde_json::from_str(&res)?;
    let data = res["data"]["quizFromShortcode"].clone();
    Ok(data)
}

#[derive(Debug, Responder)]
pub enum TakeQuizResponse {
    Template(Html<String>),
    Redirect(Redirect),
}

#[rocket::get("/q/take/<shortcode>")]
pub fn get_quiz(
    shortcode: String,
    app_state: State<AppState>,
    cookies: Cookies,
) -> Result<TakeQuizResponse, QuizdError> {
    let identity = match cookies.get(IDENTITY_COOKIE) {
        Some(identity) => identity.value(),
        None => {
            return Ok(TakeQuizResponse::Redirect(Redirect::to(uri!(
                super::gather_identity::get_identity: shortcode
            ))))
        }
    };

    let client = reqwest::blocking::Client::new();

    let api_query = generate_get_quiz_api_query(&shortcode)?;
    let quiz_data = submit_get_quiz_api_query(&client, &app_state.api_url, &api_query)?;

    let mut context =
        Context::from_value(quiz_data).expect("failed to convert api result into context");
    context.insert(IDENTITY_COOKIE, &identity);

    let template = app_state
        .template_engine
        .render("quiz.html.tera", &context)?;

    Ok(TakeQuizResponse::Template(Html(template)))
}
