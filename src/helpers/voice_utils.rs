use serenity::{
    model::{
        id::{ChannelId, GuildId}, 
        channel::Message
    }, 
    client::Context,
    framework::standard::{macros::command, CommandResult}
};
use crate::structures::cmd_data::{Lavalink, VoiceManager, VoiceTimerMap};
use serenity_lavalink::nodes::Node;
use futures::future::{Abortable, AbortHandle};
use std::time::Duration;
use tokio::time::delay_for;

#[command]
#[aliases("connect")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
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

    if manager.join(guild.id, connect_to).is_some() {
        let lava_lock = data.get::<Lavalink>().unwrap();
        let mut lava_client = lava_lock.write().await;
        Node::new(&mut lava_client, msg.guild_id.unwrap(), msg.channel_id);

        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            create_new_timer(ctx_clone, guild.id).await;
        });

        msg.channel_id.say(ctx, format!("Joined {}", connect_to.name(ctx).await.unwrap())).await?;
    } else {
        msg.channel_id.say(ctx, "There was an error when joining the channel").await?;
    }

    Ok(())
}

#[command]
#[aliases("dc")]
async fn disconnect(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = ctx.cache.guild_channel(msg.channel_id).await.unwrap().guild_id;
    let guild = msg.guild(ctx).await.unwrap();
    
    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id.say(ctx, "Please connect to a voice channel before executing this command!").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;

    match leavevc_internal(ctx, &guild_id).await {
        Ok(_) => {
            let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();
        
            if let Some(future_guard) = voice_timer_map.get(&guild_id) {
                future_guard.value().abort();
            }

            msg.channel_id.say(ctx, "Left the voice channel!").await?;
        },
        Err(_e) => {
            msg.channel_id.say(ctx, "The bot isn't in a voice channel!").await?;
        }
    }

    Ok(())
}

pub async fn leavevc_internal(ctx: &Context, guild_id: &GuildId) -> CommandResult {
    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock().await;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id);
        {
            let data = ctx.data.read().await;
            let lava_lock = data.get::<Lavalink>().unwrap();
            let mut lava_client = lava_lock.write().await;
            let node = lava_client.nodes.get(guild_id).unwrap().clone();
    
            let _ = node.destroy(&mut lava_client, guild_id).await;
        }
    } else {
        return Err("The bot isn't in a voice channel!".into());
    }

    Ok(())
} 

pub async fn create_new_timer(ctx: Context, guild_id: GuildId) {
    let data = ctx.data.read().await;
    let voice_timer_map = data.get::<VoiceTimerMap>().unwrap();
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(leavevc_internal(&ctx, &guild_id), abort_registration);

    voice_timer_map.insert(guild_id, abort_handle);
    delay_for(Duration::from_secs(300)).await;
    match future.await {
        Ok(_) => {},
        Err(_e) => {}
    };
}

pub async fn voice_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("summon: Forces the bot to join the voice chat \nAlias: connect \n\n");
    content.push_str("disconnect: Leaves the voice chat and clears everything \n\n");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Voice Help");
            e.description("Description: General commands for voice chat");
            e.field("Commands", content, false);
            e.footer(|f| {
                f.text("The user has to be in the voice chat on execution!");
                f
            });
            e
        })
    }).await;
}