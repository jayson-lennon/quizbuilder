use crate::types::id::{QuestionId, QuizId};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A question")]
pub struct QuizQuestion {
    pub quiz_question_id: QuestionId,
    pub quiz_id: QuizId,
    pub question_data: String,
    pub position: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New question")]
pub struct QuizQuestionInput {
    pub quiz_id: QuizId,
    pub question_data: String,
    pub position: Option<i32>,
}
