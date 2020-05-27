use sqlx::postgres::PgPool;

pub async fn obtain_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let connection_string = "postgres://postgres:24bd2001@localhost:5432/CourtJester";

    let pool = PgPool::builder()
        .max_size(10)
        .build(&connection_string).await?;
    
    Ok(pool)
}