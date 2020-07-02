use crate::schema::{QuizQuestion, QuizQuestionInput};
use crate::types::id::{QuestionId, QuizId};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn new(
    question: QuizQuestionInput,
    conn: &mut PgConnection,
) -> Result<QuizQuestion, sqlx::Error> {
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        "INSERT INTO quiz_questions (quiz_question_id, quiz_id, question_data, position)
        VALUES ($1, $2, $3, $4)",
        id,
        Uuid::from(question.quiz_id),
        question.question_data,
        question.position
    )
    .execute(conn)
    .await?;

    Ok(QuizQuestion {
        quiz_question_id: id.into(),
        quiz_id: question.quiz_id,
        question_data: question.question_data,
        position: question.position,
    })
}

#[cfg(test)]
pub mod test {
    use crate::test::util;

    use crate::db::quiz;

    #[test]
    fn saves_quiz_question() {
        //
        //        use uuid::Uuid;
        //
        //        let mut conn = util::new_connection();
        //        let shortcode = Uuid::new_v4().to_string();
        //        let added = smol::run(quiz::save_shortcode(&shortcode, &mut conn));
        //        assert!(added.is_ok());
        //
        //        let added = smol::run(quiz::save_shortcode(&shortcode, &mut conn));
        //        assert!(added.is_err());
    }
}
