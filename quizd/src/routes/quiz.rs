use crate::{AppState, QuizdError};
use chrono::{DateTime, SecondsFormat, Utc};
use rocket::{
    request::{FormItems, FromForm, LenientForm},
    response::content::Html,
    State,
};
use rocket_contrib::uuid::Uuid;
use tera::Context;

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
        let graphql_query = r#""{ quizFromShortcode(shortcode:\"_SHORTCODE_\") { quizId, openDate, closeDate, duration, questions { quizQuestionId, position, questionData, options { quizOptionId, optionData, position, optionType } } } }""#;
        let graphql_query = graphql_query.replace("_SHORTCODE_", &shortcode);
        let json_request = r#"{
                "query": _GRAPHQL_QUERY_,
                "variables": null
            }"#;
        json_request.replace("_GRAPHQL_QUERY_", &graphql_query)
    };

    // TODO: Cache result of quiz API response.
    // NOTE: Quizzes are immutable once they have been opened. However,
    //       the "closeDate" of a quiz should be respected.
    let res = client
        .post(&app_state.api_url)
        .body(api_query)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let res: serde_json::Value = serde_json::from_str(&res)?;
    let data = res["data"]["quizFromShortcode"].clone();
    let context = Context::from_value(data).expect("failed to convert api result into context");

    let template = app_state
        .template_engine
        .render("index.html.tera", &context)?;
    Ok(Html(template))
}

#[derive(Debug)]
pub struct Answer {
    question_id: Uuid,
    option_id: Uuid,
}

#[derive(Debug)]
pub struct QuizFormSubmission {
    quiz_id: Uuid,
    answers: Vec<Answer>,
}

impl<'f> FromForm<'f> for QuizFormSubmission {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _: bool) -> Result<QuizFormSubmission, ()> {
        use std::str::FromStr;

        let mut quiz_id = None;
        let mut answers = vec![];

        // TODO: error handling
        for item in items {
            let key = item.key.url_decode().expect("failed to decode form key");
            match key.as_str() {
                "quiz_id" => {
                    let quiz_id_as_str = item.value.url_decode().expect("failed to decode quiz id");
                    quiz_id = Uuid::from_str(&quiz_id_as_str).ok();
                }
                "answer" => {
                    let value = item
                        .value
                        .url_decode()
                        .expect("failed to decode form value");
                    let answer_pair: Vec<&str> = value.split('_').collect();
                    if answer_pair.len() != 2 {
                        panic!("wrong pair length");
                    }
                    let answer = Answer {
                        question_id: Uuid::from_str(answer_pair[0])
                            .expect("failed to parse question id"),
                        option_id: Uuid::from_str(answer_pair[1])
                            .expect("failed to parse option id"),
                    };
                    answers.push(answer);
                }
                _ => (),
            }
        }
        match quiz_id {
            Some(id) => Ok(QuizFormSubmission {
                quiz_id: id,
                answers,
            }),
            None => Err(()),
        }
    }
}

// TODO: Make graphql query builder/macro.
fn create_mutation_query(
    identity: &str,
    start_time: DateTime<Utc>,
    finish_time: DateTime<Utc>,
    form: QuizFormSubmission,
) -> String {
    let compiled_answers = {
        let mut answers = vec![];
        let answer_chunk =
            r#"{quizQuestionId: \"_QUIZ_QUESTION_ID_\", quizOptionId: \"_QUIZ_OPTION_ID_\"}"#;
        for answer in form.answers.iter() {
            let formatted_answer =
                answer_chunk.replace("_QUIZ_QUESTION_ID_", &answer.question_id.to_string());
            let formatted_answer =
                formatted_answer.replace("_QUIZ_OPTION_ID_", &answer.option_id.to_string());
            answers.push(formatted_answer);
        }
        answers.join(",")
    };

    let mutation_query = r#"mutation { createQuizSubmission(quizSubmission: {identity: \"_IDENTITY_\", quizId: \"_QUIZ_ID_\", startDate: \"_START_TIME_\", finishDate: \"_FINISH_TIME_\", answers: [_ANSWERS_]}) { quizSubmissionId }}"#;
    let mutation_query = mutation_query.replace("_IDENTITY_", identity);
    let mutation_query = mutation_query.replace("_QUIZ_ID_", &form.quiz_id.to_string());
    let mutation_query = mutation_query.replace(
        "_START_TIME_",
        &format!(
            "{}",
            start_time.to_rfc3339_opts(SecondsFormat::Millis, true)
        ),
    );
    let mutation_query = mutation_query.replace(
        "_FINISH_TIME_",
        &format!(
            "{}",
            finish_time.to_rfc3339_opts(SecondsFormat::Millis, true)
        ),
    );
    let mutation_query = mutation_query.replace("_ANSWERS_", &compiled_answers);
    let json_request = r#"{
        "operationName":null,
        "variables":{},
        "query": "_GRAPHQL_QUERY_"
    }"#;
    json_request.replace("_GRAPHQL_QUERY_", &mutation_query)
}

#[rocket::post("/submit", data = "<submission>")]
pub fn submit(
    submission: LenientForm<QuizFormSubmission>,
    app_state: State<AppState>,
) -> Result<Html<String>, QuizdError> {
    let identity = "sample_identity";
    let start_time = Utc::now();
    let finish_time = Utc::now();
    let mutation_query =
        create_mutation_query(identity, start_time, finish_time, submission.into_inner());

    let client = reqwest::blocking::Client::new();

    let _ = client
        .post(&app_state.api_url)
        .body(mutation_query)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    Ok(Html("ok".to_string()))
}
