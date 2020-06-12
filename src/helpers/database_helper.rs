use sqlx::PgPool;
use std::error::Error;

pub async fn obtain_pool(db_connection: String) -> Result<PgPool, Box<dyn Error + Send + Sync>> {

    let connection_string = &db_connection;

    let pool = PgPool::builder()
        .max_size(10)
        .build(&connection_string).await?;
    
    Ok(pool)
}