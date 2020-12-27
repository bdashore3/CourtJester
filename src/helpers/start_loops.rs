use rand::{Rng, SeedableRng, prelude::StdRng};
use serenity::{client::bridge::gateway::ShardMessenger, framework::standard::CommandResult, model::{id::GuildId, prelude::Activity}, prelude::*};
use sqlx::PgPool;
use crate::ConnectionPool;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::time::delay_for;


pub async fn starboard_removal_loop(pool: &PgPool) -> CommandResult {
    loop {    
        let delete_data = sqlx::query!("SELECT guild_id, reaction_message_id, sent_message_id, delete_time FROM starboard")
            .fetch_all(pool).await?;
        
        for i in delete_data {
            println!("Checking delete status on starboard message {}", i.guild_id);

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards?")
                .as_secs() as i64;

            if i.delete_time <= current_time {
                println!("Deleting guild {} from the database \n", i.guild_id);
                sqlx::query!("DELETE FROM starboard WHERE guild_id = $1 AND reaction_message_id = $2 AND sent_message_id = $3",
                        i.guild_id, i.reaction_message_id, i.sent_message_id)
                    .execute(pool).await?;
            } else {
                println!("Entry's time isn't greater than a week! Not deleting guild {}! \n", i.guild_id);
            }
        }
    
        delay_for(Duration::from_secs(345600)).await;
    }
}

pub async fn guild_pruner(ctx: &Context) -> CommandResult {
    let pool = ctx.data.read().await
        .get::<ConnectionPool>().cloned().unwrap();

    let guilds = ctx.cache.guilds().await;

    let guild_data = sqlx::query!("SELECT guild_id FROM guild_info")
        .fetch_all(&pool).await?;

    println!(" ");

    for guild in guild_data {
        if !guilds.contains(&GuildId::from(guild.guild_id as u64)) {
            println!("Removing guild: {}", guild.guild_id);

            sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild.guild_id)
                .execute(&pool).await?;
        }
    }

    println!(" ");

    Ok(())
}

pub async fn activity_loop(messenger: &ShardMessenger) {
    let activity_vec = vec![
        Activity::playing("as the fool"),
        Activity::listening("a tune!"),
        Activity::listening("straight vibes"),
        Activity::playing("vibe checks"),
        Activity::playing("hacking the mainframe"),
        Activity::playing("boof simulator 2021"),
        Activity::playing("ZA WARUDO!"),
        Activity::listening("lofi music"),
        Activity::playing("Minecraft"),
        Activity::listening("Bhai tunes"),
        Activity::playing("Purging scalpers"),
        Activity::listening("the rustdoc audiobook")
    ];

    let mut rng = StdRng::from_entropy();

    loop {
        let val = rng.gen_range(0..=activity_vec.len() - 1);

        messenger.set_activity(Some(activity_vec[val].to_owned()));

        delay_for(Duration::from_secs(7200)).await;
    }
}
