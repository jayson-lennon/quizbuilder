use crate::schema::QuizAnswer;
use crate::types::id::{OptionId, QuestionId, QuizId, SubmissionId};
use chrono::{DateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A submission")]
pub struct QuizSubmission {
    pub quiz_submission_id: SubmissionId,
    pub identity: String,
    pub quiz_id: QuizId,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub answers: Vec<QuizAnswer>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New submission")]
pub struct QuizSubmissionInput {
    pub identity: String,
    pub quiz_id: QuizId,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub answers: Vec<QuizSubmissionAnswerInput>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new answer")]
pub struct QuizSubmissionAnswerInput {
    pub quiz_question_id: QuestionId,
    pub quiz_option_id: OptionId,
}
