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
    #[graphql(description = "List of all users")]
    fn quizzes(context: &Context) -> FieldResult<Vec<schema::Quiz>> {
        todo!();
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn quiz(context: &Context, id: QuizId) -> FieldResult<schema::Quiz> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        let quiz = smol::run(db::quiz::find_by_id(id, &mut conn))?;
        Ok(quiz)
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

pub fn new() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
