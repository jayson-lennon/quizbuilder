use rocket::{response::content::Html, State};
use rocket_contrib::uuid::Uuid;
use std::str::FromStr;
use tera::Context;

use crate::{error::QuizMgmtdError, AppState};

fn submission_details(
    submission_id: Uuid,
    app_state: &State<AppState>,
) -> Result<serde_json::Value, QuizMgmtdError> {
    let client = reqwest::blocking::Client::new();
    let api_query = r#"{"operationName":null,"variables":{},"query":"{submission(submissionId:\"__SUBMISSION_ID__\") {quizId startDate finishDate identity answers { quizQuestionId quizOptionId }}}"}"#.replace("__SUBMISSION_ID__", &submission_id.to_string());
    let res = client
        .post(&app_state.api_url)
        .body(api_query.to_string())
        .header("Content-Type", "application/json")
        .send()?
        .text()?;
    let res: serde_json::Value = serde_json::from_str(&res)?;
    Ok(res["data"]["submission"].clone())
}

fn quiz_from_id(
    quiz_id: Uuid,
    app_state: &State<AppState>,
) -> Result<serde_json::Value, QuizMgmtdError> {
    let client = reqwest::blocking::Client::new();
    let api_query = r#"{"operationName":null,"variables":{},"query":"{quiz(quizId:\"__QUIZ_ID__\") {name shortcode dateCreated openDate closeDate duration questions { quizQuestionId questionData options { quizOptionId optionData isCorrect }}}}"}"#.replace("__QUIZ_ID__", &quiz_id.to_string());
    let res = client
        .post(&app_state.api_url)
        .body(api_query.to_string())
        .header("Content-Type", "application/json")
        .send()?
        .text()?;
    let res: serde_json::Value = serde_json::from_str(&res)?;

    Ok(res["data"]["quiz"].clone())
}

fn apply_answers(quiz: &mut serde_json::Value, submission: &serde_json::Value) {
    let questions = quiz["questions"]
        .as_array_mut()
        .expect("failed to get quiz questions array");
    let answers = submission["answers"]
        .as_array()
        .expect("failed to get submission answer array");

    for question in questions {
        let options = question["options"]
            .as_array_mut()
            .expect("failed to get option array");
        for option in options {
            option["marked"] = serde_json::Value::Bool(false);
            for answer in answers {
                if option["quizOptionId"]
                    .as_str()
                    .expect("failed to get quiz option id")
                    == answer["quizOptionId"]
                {
                    option["marked"] = serde_json::Value::Bool(true);
                }
            }
        }
    }
}

#[rocket::get("/quiz/submission/<submission_id>")]
pub fn get(
    submission_id: Uuid,
    app_state: State<AppState>,
) -> Result<Html<String>, QuizMgmtdError> {
    let submission = submission_details(submission_id, &app_state)?;
    let quiz_id = Uuid::from_str(submission["quizId"].as_str().unwrap_or_else(|| ""))?;
    let mut quiz = quiz_from_id(quiz_id, &app_state)?;

    apply_answers(&mut quiz, &submission);

    let mut context = Context::new();
    context.insert("quiz", &quiz);
    context.insert("submission", &submission);

    let template = app_state
        .template_engine
        .render("quiz-submission.tera", &context)?;

    Ok(Html(template))
}
