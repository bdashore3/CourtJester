use twilight::http::Client as HttpClient;
use twilight::{model::id::GuildId, cache::InMemoryCache};
use std::collections::{
    HashMap,
    HashSet,
};
use sqlx::PgPool;
use tokio::sync::RwLock;

pub type CommandResult = ::std::result::Result<(), Box<dyn std::error::Error>>;

pub struct Context<'a> {
    pub http: &'a HttpClient,
    pub pool: &'a PgPool,
    pub data: &'a HashMap<String, String>,
    pub guild_set: &'a RwLock<HashSet<GuildId>>,
    pub cache: &'a InMemoryCache
}

impl <'a> Context<'a> {
    pub fn new(
        http: &'a HttpClient,
        pool: &'a PgPool,
        data: &'a HashMap<String, String>,
        guild_set: &'a RwLock<HashSet<GuildId>>,
        cache: &'a InMemoryCache
    ) -> Self {
        Context {
            http,
            pool,
            data,
            guild_set,
            cache
        }
    }
}