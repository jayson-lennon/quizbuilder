use juniper::{FieldError, FieldResult, RootNode};
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::{schema, types::id::QuizId};

pub struct Context {
    pub dbpool: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn quizzes(context: &Context) -> FieldResult<Vec<schema::Quiz>> {
        todo!();
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn quiz(context: &Context, id: String) -> FieldResult<schema::Quiz> {
        Ok(schema::Quiz {
            quiz_id: QuizId(Uuid::new_v4()),
            name: "sample".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_quiz(context: &Context, quiz: schema::QuizInput) -> FieldResult<schema::Quiz> {
        todo!();
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
