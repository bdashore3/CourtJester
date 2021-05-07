use std::sync::atomic::{AtomicBool, Ordering};

use crate::{helpers::start_loops, reactions::reaction_handler, ConnectionPool, PrefixMap};
use lavalink_rs::gateway::LavalinkEventHandler;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Reaction,
        guild::{Guild, GuildUnavailable},
        id::GuildId,
        prelude::Ready,
    },
};

pub struct SerenityHandler {
    pub run_loop: AtomicBool,
}

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        if self.run_loop.load(Ordering::Relaxed) {
            self.run_loop.store(false, Ordering::Relaxed);

            println!("Running guild pruner!");
            if let Err(e) = start_loops::guild_pruner(&ctx).await {
                panic!("Error when pruning guilds! {}", e);
            }

            let pool = ctx
                .data
                .read()
                .await
                .get::<ConnectionPool>()
                .cloned()
                .unwrap();

            println!("Starting starboard deletion loop!");
            tokio::spawn(async move {
                if let Err(e) = start_loops::starboard_removal_loop(&pool).await {
                    panic!("Delete buffer failed to start!: {}", e);
                };
            });

            println!("Starting activity loop!");
            tokio::spawn(async move {
                start_loops::activity_loop(&ctx.shard).await;
            });
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        if is_new {
            sqlx::query!(
                "INSERT INTO guild_info VALUES($1, null) ON CONFLICT DO NOTHING",
                guild.id.0 as i64
            )
            .execute(&pool)
            .await
            .unwrap();
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, _full: Option<Guild>) {
        let (pool, prefixes) = {
            let data = ctx.data.read().await;
            let pool = data.get::<ConnectionPool>().cloned().unwrap();
            let prefixes = data.get::<PrefixMap>().cloned().unwrap();

            (pool, prefixes)
        };

        if let Err(e) = sqlx::query!(
            "DELETE FROM guild_info WHERE guild_id = $1",
            incomplete.id.0 as i64
        )
        .execute(&pool)
        .await
        {
            eprintln!("Error in guild removal! (ID {}): {}", incomplete.id.0, e)
        }

        if prefixes.contains_key(&incomplete.id) {
            prefixes.remove(&incomplete.id);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let _ = reaction_handler::dispatch_reaction(&ctx, &reaction, false).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let _ = reaction_handler::dispatch_reaction(&ctx, &reaction, true).await;
    }
}

pub struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}
