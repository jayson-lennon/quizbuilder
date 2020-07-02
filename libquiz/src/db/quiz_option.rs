use crate::schema::{QuizOption, QuizOptionInput};
use crate::types::id::QuestionId;
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

pub async fn get_all(
    question_id: QuestionId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizOption>, sqlx::Error> {
    Ok(sqlx::query!(
        "SELECT
          quiz_option_id, option_data, is_correct, position
        FROM quiz_options WHERE quiz_question_id = $1",
        Uuid::from(question_id)
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|opt| QuizOption {
        quiz_option_id: opt.quiz_option_id.into(),
        quiz_question_id: question_id,
        option_data: opt.option_data,
        is_correct: opt.is_correct,
        position: opt.position,
    })
    .collect::<Vec<_>>())
}
