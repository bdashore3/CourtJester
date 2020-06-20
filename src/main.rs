mod handlers;
mod helpers;
mod structures;
mod commands;

use twilight::{
    gateway::{Cluster, ClusterConfig, Event},
    http::Client as HttpClient,
    cache::{
        twilight_cache_inmemory::config::{InMemoryConfigBuilder, EventType},
        InMemoryCache
    }, 
    model::id::GuildId
};
use futures::StreamExt;
use std::{env, error::Error};
use helpers::{
    database_helper,
    credentials_helper
};
use handlers::command_handler;
use structures::Context;
use std::collections::{
    HashSet,
    HashMap
};
use tokio::sync::RwLock;
use crate::commands::config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let mut data: HashMap<String, String> = HashMap::new();
    let creds = credentials_helper::read_creds(args[1].to_string()).unwrap();

    let http = HttpClient::new(&creds.bot_token);

    let cache_config = InMemoryConfigBuilder::new()
        .event_types(
        EventType::MESSAGE_CREATE
            | EventType::GUILD_CREATE
        )
        .build();
    let cache = InMemoryCache::from(cache_config);

    let pool = database_helper::obtain_pool(creds.db_connection).await?;
    data.insert("default_prefix".to_string(), creds.default_prefix);

    let guild_set: HashSet<GuildId> = HashSet::new();
    let guild_rwlock = RwLock::new(guild_set);

    let cluster_config = ClusterConfig::builder(&creds.bot_token).build();
    let cluster = Cluster::new(cluster_config);
    cluster.up().await?;

    let mut events = cluster.events().await;

    let ctx = Context::new(&http, &pool, &data, &guild_rwlock, &cache);

    while let Some(event) = events.next().await {
        cache.update(&event.1).await.expect("Error in caching data!");

        match handle_event(event, &ctx).await {
            Ok(_) => (),
            Err(why) => println!("{}", why),
        };
    }

    Ok(())
}

async fn handle_event(event: (u64, Event), ctx: &Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        (id, Event::Ready(info)) => {
            let mut guild_set = ctx.guild_set.write().await;
            guild_set.extend(info.guilds.keys());
            println!("Guild Set on ready: {:?}", guild_set.clone());
            println!("Connected on shard {}", id);
        },
        (_, Event::MessageCreate(msg)) => {
            if msg.author.bot {
                return Ok(())
            }

            let default_prefix = ctx.data.get("default_prefix").unwrap().to_string();
            let prefix = get_prefix(&ctx.pool, msg.guild_id.unwrap().0 as i64, default_prefix).await.unwrap();

            if &msg.0.content[..prefix.len()] == prefix.as_str() {
                match command_handler::handle_command(&msg.0, ctx, prefix.len()).await {
                    Ok(()) => {},
                    Err(error) => println!("Command Error!: {}", error)
                };
            }
        },
        (_, Event::GuildCreate(guild)) => {
            let guild_set = ctx.guild_set.read().await;
            println!("guild set on call: {:?}", guild_set.clone());
            if !guild_set.contains(&guild.id) {
                sqlx::query!("INSERT INTO guild_info VALUES($1, null) ON CONFLICT DO NOTHING", guild.id.0 as i64)
                    .execute(ctx.pool).await.unwrap();
            }
        },
        (_, Event::GuildDelete(guild)) => {
            sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild.id.0 as i64)
                .execute(ctx.pool).await.unwrap();
            
            let mut guild_set = ctx.guild_set.write().await;

            guild_set.remove(&guild.id);
        }
        _ => {}
    }

    Ok(())
}
