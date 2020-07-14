use super::QuizSubmission;
use crate::types::id::QuizId;
use juniper::GraphQLObject;

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A quiz score")]
pub struct QuizScore {
    pub quiz_id: QuizId,
    pub submission: QuizSubmission,
    pub total_questions: i32,
    pub total_correct: i32,
}
