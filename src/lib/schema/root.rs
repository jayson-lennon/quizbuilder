use juniper::{FieldResult, RootNode};
use sqlx::postgres::PgPool;

use crate::{db, schema, types::id::QuizId};

pub struct Context {
    pub db_pool: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Get all quizzes")]
    fn quizzes(context: &Context) -> FieldResult<Vec<schema::Quiz>> {
        todo!();
    }

    #[graphql(description = "Get a Quiz by ID")]
    fn quiz(context: &Context, id: QuizId) -> FieldResult<schema::Quiz> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz::find_by_id(id, &mut conn))?)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_quiz(context: &Context, quiz: schema::QuizInput) -> FieldResult<schema::Quiz> {
        todo!();
    }

    fn create_quiz_question(
        context: &Context,
        quiz_question: schema::QuizQuestionInput,
    ) -> FieldResult<schema::QuizQuestion> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_question::new(quiz_question, &mut conn))?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn new() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
