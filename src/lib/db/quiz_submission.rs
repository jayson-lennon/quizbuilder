use crate::schema::{QuizSubmission, QuizSubmissionInput};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn new(
    input: QuizSubmissionInput,
    conn: &mut PgConnection,
) -> Result<QuizSubmission, sqlx::Error> {
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        "INSERT INTO quiz_submissions
          (quiz_submission_id, identity, quiz_id, start_date, finish_date)
        VALUES ($1, $2, $3, $4, $5)",
        id,
        input.identity,
        Uuid::from(input.quiz_id),
        input.start_date,
        input.finish_date
    )
    .execute(conn)
    .await?;

    Ok(QuizSubmission {
        quiz_submission_id: id.into(),
        identity: input.identity,
        quiz_id: input.quiz_id,
        start_date: input.start_date,
        finish_date: input.finish_date,
    })
}
