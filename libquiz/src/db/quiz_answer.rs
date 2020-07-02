use crate::schema::{QuizAnswer, QuizAnswerInput, QuizAnswerUpdate};
use crate::types::id::SubmissionId;
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

pub async fn get_all(
    submission_id: SubmissionId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizAnswer>, sqlx::Error> {
    Ok(sqlx::query!(
        "SELECT
          quiz_question_id, quiz_option_id
        FROM quiz_answers WHERE quiz_submission_id = $1",
        Uuid::from(submission_id)
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|answer| QuizAnswer {
        quiz_submission_id: submission_id,
        quiz_question_id: answer.quiz_question_id.into(),
        quiz_option_id: answer.quiz_option_id.into(),
    })
    .collect::<Vec<_>>())
}
