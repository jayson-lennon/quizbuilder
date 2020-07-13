use crate::schema::{FullQuizOptionInput, QuizOption};
use crate::types::id::{QuestionId, QuizId};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A question")]
pub struct QuizQuestion {
    pub quiz_question_id: QuestionId,
    pub quiz_id: QuizId,
    pub question_data: String,
    pub position: Option<i32>,
    pub options: Vec<QuizOption>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New question")]
pub struct QuizQuestionInput {
    pub quiz_id: QuizId,
    pub question_data: String,
    pub position: Option<i32>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New question as part of a full quiz")]
pub struct FullQuizQuestionInput {
    pub question_data: String,
    pub position: Option<i32>,
    pub options: Vec<FullQuizOptionInput>,
}
