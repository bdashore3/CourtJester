use serenity::{
    client::Context, 
    framework::standard::{
        Args, 
        macros::command, 
        CommandResult
    }, 
    model::channel::{ReactionType, Message}, builder::CreateEmbed
};
use crate::{helpers::command_utils, structures::{Lavalink, VoiceManager}};
use std::sync::Arc;
use rust_clock::Clock;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    if args.len() < 1 {
        msg.channel_id.say(ctx, "Please enter a track URL after the command!").await?;
        return Ok(())
    }

    let query = args.message().to_string();

    let guild_id = msg.guild_id.unwrap();

    let data = ctx.data.read().await;
    let manager_lock = data.get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock().await;

    if let Some(handler) = manager.get_mut(guild_id) {
        let lava_lock = data.get::<Lavalink>().unwrap();
        let mut lava_client = lava_lock.write().await;

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

        msg.channel_id.say(ctx, format!("Added to queue: {}", query_info.tracks[0].info.title)).await?;
    } else {
        msg.channel_id.say(ctx, "Please make sure the bot has joined the vc!").await?;
    }

    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    let node = lava_client.nodes.get_mut(&guild_id).unwrap();

    node.pause(&lava_client_read, &guild_id).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("⏸"))).await?;

    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let mut lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    let node = lava_client.nodes.get_mut(&guild_id).unwrap();

    node.stop(&mut lava_client_read, &guild_id).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("🛑"))).await?;

    Ok(())
}

#[command]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let lava_client_read = lava_lock.read().await.clone();
    let mut lava_client = lava_lock.write().await;
    let node = lava_client.nodes.get_mut(&guild_id).unwrap();

    node.resume(&lava_client_read, &msg.guild_id.unwrap()).await?;
    msg.react(ctx, ReactionType::Unicode(String::from("▶"))).await?;

    Ok(())
}

#[command]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().read().await;

    if let Some(node) = lava_client.nodes.get(&msg.guild_id.unwrap()) {
        let track = node.now_playing.as_ref();
        if let Some(t) = track {
            msg.channel_id.say(ctx, format!("Now playing: {}", t.track.info.title)).await?;
        } else {
            msg.channel_id.say(ctx, "There is nothing playing right now").await?;
        }
    }

    Ok(())
}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_client = data.get::<Lavalink>().unwrap().read().await;
    let node = lava_client.nodes.get(&msg.guild_id.unwrap()).unwrap();
    let queue = &node.queue;

    let mut eb = CreateEmbed::default();
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

#[command]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().unwrap();
    let mut lava_client = lava_lock.write().await.clone();
    let node = lava_client.nodes.get_mut(&guild_id).unwrap();

    if node.queue.is_empty() {
        let mut lava_client_read = lava_lock.read().await.clone();
        node.stop(&mut lava_client_read, &guild_id).await?;
    } else {
        node.skip();
    }

    msg.channel_id.say(ctx, "Skipping the current track...").await?;

    Ok(())
}