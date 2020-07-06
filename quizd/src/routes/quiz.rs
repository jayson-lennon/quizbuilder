use crate::{AppState, QuizdError};
use rocket::response::content::Html;
use rocket::State;

#[rocket::get("/q/<shortcode>")]
pub fn get(shortcode: String, app_state: State<AppState>) -> Result<Html<String>, QuizdError> {
    let client = reqwest::blocking::Client::new();

    // TODO: Pull shortcode options from API and cache results.
    // NOTE: When implementing shortcode API request, the cache should be refreshed
    //       whenever shortcode validation fails. This would be an indication
    //       that the shortcode rules have changed.
    let shortcode_options = libquiz::db::quiz::ShortCodeOptions::default();
    if !libquiz::db::quiz::validate_shortcode(&shortcode, &shortcode_options) {
        return Err(QuizdError::InvalidShortcode);
    }

    let api_query = {
        let graphql_query = r#""{ quizFromShortcode(shortcode:\"_SHORTCODE_\") { quizId, dateCreated, openDate, closeDate, duration, questions { quizQuestionId, quizId, position, questionData, options { quizOptionId, optionData, position } } } }""#;
        let graphql_query = graphql_query.replace("_SHORTCODE_", &shortcode);
        let json_request = r#"{
                "query": _GRAPHQL_QUERY_,
                "variables": null
            }"#;
        json_request.replace("_GRAPHQL_QUERY_", &graphql_query)
    };

    // TODO: Cache result of quiz response.
    // NOTE: Quizzes are immutable once they have been opened, so it is ok to cache
    //       a quiz for any amount of time.
    let res = client
        .post(&app_state.api_url)
        .body(api_query)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let res: serde_json::Value = serde_json::from_str(&res)?;
    let template = app_state.template_engine.render("index", &res)?;
    Ok(Html(template))
}
