use serenity::{
    client::Context, 
    framework::standard::{
        Args, 
        macros::command, 
        CommandResult
    }, 
    model::{
        id::{ChannelId, GuildId},
        channel::{ReactionType, Message}
    },
    builder::CreateEmbed
};
use crate::{
    helpers::voice_utils, 
    structures::{Lavalink, VoiceManager, VoiceTimerMap}
};
use std::{time::Duration, sync::Arc};
use rust_clock::Clock;
use tokio::time::delay_for;
use serenity_lavalink::nodes::Node;

#[command]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let guild_id = msg.guild_id.unwrap();

    let voice_channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let voice_channel = match voice_channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
            return Ok(())
        }
    };

    if args.len() < 1 {
        msg.channel_id.say(ctx, "Please enter a track URL after the command!").await?;
        return Ok(())
    }

    let query = args.message().to_string();

    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().unwrap();

    {
        let mut manager = manager_lock.lock().await;

        if manager.get(&guild_id).is_none() {
            manager.join(&guild_id, voice_channel);
        }
    }

    loop {
        let mut manager = manager_lock.lock().await;

        let handler = manager
            .get_mut(&guild_id)
            .ok_or("Unable to get voice handler.")?;

        if handler.token.is_some() && handler.session_id.is_some() && handler.endpoint.is_some() {
            break;
        }

        delay_for(Duration::from_millis(500)).await;
    }

    let data = ctx.data.read().await;
    let mut manager = manager_lock.lock().await;
    
    if let Some(handler) = manager.get_mut(guild_id) {
        let lava_lock = data.get::<Lavalink>().unwrap();
        let mut lava_client = lava_lock.write().await;

        if lava_client.nodes.get(&guild_id).is_none() {
            Node::new(&mut lava_client, guild_id, msg.channel_id);
        }

        let query_info = lava_client.auto_search_tracks(&query).await?;

        if query_info.tracks.is_empty() {
            msg.channel_id.say(ctx, "Couldn't find the video!").await?;
            return Ok(())
        }

        {
            let node = lava_client.nodes.get_mut(&guild_id).unwrap();

            node.play(query_info.tracks[0].clone())
                .queue();
        }
        let node = lava_client.nodes.get(&guild_id).unwrap();

        if !lava_client.loops.contains(&guild_id) {
            node.start_loop(Arc::clone(lava_lock), Arc::new(handler.clone())).await;
        }

        let mut cl = Clock::new();
        cl.set_time_ms(query_info.tracks[0].info.length as i64);

        msg.channel_id.send_message(ctx, |m| {
            m.content("Added to queue:");
            m.embed(|e| {
                e.color(0x98fb98);
                e.title(&query_info.tracks[0].info.title);
                e.url(&query_info.tracks[0].info.uri);
                e.field("Uploader", &query_info.tracks[0].info.author, true);
                e.field("Length", cl.get_time(), true);
                e.footer(|f| {
                    f.text(format!("Requested by {}", msg.author.name));
                    f
                })
            })
        }).await?;

        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            queue_checker(ctx_clone, guild_id).await;
        });
    }
    
    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    if let Some(node) = lava_client.nodes.get_mut(&guild_id) {
        node.pause(&lava_client_read, &guild_id).await?;
        msg.react(ctx, ReactionType::Unicode(String::from("⏸"))).await?;
    
        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            voice_utils::create_new_timer(ctx_clone, guild_id).await;
        });
    };

    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;

    let lava_lock = data.get::<Lavalink>().unwrap();
    let mut lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    match lava_client.nodes.get_mut(&guild_id) {
        Some(node) => {
            node.stop(&mut lava_client_read, &guild_id).await?;
            msg.react(ctx, ReactionType::Unicode(String::from("🛑"))).await?;
        
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                voice_utils::create_new_timer(ctx_clone, guild_id).await;
            });   
        },
        None => {
            msg.channel_id.say(ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
        }
    };

    Ok(())
}

#[command]
#[aliases("unpause")]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    let node = match lava_client.nodes.get_mut(&guild_id) {
        Some(node) => node,
        None => {
            msg.channel_id.say(ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
            return Ok(())
        }
    };

    let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();

    if let Some(future_guard) = voice_timer_map.get(&guild_id) {
        future_guard.value().abort();
    }

    node.resume(&lava_client_read, &msg.guild_id.unwrap()).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("▶"))).await?;

    Ok(())
}

#[command]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().read().await;
    let node = match lava_client.nodes.get(&msg.guild_id.unwrap()) {
        Some(node) => node,
        None => {
            return Ok(())
        }
    };
    let queue = &node.queue;

    if queue.is_empty() && node.now_playing.is_none() {
        msg.channel_id.say(ctx, "The queue is currently empty!").await?;
        return Ok(())
    }

    let mut eb = CreateEmbed::default();
    eb.color(0x0377fc);
    eb.title(format!("Queue for {}", guild_id.name(ctx).await.unwrap()));

    if let Some(t) = node.now_playing.as_ref() {
        let mut cl = Clock::new();
        cl.set_time_ms(t.track.info.length as i64);
        eb.field("Now Playing", format!("[{}]({}) | `{}`", t.track.info.title, t.track.info.uri, cl.get_time()), false);
    }

    if queue.len() > 0 {
        let mut queue_string = String::new();

        for i in queue {
            let mut cl = Clock::new();
            cl.set_time_ms(i.track.info.length as i64);
            queue_string.push_str(&format!("[{}]({}) | `{}` \n\n", i.track.info.title, i.track.info.uri, cl.get_time()));
        }

        eb.field("Next Songs", queue_string, false);
    }

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.0 = eb.0;
            e
        })
    }).await?;

    Ok(())
}

async fn queue_checker(ctx: Context, guild_id: GuildId) {
    loop {
        delay_for(Duration::from_secs(60)).await;
        {
            let data = ctx.data.read().await;
            let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();

            if voice_timer_map.get(&guild_id).is_some() {
                return
            }

            let lava_lock = data.get::<Lavalink>().unwrap();
            let lava_client = lava_lock.read().await;
            let node = match lava_client.nodes.get(&guild_id) {
                Some(node) => node,
                None => {
                    return
                }

            };
            
            if node.queue.is_empty() && node.now_playing.is_none() {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    voice_utils::create_new_timer(ctx_clone, guild_id).await;
                });
                break;
            }
        }
    }
}

#[command]
#[aliases("c")]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let mut lava_client = data.get::<Lavalink>().unwrap().write().await;
    let node = match lava_client.nodes.get_mut(&msg.guild_id.unwrap()) {
        Some(node) => node,
        None => {
            msg.channel_id.say(ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
            return Ok(())
        }
    };
    node.queue.clear();

    msg.react(ctx, ReactionType::Unicode(String::from("💣"))).await?;

    Ok(())
}

#[command]
#[aliases("s")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "You're not in a voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let mut lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    let node = match lava_client.nodes.get_mut(&guild_id) {
        Some(node) => node,
        None => {
            msg.channel_id.say(ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
            return Ok(())
        }
    };
    
    if node.queue.is_empty() {
        node.stop(&mut lava_client_read, &guild_id).await?;
    } else {
        node.skip();
    }

    msg.channel_id.say(ctx, "Skipping the current track...").await?;

    Ok(())
}

pub async fn music_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("play <URL or search keywords> : Plays the specified track \n\n");
    content.push_str("pause: Pauses the current track \n\n");
    content.push_str("resume <author> <text>: Resumes the current track \nAlias: unpause \n\n");
    content.push_str("stop: Stops the current track and empties the queue. Doesn't disconnect the bot \n\n");
    content.push_str("skip: Skips the current track. If there are no tracks in the queue, the player is stopped \n\n");
    content.push_str("queue: See the current queue for the guild and what's playing");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Music Help");
            e.description("Description: Commands for playing music");
            e.field("Commands", content, false);
            e.footer(|f| {
                f.text("For more information on voice commands, please check voice help");
                f
            });
            e
        })
    }).await;
}