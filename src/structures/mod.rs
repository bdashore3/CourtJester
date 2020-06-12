use twilight::http::Client as HttpClient;
use std::collections::HashMap;
use sqlx::PgPool;

pub type CommandResult = ::std::result::Result<(), Box<dyn std::error::Error>>;

pub struct Context {
    pub http: HttpClient,
    pub pool: PgPool,
    pub data: HashMap<String, String>
}