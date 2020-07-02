use crate::types::id::{OptionId, QuestionId};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "An option")]
pub struct QuizOption {
    pub quiz_option_id: OptionId,
    pub quiz_question_id: QuestionId,
    pub option_data: String,
    pub is_correct: bool,
    pub position: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Option")]
pub struct QuizOptionInput {
    pub quiz_question_id: QuestionId,
    pub option_data: String,
    pub is_correct: bool,
    pub position: Option<i32>,
}
