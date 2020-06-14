use twilight::http::Client as HttpClient;
use twilight::model::gateway::payload::MessageCreate;
use twilight::cache::InMemoryCache;
use std::collections::HashMap;
use sqlx::PgPool;

pub type CommandResult = ::std::result::Result<(), Box<dyn std::error::Error>>;

pub struct Context<'a> {
    pub http: &'a HttpClient,
    pub pool: &'a PgPool,
    pub data: &'a HashMap<String, String>,
    pub cache: &'a InMemoryCache
}

impl <'a> Context<'a> {
    pub fn new(
        http: &'a HttpClient,
        pool: &'a PgPool,
        data: &'a HashMap<String, String>,
        cache: &'a InMemoryCache
    ) -> Self {
        Context {
            http,
            pool,
            data,
            cache
        }
    }
}