use crate::types::id::QuizId;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A sample quiz")]
pub struct Quiz {
    pub quiz_id: QuizId,
    pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Quiz input")]
pub struct QuizInput {
    pub name: String,
}
