use sqlx::postgres::PgPool;

pub async fn obtain_pool(db_connection: String) -> Result<PgPool, Box<dyn std::error::Error>> {

    let connection_string = &db_connection;

    let pool = PgPool::builder()
        .max_size(10)
        .build(&connection_string).await?;
    
    Ok(pool)
}