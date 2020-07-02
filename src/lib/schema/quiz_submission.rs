use crate::schema::QuizAnswer;
use crate::types::id::{QuizId, SubmissionId};
use chrono::{DateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A submission")]
pub struct QuizSubmission {
    pub quiz_submission_id: SubmissionId,
    pub identity: String,
    pub quiz_id: QuizId,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
    pub answers: Vec<QuizAnswer>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New submission")]
pub struct QuizSubmissionInput {
    pub identity: String,
    pub quiz_id: QuizId,
    pub start_date: DateTime<Utc>,
    pub finish_date: Option<DateTime<Utc>>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A submission")]
pub struct QuizSubmissions {
    pub submissions: Vec<QuizSubmission>,
}
