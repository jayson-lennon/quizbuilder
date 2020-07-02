use crate::types::id::QuizId;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A quiz")]
pub struct Quiz {
    pub quiz_id: QuizId,
    pub name: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Quiz input")]
pub struct QuizInput {
    pub name: Option<String>,
}
