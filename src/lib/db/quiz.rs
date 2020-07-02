use crate::schema::Quiz;
use crate::types::id::QuizId;
use sqlx::postgres::PgConnection;
use std::borrow::BorrowMut;

pub async fn find_by_id(id: QuizId, conn: &mut PgConnection) -> Result<Quiz, sqlx::Error> {
    let id = id.0;
    let quiz = sqlx::query!(
        r#"SELECT
          quiz_id,
          name
        FROM quizzes WHERE quizzes.quiz_id = $1"#,
        id
    )
    .fetch_one(conn)
    .await?;

    Ok(Quiz {
        quiz_id: quiz.quiz_id.into(),
        name: quiz.name,
    })
}

struct ShortCodeOptions {
    len: i32,
    allowed_chars: String,
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

pub async fn save_shortcode(shortcode: &str, conn: &mut PgConnection) -> Result<(), sqlx::Error> {
    // This will be an error if the shortcode already exists, so no need to
    // return a value.
    let _ = sqlx::query!(
        "INSERT INTO quiz_shortcodes (shortcode) VALUES ($1)",
        shortcode
    )
    .execute(conn)
    .await?;
    Ok(())
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

    #[test]
    fn saves_shortcode() {
        use uuid::Uuid;

        let mut conn = util::new_connection();
        let shortcode = Uuid::new_v4().to_string();
        let added = smol::run(quiz::save_shortcode(&shortcode, &mut conn));
        assert!(added.is_ok());

        let added = smol::run(quiz::save_shortcode(&shortcode, &mut conn));
        assert!(added.is_err());
    }
}
