use lavalink_rs::LavalinkClient;
use rust_clock::Clock;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::{Message, ReactionType},
        id::{ChannelId, GuildId},
    },
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::{
    helpers::{command_utils, permissions_helper, voice_utils},
    structures::cmd_data::{Lavalink, SpotifyClient, VoiceTimerMap},
    JesterError, PermissionType,
};

#[command]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let guild_id = msg.guild_id.unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let voice_channel = match voice_channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id
                .say(ctx, "You're not in a voice channel!")
                .await?;
            return Ok(());
        }
    };

    if args.is_empty() {
        msg.channel_id
            .say(ctx, "Please enter a track URL after the command!")
            .await?;
        return Ok(());
    }

    let manager = songbird::get(ctx).await.unwrap();
    let voice_timer_map = ctx
        .data
        .read()
        .await
        .get::<VoiceTimerMap>()
        .cloned()
        .unwrap();

    if manager.get(guild_id).is_none() {
        voice_utils::join_voice_internal(ctx, msg, voice_channel).await?;
    }

    if voice_timer_map.contains_key(&guild_id) {
        if let Some(future_guard) = voice_timer_map.get(&guild_id) {
            future_guard.value().abort();
        }
        voice_timer_map.remove(&guild_id);
    }

    let args_message = args.message();

    let query = if args_message.contains("https://open.spotify.com") {
        let track_id = match args_message.rsplit('/').next() {
            Some(id) => id,
            None => {
                msg.channel_id
                    .say(ctx, JesterError::MissingError("valid Spotify URL"))
                    .await?;
                return Ok(());
            }
        };

        match get_spotify_track_info(track_id, &ctx).await {
            Some(track_info) => track_info,
            None => {
                msg.channel_id
                    .say(ctx, "Couldn't find the track on spotify! Check the URL?")
                    .await?;
                return Ok(());
            }
        }
    } else {
        args_message.to_string()
    };

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let lava_client = lava_lock.lock().await;

    let query_info = lava_client.auto_search_tracks(&query).await?;

    if query_info.tracks.is_empty() {
        msg.channel_id
            .say(ctx, "Couldn't find the video on YouTube! Check the query?")
            .await?;
        return Ok(());
    }

    drop(lava_client);

    if let Err(why) = LavalinkClient::play(guild_id, query_info.tracks[0].clone())
        .queue(Arc::clone(&lava_lock))
        .await
    {
        return Err(why.into());
    };

    let track_info = query_info.tracks[0].info.as_ref();

    let mut cl = Clock::new();
    cl.set_time_ms(track_info.unwrap().length as i64);

    msg.channel_id
        .send_message(ctx, |m| {
            m.content("Added to queue:");
            m.embed(|e| {
                e.color(0x98fb98);
                e.title(&track_info.unwrap().title);
                e.url(&track_info.unwrap().uri);
                e.field("Uploader", &track_info.unwrap().author, true);
                e.field("Length", cl.get_time(), true);
                e.footer(|f| {
                    f.text(format!("Requested by {}", msg.author.name));
                    f
                })
            })
        })
        .await?;

    let ctx_clone = ctx.clone();
    tokio::spawn(async move {
        queue_checker(ctx_clone, guild_id).await;
    });

    Ok(())
}

pub async fn get_spotify_track_info(track_id: &str, ctx: &Context) -> Option<String> {
    let spotify = ctx
        .data
        .read()
        .await
        .get::<SpotifyClient>()
        .cloned()
        .unwrap();

    if let Ok(track) = spotify.tracks().get_track(track_id, None).await {
        Some(track.data.name + " " + &track.data.artists[0].name)
    } else {
        None
    }
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    if lava_client.nodes.contains_key(&guild_id.0) {
        lava_client.pause(guild_id).await?;
        msg.react(ctx, ReactionType::Unicode(String::from("⏸")))
            .await?;

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
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    if !lava_client.nodes.contains_key(&guild_id.0) {
        msg.channel_id
            .say(
                ctx,
                "The bot isn't connected to a voice channel or node! Please re-run join or play!",
            )
            .await?;
        return Ok(());
    }

    lava_client.skip(guild_id).await;
    lava_client.stop(guild_id).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("🛑")))
        .await?;

    let ctx_clone = ctx.clone();
    tokio::spawn(async move {
        voice_utils::create_new_timer(ctx_clone, guild_id).await;
    });

    Ok(())
}

#[command]
#[aliases("unpause")]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    if !lava_client.nodes.contains_key(&guild_id.0) {
        msg.channel_id
            .say(
                ctx,
                "The bot isn't connected to a voice channel or node! Please re-run join or play!",
            )
            .await?;

        return Ok(());
    }

    let voice_timer_map = ctx
        .data
        .read()
        .await
        .get::<VoiceTimerMap>()
        .cloned()
        .unwrap();

    if let Some(future_guard) = voice_timer_map.get(&guild_id) {
        future_guard.value().abort();
    }

    lava_client.resume(msg.guild_id.unwrap()).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("▶")))
        .await?;

    Ok(())
}

#[command]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let lava_client = lava_lock.lock().await;

    let node = match lava_client.nodes.get(&msg.guild_id.unwrap().0) {
        Some(node) => node,
        None => {
            msg.channel_id.say(ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;

            return Ok(());
        }
    };

    let queue = &node.queue;

    if queue.is_empty() && node.now_playing.is_none() {
        msg.channel_id
            .say(ctx, "The queue is currently empty!")
            .await?;

        return Ok(());
    }

    let mut eb = CreateEmbed::default();
    eb.color(0x0377fc);
    eb.title(format!("Queue for {}", guild_id.name(ctx).await.unwrap()));

    if let Some(t) = node.now_playing.as_ref() {
        let t_info = t.track.info.as_ref();

        let mut cl = Clock::new();
        cl.set_time_ms(t_info.unwrap().length as i64);
        eb.field(
            "Now Playing",
            format!(
                "[{}]({}) | `{}`",
                t_info.unwrap().title,
                t_info.unwrap().uri,
                cl.get_time()
            ),
            false,
        );
    }

    if queue.len() > 1 {
        let mut queue_string = String::new();

        for (num, t) in queue.iter().enumerate().skip(1) {
            let t_info = t.track.info.as_ref();

            let mut cl = Clock::new();
            cl.set_time_ms(t_info.unwrap().length as i64);
            queue_string.push_str(&format!(
                "{}. [{}]({}) | `{}` \n\n",
                num,
                t_info.unwrap().title,
                t_info.unwrap().uri,
                cl.get_time()
            ));
        }

        eb.field("Next Songs", queue_string, false);
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.0 = eb.0;
                e
            })
        })
        .await?;

    Ok(())
}

async fn queue_checker(ctx: Context, guild_id: GuildId) {
    loop {
        sleep(Duration::from_secs(60)).await;
        {
            let (voice_timer_map, lava_lock) = {
                let data = ctx.data.read().await;
                let voice_timer_map = data.get::<VoiceTimerMap>().cloned().unwrap();
                let lava_lock = data.get::<Lavalink>().cloned().unwrap();

                (voice_timer_map, lava_lock)
            };

            if voice_timer_map.get(&guild_id).is_some() {
                return;
            }

            let lava_client = lava_lock.lock().await;
            let node = match lava_client.nodes.get(&guild_id.0) {
                Some(node) => node,
                None => return,
            };

            if node.queue.is_empty() && node.now_playing.is_none() {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    voice_utils::create_new_timer(ctx_clone, guild_id).await;
                });
                return;
            }
        }
    }
}

#[command]
#[aliases("c")]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    let node = match lava_client.nodes.get_mut(&msg.guild_id.unwrap().0) {
        Some(node) => node,
        None => {
            msg.channel_id.say(
                ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
            return Ok(());
        }
    };

    if !permissions_helper::check_permission(ctx, msg, None, false).await? {
        msg.channel_id
            .say(
                ctx,
                JesterError::PermissionError(PermissionType::SelfPerm("moderator")),
            )
            .await?;
    } else {
        node.queue.clear();

        msg.react(ctx, ReactionType::Unicode(String::from("💣")))
            .await?;
    }

    Ok(())
}

#[command]
#[aliases("r")]
async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    let node = match lava_client.nodes.get_mut(&msg.guild_id.unwrap().0) {
        Some(node) => node,
        None => {
            msg.channel_id.say(
                ctx, "The bot isn't connected to a voice channel or node! Please re-run join or play!").await?;
            return Ok(());
        }
    };

    let clear_num = match args.single::<usize>() {
        Ok(size) => {
            if size == 0 {
                msg.channel_id
                    .say(ctx, JesterError::MissingError("number greater than 0"))
                    .await?;

                return Ok(());
            }

            size
        }
        Err(_) => {
            msg.channel_id
                .say(ctx, JesterError::MissingError("number"))
                .await?;

            return Ok(());
        }
    };

    node.queue.remove(clear_num - 1);

    let track = node.queue[clear_num - 1].track.info.as_ref();
    let name = &track.unwrap().title;

    msg.channel_id
        .say(ctx, format!("Successfully removed track {}", name))
        .await?;

    Ok(())
}

#[command]
#[aliases("s")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    if !lava_client.nodes.contains_key(&guild_id.0) {
        msg.channel_id
            .say(
                ctx,
                "The bot isn't connected to a voice channel or node! Please re-run join or play!",
            )
            .await?;
        return Ok(());
    }

    if let Some(_track) = lava_client.skip(guild_id).await {
        let node = lava_client.nodes.get(&msg.guild_id.unwrap().0).unwrap();

        if node.queue.is_empty() && node.now_playing.is_none() {
            lava_client.stop(guild_id).await?;
        }
    }

    msg.react(ctx, ReactionType::Unicode(String::from("⏭️")))
        .await?;

    Ok(())
}

#[command]
async fn seek(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .say(ctx, "Please provide a valid number of seconds!")
            .await?;
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(ctx, "You're not in a voice channel!")
            .await?;
        return Ok(());
    }

    let time = match command_utils::deconstruct_time(args.single::<String>().unwrap()) {
        Ok(time) => time,
        Err(e) => {
            msg.channel_id
                .say(
                    ctx,
                    JesterError::MissingError(&format!("valid amount of {}", e)),
                )
                .await?;

            return Ok(());
        }
    };

    let lava_lock = ctx.data.read().await.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_lock.lock().await;

    if !lava_client.nodes.contains_key(&guild_id.0) {
        msg.channel_id
            .say(
                ctx,
                "The bot isn't connected to a voice channel or node! Please re-run join or play!",
            )
            .await?;
        return Ok(());
    };

    lava_client
        .seek(guild_id, Duration::from_secs(time))
        .await?;
    msg.channel_id.say(ctx, "Seeking!").await?;

    Ok(())
}

pub async fn music_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "play <URL or search keywords> : Plays the specified track \n\n",
        "pause: Pauses the current track \n\n",
        "resume <author> <text>: Resumes the current track \nAlias: unpause \n\n",
        "stop: Stops the current track and empties the queue. Doesn't disconnect the bot \n\n",
        "skip: Skips the current track. If there are no tracks in the queue, the player is stopped \n\n",
        "seek <time>: Seeks in the current track using hh:mm:ss format. mm:ss is also supported",
        "clear (track number): either clears the entire queue, or removes a specific track",
        "queue: See the current queue for the guild and what's playing");

    let _ = channel_id
        .send_message(ctx, |m| {
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
        })
        .await;
}
