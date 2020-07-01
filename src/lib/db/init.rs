use sqlx::postgres::PgPool;

pub async fn new_pool(url: &str, size: u32) -> Result<PgPool, sqlx::error::Error> {
    let db_pool = PgPool::builder().max_size(size).build(url).await?;
    Ok(db_pool)
}
