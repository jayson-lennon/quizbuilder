use crate::schema::Quiz;
use crate::types::id::QuizId;
use sqlx::postgres::PgPool;

pub async fn find_by_id(id: QuizId, db_pool: &PgPool) -> Result<Quiz, sqlx::Error> {
    let id = id.0;
    let quiz = sqlx::query!(
        r#"SELECT
          quiz_id,
          name
        FROM quizzes WHERE quizzes.quiz_id = $1"#,
        id
    )
    .fetch_one(db_pool)
    .await?;

    Ok(Quiz {
        quiz_id: quiz.quiz_id.into(),
        name: quiz.name,
    })
}
