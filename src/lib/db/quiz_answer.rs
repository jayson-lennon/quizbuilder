use crate::schema::{QuizAnswer, QuizAnswerInput};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn new(
    input: QuizAnswerInput,
    conn: &mut PgConnection,
) -> Result<QuizAnswer, sqlx::Error> {
    let _ = sqlx::query!(
        "INSERT INTO quiz_answers
          (quiz_submission_id, quiz_question_id, quiz_option_id)
        VALUES ($1, $2, $3)",
        Uuid::from(input.quiz_submission_id),
        Uuid::from(input.quiz_question_id),
        Uuid::from(input.quiz_option_id)
    )
    .execute(conn)
    .await?;

    Ok(QuizAnswer {
        quiz_submission_id: input.quiz_submission_id,
        quiz_question_id: input.quiz_question_id,
        quiz_option_id: input.quiz_option_id,
    })
}
