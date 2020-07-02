use crate::schema::{QuizAnswer, QuizAnswerInput, QuizAnswerUpdate};
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

pub async fn update(
    input: QuizAnswerUpdate,
    conn: &mut PgConnection,
) -> Result<QuizAnswer, sqlx::Error> {
    let deleted = sqlx::query!(
        "DELETE FROM quiz_answers
          WHERE quiz_submission_id = $1 AND
                quiz_question_id = $2 AND
                quiz_option_id = $3",
        Uuid::from(input.quiz_submission_id),
        Uuid::from(input.quiz_question_id),
        Uuid::from(input.old_quiz_option_id),
    )
    .execute(&mut *conn)
    .await?;

    if deleted == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    let _ = sqlx::query!(
        "INSERT INTO quiz_answers
          (quiz_submission_id, quiz_question_id, quiz_option_id)
        VALUES ($1, $2, $3)

        ON CONFLICT (quiz_submission_id, quiz_question_id, quiz_option_id)
          DO NOTHING",
        Uuid::from(input.quiz_submission_id),
        Uuid::from(input.quiz_question_id),
        Uuid::from(input.new_quiz_option_id),
    )
    .execute(conn)
    .await?;

    Ok(QuizAnswer {
        quiz_submission_id: input.quiz_submission_id,
        quiz_question_id: input.quiz_question_id,
        quiz_option_id: input.new_quiz_option_id,
    })
}
