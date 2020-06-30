use twilight::http::Client as HttpClient;
use twilight::{model::id::GuildId, cache::InMemoryCache};
use twilight::standby::Standby;
use std::{sync::Arc, collections::{
    HashMap,
    HashSet,
}, error::Error};
use sqlx::PgPool;
use tokio::sync::RwLock;

pub(crate) type CommandResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone)]
pub struct Context {
    pub http: HttpClient,
    pub pool: Arc<PgPool>,
    pub data: HashMap<String, String>,
    pub guild_set: Arc<RwLock<HashSet<GuildId>>>,
    pub cache: Arc<InMemoryCache>,
    pub standby: Standby
}

impl Context {
    pub fn new(
        http: HttpClient,
        pool: Arc<PgPool>,
        data: HashMap<String, String>,
        guild_set: Arc<RwLock<HashSet<GuildId>>>,
        cache: Arc<InMemoryCache>,
        standby: Standby
    ) -> Self {
        Context {
            http,
            pool,
            data,
            guild_set,
            cache,
            standby
        }
    }
}