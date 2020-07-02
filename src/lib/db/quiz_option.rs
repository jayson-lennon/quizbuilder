use crate::schema::{QuizOption, QuizOptionInput};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn new(
    input: QuizOptionInput,
    conn: &mut PgConnection,
) -> Result<QuizOption, sqlx::Error> {
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        "INSERT INTO quiz_options
          (quiz_option_id, quiz_question_id, option_data, is_correct, position)
        VALUES ($1, $2, $3, $4, $5)",
        id,
        Uuid::from(input.quiz_question_id),
        input.option_data,
        input.is_correct,
        input.position
    )
    .execute(conn)
    .await?;

    Ok(QuizOption {
        quiz_option_id: id.into(),
        quiz_question_id: input.quiz_question_id,
        option_data: input.option_data,
        is_correct: input.is_correct,
        position: input.position,
    })
}
