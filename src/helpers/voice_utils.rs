use serenity::{
    model::channel::Message, 
    client::Context, 
    framework::standard::{macros::command, CommandResult}
};
use crate::structures::{Lavalink, VoiceManager};

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
        msg.channel_id.say(ctx, format!("Joined {}", connect_to)).await?;
    } else {
        msg.channel_id.say(ctx, "There was an error when joining the channel").await?;
    }

    Ok(())
}

#[command]
pub async fn leavevc(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = ctx.cache.guild_channel(msg.channel_id).await.unwrap().guild_id;

    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock().await;
    
    if manager.get(guild_id).is_some() {
        manager.remove(guild_id);
        {
            let data = ctx.data.read().await;
            let lava_client = data.get::<Lavalink>().unwrap();
            lava_client.destroy(&msg.guild_id.unwrap()).await?;
        }

        msg.channel_id.say(ctx, "Left the voice channel!").await?;
    } else {
        msg.channel_id.say(ctx, "Not connected to a Voice channel!").await?;
    }

    Ok(())
}