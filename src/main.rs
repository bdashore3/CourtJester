mod handlers;
mod helpers;
mod structures;
mod commands;
mod reactions;

use twilight::{
    gateway::{
        Cluster, 
        ClusterConfig, 
        Event, 
        cluster::config::ShardScheme
    },
    http::Client as HttpClient,
    cache::{
        twilight_cache_inmemory::config::{InMemoryConfigBuilder, EventType},
        InMemoryCache
    }, 
    model::{gateway::GatewayIntents, id::GuildId},
    standby::Standby,
};
use futures::StreamExt;
use std::{env, error::Error};
use helpers::{
    database_helper,
    credentials_helper
};
use handlers::{
    command_handler,
    reaction_handler
};
use structures::Context;
use std::{sync::Arc, collections::{
    HashSet,
    HashMap
}};
use tokio::sync::RwLock;
use crate::commands::config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut data: HashMap<String, String> = HashMap::new();
    let creds = credentials_helper::read_creds(args[1].to_string()).unwrap();

    let pool = database_helper::obtain_pool(creds.db_connection).await?;
    data.insert("default_prefix".to_string(), creds.default_prefix);

    let guild_set: HashSet<GuildId> = HashSet::new();
    let guild_rwlock = Arc::new(RwLock::new(guild_set));

    let scheme = ShardScheme::Auto;

    let config = ClusterConfig::builder(&creds.bot_token)
        .shard_scheme(scheme)
        .intents(Some(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS,
        ))
        .build();

    // Start up the cluster
    let cluster = Cluster::new(config).await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = HttpClient::new(&creds.bot_token);
    let cache_config = InMemoryConfigBuilder::new()
        .event_types(
            EventType::MESSAGE_CREATE
                | EventType::GUILD_CREATE,
        )
        .build();
    let cache = InMemoryCache::from(cache_config);
    let standby = Standby::new();

    let mut events = cluster.events().await;

    let ctx = Context::new(http, Arc::new(pool), data, guild_rwlock, Arc::new(cache), standby);

    while let Some(event) = events.next().await {
        ctx.cache.as_ref().update(&event.1).await.expect("Error in caching data!");
        ctx.standby.process(&event.1);
        tokio::spawn(handle_event(event, ctx.clone()));
    }

    Ok(())
}

async fn handle_event(event: (u64, Event), ctx: Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        (id, Event::Ready(info)) => {
            let mut guild_set = ctx.guild_set.write().await;
            guild_set.extend(info.guilds.keys());
            println!("Connected to Discord on shard {}", id);
        },
        (_, Event::MessageCreate(msg)) => {
            if msg.author.bot {
                return Ok(())
            }

            let default_prefix = ctx.data.get("default_prefix").unwrap().to_string();
            let prefix = get_prefix(&ctx.pool, msg.guild_id.unwrap().0 as i64, default_prefix).await.unwrap();

            if &msg.0.content[..prefix.len()] == prefix.as_str() {
                match command_handler::handle_command(&msg.0, &ctx, prefix.len()).await {
                    Ok(()) => {},
                    Err(error) => println!("Command Error!: {}", error)
                };
            }
        },
        (_, Event::GuildCreate(guild)) => {
            let guild_set = ctx.guild_set.read().await;
            if !guild_set.contains(&guild.id) {
                sqlx::query!("INSERT INTO guild_info VALUES($1, null) ON CONFLICT DO NOTHING", guild.id.0 as i64)
                    .execute(ctx.pool.as_ref()).await.unwrap();
            }
        },
        (_, Event::GuildDelete(guild)) => {
            sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild.id.0 as i64)
                .execute(ctx.pool.as_ref()).await.unwrap();
            
            let mut guild_set = ctx.guild_set.write().await;

            guild_set.remove(&guild.id);
        },
        (_, Event::ReactionAdd(reaction)) => {
            match reaction_handler::dispatch_reaction(&ctx, &reaction.0, false).await {
                Ok(()) => {},
                Err(error) => println!("Reaction Error!: {}", error)
            };
        },
        (_, Event::ReactionRemove(reaction)) => {
            match reaction_handler::dispatch_reaction(&ctx, &reaction.0, true).await {
                Ok(()) => {},
                Err(error) => println!("Reaction Error: {}", error)
            };
        }
        _ => {}
    }

    Ok(())
}
