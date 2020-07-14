use crate::{
    config::{IDENTITY_COOKIE, QUIZ_START_TIME_COOKIE},
    AppState, QuizdError,
};
use chrono::{DateTime, SecondsFormat, Utc};
use rocket::{
    http::{Cookie, Cookies},
    request::{FormItems, FromForm, LenientForm},
    response::content::Html,
    State,
};
use rocket_contrib::uuid::Uuid;

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
            let key = {
                let decoded = item.key.url_decode().expect("failed to decode form key");
                if decoded.contains("-") {
                    decoded.split("-").collect::<Vec<_>>()[0].to_owned()
                } else {
                    decoded.to_owned()
                }
            };
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
        if let Some(quiz_id) = quiz_id {
            Ok(QuizFormSubmission { quiz_id, answers })
        } else {
            Err(())
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
    let identity = identity.replace(r#"""#, r#"\\\""#);
    let identity = identity.replace("\n", "");
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
    let mutation_query = mutation_query.replace("_IDENTITY_", &identity);
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

#[rocket::post("/submit_quiz", data = "<submission>")]
pub fn submit(
    submission: LenientForm<QuizFormSubmission>,
    app_state: State<AppState>,
    mut cookies: Cookies,
) -> Result<Html<String>, QuizdError> {
    let quiz_start_time = match cookies.get_private(QUIZ_START_TIME_COOKIE) {
        Some(start_time) => {
            let start_time = start_time.value();
            DateTime::parse_from_rfc3339(start_time)?
        }
        None => return Err(QuizdError::MissingQuizStartTime),
    };

    let identity = match cookies.get(IDENTITY_COOKIE) {
        Some(identity) => identity.value(),
        None => return Err(QuizdError::MissingIdentity),
    };

    let start_time = quiz_start_time;
    let finish_time = Utc::now();
    let mutation_query = create_mutation_query(
        identity,
        start_time.into(),
        finish_time,
        submission.into_inner(),
    );

    let client = reqwest::blocking::Client::new();

    let _ = client
        .post(&app_state.api_url)
        .body(mutation_query)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    cookies.remove(Cookie::named(IDENTITY_COOKIE));
    cookies.remove_private(Cookie::named(QUIZ_START_TIME_COOKIE));

    Ok(Html("Quiz submitted successfully".to_string()))
}
