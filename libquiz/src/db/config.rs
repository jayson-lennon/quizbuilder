use sqlx::postgres::PgConnection;

pub mod key {
    pub const SHORTCODE_LEN: &str = "shortcode_len";
    pub const SHORTCODE_CHARS: &str = "shortcode_chars";
}

pub mod default {
    pub const SHORTCODE_LEN: i32 = 8;
    pub const SHORTCODE_CHARS: &str = "abcdefghjkmnprtuwxyABCEFHJKLMNR234679";
}

pub async fn get(key: &str, conn: &mut PgConnection) -> Result<String, sqlx::Error> {
    Ok(
        sqlx::query!("SELECT app_value FROM app_config WHERE app_key = $1", key)
            .fetch_one(conn)
            .await?
            .app_value,
    )
}
