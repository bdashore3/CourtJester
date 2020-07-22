use serenity::{
    model::{id::GuildId, channel::Message}, 
    client::Context, 
    framework::standard::{macros::command, CommandResult}
};
use crate::{
    helpers::command_utils,
    structures::{Lavalink, VoiceManager, VoiceTimerMap}
};
use serenity_lavalink::nodes::Node;
use futures::future::{Abortable, AbortHandle};
use std::time::Duration;
use tokio::time::delay_for;

#[command]
pub async fn joinvc(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id.say(ctx, "You're not in a voice channel!").await?;

            return Ok(())
        }
    };

    let data = ctx.data.read().await;
    let manager_lock = data.get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock().await;

    if manager.join(msg.guild_id.unwrap(), connect_to).is_some() {
        let lava_lock = data.get::<Lavalink>().unwrap();
        let mut lava_client = lava_lock.write().await;
        Node::new(&mut lava_client, msg.guild_id.unwrap(), msg.channel_id);

        msg.channel_id.say(ctx, format!("Joined {}", connect_to)).await?;
    } else {
        msg.channel_id.say(ctx, "There was an error when joining the channel").await?;
    }

    Ok(())
}

#[command]
pub async fn leavevc(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = ctx.cache.guild_channel(msg.channel_id).await.unwrap().guild_id;
    let guild = msg.guild(ctx).await.unwrap();
    
    if !command_utils::check_voice_state(guild, msg.author.id).await {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
        return Ok(())
    }

    match leavevc_internal(ctx, &guild_id).await {
        Ok(_) => {
            let data = ctx.data.read().await;
            let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();
        
            if let Some(future_guard) = voice_timer_map.get(&guild_id) {
                future_guard.value().abort();
            }

            msg.channel_id.say(ctx, "Left the voice channel!").await?;
        },
        Err(e) => {
            msg.channel_id.say(ctx, "There was an error when trying to disconnect!").await?;
        }
    }

    Ok(())
}

pub async fn leavevc_internal(ctx: &Context, guild_id: &GuildId) -> CommandResult {
    let data = ctx.data.read().await;
    let manager_lock = data.get::<VoiceManager>().unwrap();
    let mut manager = manager_lock.lock().await;

    manager.remove(guild_id);
    {
        let data = ctx.data.read().await;
        let lava_lock = data.get::<Lavalink>().unwrap();
        let mut lava_client = lava_lock.write().await;
        let node = lava_client.nodes.get(guild_id).unwrap().clone();

        let _ = node.destroy(&mut lava_client, guild_id).await;
    }

    Ok(())
} 

pub async fn create_new_timer(ctx: Context, guild_id: GuildId) {
    let data = ctx.data.read().await;
    let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(leavevc_internal(&ctx, &guild_id), abort_registration);

    voice_timer_map.insert(guild_id, abort_handle);
    delay_for(Duration::from_secs(600)).await;
    match future.await {
        Ok(_) => {},
        Err(_e) => {}
    };
}