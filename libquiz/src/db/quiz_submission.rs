use crate::schema::{QuizSubmission, QuizSubmissionInput};
use crate::types::id::{QuizId, SubmissionId};
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
        answers: vec![],
    })
}

pub async fn find_by_id(
    id: SubmissionId,
    conn: &mut PgConnection,
) -> Result<QuizSubmission, sqlx::Error> {
    let submission = sqlx::query!(
        "SELECT
          identity, quiz_id, start_date, finish_date
        FROM quiz_submissions WHERE quiz_submission_id = $1",
        Uuid::from(id)
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(QuizSubmission {
        quiz_submission_id: id,
        identity: submission.identity,
        quiz_id: submission.quiz_id.into(),
        start_date: submission.start_date,
        finish_date: submission.finish_date,
        answers: super::quiz_answer::get_all(id, conn).await?,
    })
}

pub async fn get_all(
    quiz_id: QuizId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizSubmission>, sqlx::Error> {
    let submissions = sqlx::query!(
        "SELECT
          identity, quiz_submission_id, start_date, finish_date
        FROM quiz_submissions WHERE quiz_id = $1",
        Uuid::from(quiz_id)
    )
    .fetch_all(&mut *conn)
    .await?;

    let mut mapped_submissions = vec![];

    for sub in submissions.into_iter() {
        let submission_id = sub.quiz_submission_id.into();
        let sub = QuizSubmission {
            quiz_submission_id: submission_id,
            identity: sub.identity,
            quiz_id: quiz_id,
            start_date: sub.start_date,
            finish_date: sub.finish_date,
            answers: super::quiz_answer::get_all(submission_id, conn).await?,
        };
        mapped_submissions.push(sub);
    }

    Ok(mapped_submissions)
}
