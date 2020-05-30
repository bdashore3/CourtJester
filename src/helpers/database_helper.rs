use std::env;
use sqlx::postgres::PgPool;
use crate::helpers::credentials_helper;

pub async fn obtain_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let creds = credentials_helper::read_creds(args[1].to_string()).unwrap();

    let connection_string = &creds.db_connection;

    let pool = PgPool::builder()
        .max_size(10)
        .build(&connection_string).await?;
    
    Ok(pool)
}