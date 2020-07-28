use serenity::{
    client::bridge::{
        voice::ClientVoiceManager, 
        gateway::ShardManager
    }, 
    prelude::{Mutex, TypeMapKey, RwLock}, 
    model::id::GuildId
};
use std::{collections::HashMap, sync::Arc};
use sqlx::PgPool;
use serenity_lavalink::LavalinkClient;
use dashmap::DashMap;
use futures::future::AbortHandle;
use reqwest::Client as Reqwest;

// All command context data structures
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = Arc<RwLock<LavalinkClient>>;
}

pub struct VoiceTimerMap;

impl TypeMapKey for VoiceTimerMap {
    type Value = Arc<DashMap<GuildId, AbortHandle>>;
}

pub struct CommandNameMap;

impl TypeMapKey for CommandNameMap {
    type Value = Arc<Vec<String>>;
}

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Reqwest>;
}

pub struct PubCreds;

impl TypeMapKey for PubCreds {
    type Value = Arc<HashMap<String, String>>;
}