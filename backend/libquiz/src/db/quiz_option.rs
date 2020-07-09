use crate::schema::{QuizOption, QuizOptionInput, QuizOptionType};
use crate::types::id::QuestionId;
use sqlx::postgres::PgConnection;
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

pub async fn new(
    input: QuizOptionInput,
    conn: &mut PgConnection,
) -> Result<QuizOption, sqlx::Error> {
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        "INSERT INTO quiz_options
          (quiz_option_id, quiz_question_id, option_data, is_correct, position, option_type)
        VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        Uuid::from(input.quiz_question_id),
        input.option_data,
        input.is_correct,
        input.position,
        i32::try_from(input.option_type).unwrap_or_else(|_| 0)
    )
    .execute(conn)
    .await?;

    Ok(QuizOption {
        quiz_option_id: id.into(),
        quiz_question_id: input.quiz_question_id,
        option_data: input.option_data,
        is_correct: input.is_correct,
        position: input.position,
        option_type: input.option_type.into(),
    })
}

pub async fn get_all(
    question_id: QuestionId,
    conn: &mut PgConnection,
) -> Result<Vec<QuizOption>, sqlx::Error> {
    Ok(sqlx::query!(
        "SELECT
          quiz_option_id, option_data, is_correct, position, option_type
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
        option_type: opt
            .option_type
            .try_into()
            .unwrap_or_else(|_| QuizOptionType::Unknown),
    })
    .collect::<Vec<_>>())
}
