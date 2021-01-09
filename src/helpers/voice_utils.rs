use futures::future::{AbortHandle, Abortable};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};
use std::time::Duration;
use tokio::time::sleep;

use crate::structures::cmd_data::{Lavalink, VoiceTimerMap};

#[command]
#[aliases("connect")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let voice_channel = match channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id
                .say(ctx, "You're not in a voice channel!")
                .await?;

            return Ok(());
        }
    };

    match join_voice_internal(ctx, msg, voice_channel).await {
        Ok(_) => {
            msg.channel_id
                .say(
                    ctx,
                    format!("Joined {}", voice_channel.name(ctx).await.unwrap()),
                )
                .await?;

            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                create_new_timer(ctx_clone, guild.id).await;
            });
        }
        Err(_e) => {
            msg.channel_id
                .say(ctx, "There was an error when joining the channel")
                .await?;
        }
    }

    Ok(())
}

pub async fn join_voice_internal(
    ctx: &Context,
    msg: &Message,
    voice_channel: ChannelId,
) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await.unwrap().clone();

    let (_, handler) = manager.join_gateway(guild_id, voice_channel).await;

    match handler {
        Ok(conn_info) => {
            let mut data = ctx.data.write().await;
            let lava_lock = data.get_mut::<Lavalink>().unwrap();
            lava_lock
                .lock()
                .await
                .create_session(guild_id, &conn_info)
                .await?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

#[command]
#[aliases("dc")]
async fn disconnect(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = ctx
        .cache
        .guild_channel_field(msg.channel_id, |channel| channel.guild_id)
        .await
        .unwrap();
    let guild = msg.guild(ctx).await.unwrap();

    if !guild.voice_states.contains_key(&msg.author.id) {
        msg.channel_id
            .say(
                ctx,
                "Please connect to a voice channel before executing this command!",
            )
            .await?;
        return Ok(());
    }

    match leavevc_internal(ctx, guild_id).await {
        Ok(_) => {
            let voice_timer_map = ctx
                .data
                .read()
                .await
                .get::<VoiceTimerMap>()
                .cloned()
                .unwrap();

            if voice_timer_map.contains_key(&guild_id) {
                if let Some(future_guard) = voice_timer_map.get(&guild_id) {
                    future_guard.value().abort();
                }
                voice_timer_map.remove(&guild_id);
            }

            msg.channel_id.say(ctx, "Left the voice channel!").await?;
        }
        Err(_e) => {
            msg.channel_id
                .say(ctx, "The bot isn't in a voice channel!")
                .await?;
        }
    }

    Ok(())
}

pub async fn leavevc_internal(ctx: &Context, guild_id: GuildId) -> CommandResult {
    let manager = songbird::get(ctx).await.unwrap().clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;

        {
            let mut data = ctx.data.write().await;
            let lava_lock = data.get_mut::<Lavalink>().unwrap();
            let mut lava_client = lava_lock.lock().await;

            lava_client.destroy(guild_id.0).await?;
            if let Some(node) = lava_client.nodes.get_mut(&guild_id.0) {
                node.now_playing = None;
                node.queue = Vec::new();
            }
        }
    } else {
        return Err("The bot isn't in a voice channel!".into());
    }

    Ok(())
}

pub async fn create_new_timer(ctx: Context, guild_id: GuildId) {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(leavevc_internal(&ctx, guild_id), abort_registration);

    let voice_timer_map = ctx
        .data
        .read()
        .await
        .get::<VoiceTimerMap>()
        .cloned()
        .unwrap();

    voice_timer_map.insert(guild_id, abort_handle);

    sleep(Duration::from_secs(300)).await;
    match future.await {
        Ok(_) => {}
        Err(_e) => {}
    };
}

pub async fn voice_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "summon: Forces the bot to join the voice chat \nAlias: connect \n\n",
        "disconnect: Leaves the voice chat and clears everything \n\n"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
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
        })
        .await;
}
