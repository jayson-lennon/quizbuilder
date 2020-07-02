use crate::schema::{QuizQuestion, QuizQuestionInput};
use crate::types::id::QuizId;
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn new(
    input: QuizQuestionInput,
    conn: &mut PgConnection,
) -> Result<QuizQuestion, sqlx::Error> {
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        "INSERT INTO quiz_questions (quiz_question_id, quiz_id, question_data, position)
        VALUES ($1, $2, $3, $4)",
        id,
        Uuid::from(input.quiz_id),
        input.question_data,
        input.position
    )
    .execute(conn)
    .await?;

    Ok(QuizQuestion {
        quiz_question_id: id.into(),
        quiz_id: input.quiz_id,
        question_data: input.question_data,
        position: input.position,
    })
}

pub async fn get_all(
    quiz_id: QuizId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizQuestion>, sqlx::Error> {
    Ok(sqlx::query!(
        "SELECT
          quiz_question_id, question_data, position
        FROM quiz_questions WHERE quiz_id = $1",
        Uuid::from(quiz_id)
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|question| QuizQuestion {
        quiz_question_id: question.quiz_question_id.into(),
        quiz_id,
        question_data: question.question_data,
        position: question.position,
    })
    .collect::<Vec<_>>())
}
