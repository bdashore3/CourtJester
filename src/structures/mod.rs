use serenity::{
    client::bridge::gateway::ShardManager, 
    prelude::{Mutex, TypeMapKey}
};
use std::sync::Arc;
use sqlx::PgPool;

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