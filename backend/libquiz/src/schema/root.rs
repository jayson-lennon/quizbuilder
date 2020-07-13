use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
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

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Get a Quiz by ID")]
    fn quiz(context: &Context, quiz_id: QuizId) -> Result<schema::Quiz, FieldError> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz::find_by_id(quiz_id, &mut conn))?)
    }

    #[graphql(description = "Get all Submissions by Quiz ID")]
    fn quiz_from_shortcode(context: &Context, shortcode: String) -> FieldResult<schema::Quiz> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz::from_shortcode(&shortcode, &mut conn))?)
    }

    #[graphql(description = "Get a Submission by ID")]
    fn submission(
        context: &Context,
        submission_id: SubmissionId,
    ) -> FieldResult<schema::QuizSubmission> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_submission::find_by_id(
            submission_id,
            &mut conn,
        ))?)
    }

    #[graphql(description = "Get all Submissions by Quiz ID")]
    fn submissions(context: &Context, quiz_id: QuizId) -> FieldResult<Vec<schema::QuizSubmission>> {
        let mut conn = smol::run(context.db_pool.acquire())?;
        Ok(smol::run(db::quiz_submission::get_all(quiz_id, &mut conn))?)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_quiz_with_questions(
        context: &Context,
        quiz_input: schema::FullQuizInput,
    ) -> FieldResult<schema::Quiz> {
        let mut trans = smol::run(context.db_pool.begin())?;
        let quiz = smol::run(db::quiz::new_with_questions(quiz_input, &mut trans))?;
        let _ = smol::run(trans.commit())?;
        Ok(quiz)
    }

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

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn new() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Context>::new())
}
