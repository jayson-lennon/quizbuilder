use crate::schema::{QuizQuestion, QuizQuestionInput};
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
