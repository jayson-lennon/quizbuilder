use crate::schema::{FullQuizInput, Quiz, QuizInput, QuizOptionInput, QuizQuestionInput};
use crate::types::id::QuizId;
use crate::types::time::Duration;
use chrono::Utc;
use sqlx::postgres::PgConnection;
use uuid::Uuid;

pub async fn find_by_id(id: QuizId, conn: &mut PgConnection) -> Result<Quiz, sqlx::Error> {
    let quiz = sqlx::query!(
        r#"SELECT
            quiz_id,
            owner,
            name,
            date_created,
            open_date,
            close_date,
            duration_sec,
            shortcode
        FROM quizzes WHERE quizzes.quiz_id = $1"#,
        Uuid::from(id)
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(Quiz {
        quiz_id: id,
        name: quiz.name,
        owner: quiz.owner.into(),
        date_created: quiz.date_created,
        open_date: quiz.open_date,
        close_date: quiz.close_date,
        duration: {
            match quiz.duration_sec {
                Some(d) => Some(Duration(chrono::Duration::seconds(d as i64))),
                None => None,
            }
        },
        shortcode: quiz.shortcode,
        questions: super::quiz_question::get_all(id, conn).await?,
    })
}

pub async fn new(input: QuizInput, conn: &mut PgConnection) -> Result<Quiz, sqlx::Error> {
    let id = Uuid::new_v4();
    let shortcode = gen_shortcode(&ShortCodeOptions::default());
    let date_created = Utc::now();
    let _ = sqlx::query!(
        "INSERT INTO quizzes
          (quiz_id, owner, name, date_created, open_date, close_date, duration_sec, shortcode)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        id,
        input.owner.0,
        input.name,
        date_created,
        input.open_date,
        input.close_date,
        input.duration.map(|d| d.0.num_seconds() as i32),
        &shortcode,
    )
    .execute(conn)
    .await?;
    Ok(Quiz {
        quiz_id: id.into(),
        name: input.name,
        owner: input.owner,
        date_created,
        open_date: input.open_date,
        close_date: input.close_date,
        duration: input.duration,
        shortcode,
        questions: vec![],
    })
}

pub async fn new_with_questions(
    input: FullQuizInput,
    conn: &mut PgConnection,
) -> Result<Quiz, sqlx::Error> {
    let new_quiz_input = QuizInput {
        name: input.name,
        owner: input.owner,
        open_date: input.open_date,
        close_date: input.close_date,
        duration: input.duration,
    };

    let quiz = new(new_quiz_input, &mut *conn).await?;

    for question in input.questions.into_iter() {
        let input = QuizQuestionInput {
            quiz_id: quiz.quiz_id,
            question_data: question.question_data,
            position: question.position,
        };

        let question_id = {
            super::quiz_question::new(input, &mut *conn)
                .await?
                .quiz_question_id
        };

        for option in question.options.into_iter() {
            let input = QuizOptionInput {
                quiz_question_id: question_id,
                option_data: option.option_data,
                is_correct: option.is_correct,
                position: option.position,
                option_type: option.option_type,
            };

            super::quiz_option::new(input, &mut *conn).await?;
        }
    }

    let questions = super::quiz_question::get_all(quiz.quiz_id, conn).await?;

    Ok(Quiz {
        quiz_id: quiz.quiz_id,
        name: quiz.name,
        owner: quiz.owner,
        date_created: quiz.date_created,
        open_date: quiz.open_date,
        close_date: quiz.close_date,
        duration: quiz.duration,
        shortcode: quiz.shortcode,
        questions,
    })
}

pub struct ShortCodeOptions {
    len: i32,
    allowed_chars: String,
}

impl Default for ShortCodeOptions {
    fn default() -> Self {
        ShortCodeOptions {
            len: 8,
            allowed_chars: crate::db::config::default::SHORTCODE_CHARS.to_owned(),
        }
    }
}

fn gen_shortcode(options: &ShortCodeOptions) -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let chars = options.allowed_chars.as_bytes();
    let len = options.len;
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, chars.len());
            chars[idx] as char
        })
        .collect()
}

async fn get_shortcode_options(conn: &mut PgConnection) -> Result<ShortCodeOptions, sqlx::Error> {
    use super::config::key::{SHORTCODE_CHARS, SHORTCODE_LEN};

    let len = {
        let len = super::config::get(SHORTCODE_LEN, conn).await?;
        i32::from_str_radix(&len, 10).unwrap_or_else(|_| super::config::default::SHORTCODE_LEN)
    };

    let shortcode_string = super::config::get(SHORTCODE_CHARS, conn).await?;

    Ok(ShortCodeOptions {
        len,
        allowed_chars: shortcode_string,
    })
}

pub async fn from_shortcode(shortcode: &str, conn: &mut PgConnection) -> Result<Quiz, sqlx::Error> {
    let id = sqlx::query!(
        "SELECT quiz_id FROM quizzes WHERE shortcode = $1",
        shortcode
    )
    .fetch_one(&mut *conn)
    .await?;

    let id = id.quiz_id.into();
    find_by_id(id, conn).await
}

pub fn validate_shortcode(shortcode: &str, options: &ShortCodeOptions) -> bool {
    for ch in shortcode.chars() {
        if !options.allowed_chars.chars().any(|c| c == ch) {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod test {
    use crate::test::util;

    use crate::db::quiz;

    #[test]
    fn gens_shortcode() {
        let options = quiz::ShortCodeOptions {
            len: 5,
            allowed_chars: "a".to_owned(),
        };
        let shortcode = quiz::gen_shortcode(&options);
        assert_eq!(&shortcode, "aaaaa");
    }
}
