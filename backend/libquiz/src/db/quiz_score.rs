use crate::schema::QuizScore;
use crate::types::id::{QuizId, SubmissionId};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn get_all(
    quiz_id: QuizId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizScore>, sqlx::Error> {
    struct TotalCorrect {
        quiz_submission_id: SubmissionId,
        count: i32,
    }

    let question_count = sqlx::query!(
        "SELECT COUNT(*) FROM quiz_questions WHERE quiz_id = $1",
        Uuid::from(quiz_id)
    )
    .fetch_one(&mut *conn)
    .await?
    .count
    .unwrap_or_default() as i32;

    let scores: Vec<TotalCorrect> = sqlx::query!(
        "SELECT quiz_submission_id, COUNT(*)
        FROM quiz_answers ans
        JOIN quiz_options opt ON opt.quiz_option_id = ans.quiz_option_id
        JOIN quiz_questions qst ON qst.quiz_question_id = ans.quiz_question_id
        WHERE qst.quiz_id = $1
        AND opt.is_correct = true
        GROUP BY quiz_submission_id",
        Uuid::from(quiz_id)
    )
    .fetch_all(&mut *conn)
    .await?
    .into_iter()
    .map(|results| TotalCorrect {
        quiz_submission_id: SubmissionId(results.quiz_submission_id),
        count: results.count.unwrap_or_default() as i32,
    })
    .collect();

    Ok(super::quiz_submission::get_all(quiz_id, &mut *conn)
        .await?
        .into_iter()
        .map(|sub| QuizScore {
            total_correct: scores
                .iter()
                .find(|score| score.quiz_submission_id == sub.quiz_submission_id)
                .map(|score| score.count)
                .unwrap_or_default(),
            quiz_id,
            submission: sub,
            total_questions: question_count,
        })
        .collect())
}
