use juniper::{FieldResult, RootNode};
use sqlx::postgres::PgPool;

use crate::{
    db, schema,
    types::id::{QuizId, SubmissionId},
};

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

    #[graphql(description = "Get a Submission by ID")]
    fn submission(context: &Context, id: SubmissionId) -> FieldResult<schema::QuizSubmission> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_submission::find_by_id(id, &mut conn))?)
    }

    #[graphql(description = "Get all Submissions by Quiz ID")]
    fn submissions(context: &Context, id: QuizId) -> FieldResult<schema::QuizSubmissions> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_submission::get_all(id, &mut conn))?)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_quiz(context: &Context, quiz_input: schema::QuizInput) -> FieldResult<schema::Quiz> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz::new(quiz_input, &mut conn))?)
    }

    fn create_quiz_question(
        context: &Context,
        quiz_question: schema::QuizQuestionInput,
    ) -> FieldResult<schema::QuizQuestion> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_question::new(quiz_question, &mut conn))?)
    }

    fn create_quiz_option(
        context: &Context,
        quiz_option: schema::QuizOptionInput,
    ) -> FieldResult<schema::QuizOption> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_option::new(quiz_option, &mut conn))?)
    }

    fn create_quiz_submission(
        context: &Context,
        quiz_submission: schema::QuizSubmissionInput,
    ) -> FieldResult<schema::QuizSubmission> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_submission::new(
            quiz_submission,
            &mut conn,
        ))?)
    }

    fn create_quiz_answer(
        context: &Context,
        quiz_answer: schema::QuizAnswerInput,
    ) -> FieldResult<schema::QuizAnswer> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_answer::new(quiz_answer, &mut conn))?)
    }

    fn change_quiz_answer(
        context: &Context,
        updated_answer: schema::QuizAnswerUpdate,
    ) -> FieldResult<schema::QuizAnswer> {
        let mut trans = smol::run(context.db_pool.begin())?;
        let answer = smol::run(db::quiz_answer::update(updated_answer, &mut trans))?;
        let _ = smol::run(trans.commit())?;
        Ok(answer)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn new() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
