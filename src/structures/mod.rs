use serenity::{
    client::bridge::{voice::ClientVoiceManager, gateway::ShardManager}, 
    prelude::{Mutex, TypeMapKey}
};
use std::sync::Arc;
use sqlx::PgPool;
use serenity_lavalink::LavalinkClient;

// All command context data structures
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = Arc<String>;
}

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}