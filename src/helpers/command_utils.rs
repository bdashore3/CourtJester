use twilight::{
    http::Client,
    model::{
        channel::{Message, embed::Embed, ReactionType, GuildChannel}, 
        id::{GuildId, ChannelId, MessageId, UserId}
    }, 
};
use crate::structures::Context;
use std::{sync::Arc, fmt::Display};

pub async fn send_message(http: &Client, channel_id: ChannelId, message: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).content(message.into())?.await?;

    Ok(())
}

pub async fn send_embed(http: &Client, channel_id: ChannelId, content: Embed) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).embed(content)?.await?;

    Ok(())
}

pub fn get_raw_id(given_id: &str, mention_type: &str) -> Result<u64, std::num::ParseIntError> {
    if !given_id.contains("<") {
        return Ok(0)
    }

    let output = match mention_type {
        "channel" => {
            match given_id[2 .. given_id.len() - 1].parse::<u64>() {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        "user" => {
            if &given_id[..3] == "<@!" && &given_id[given_id.len() - 1 .. given_id.len()] == ">" {
                match given_id[3 .. given_id.len() - 1].parse::<u64>() {
                    Ok(i) => i,
                    Err(e) => return Err(e)
                }
            }
            else {
                match given_id[2 .. given_id.len() - 1].parse::<u64>() {
                    Ok(i) => i,
                    Err(e) => return Err(e)
                }
            }
        },
        _ => 0
    };

    Ok(output)
}

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!("https://discordapp.com/channels/{}/{}/{}", guild_id.0, channel_id.0, message_id.0)
}

pub fn get_avatar_url(user_id: UserId, avatar_hash: impl Display) -> String {
    format!("https://cdn.discordapp.com/avatars/{}/a_{}.webp?size=256", user_id.0, avatar_hash)
}

pub fn get_default_avatar_url(discriminator: &str) -> String {
    format!("https://cdn.discordapp.com/embed/avatars/{}.png", discriminator.parse::<i32>().unwrap() % 5)
}

pub async fn get_last_message(ctx: &Context, channel_id: ChannelId, message_id: MessageId) -> Result<Message, Box<dyn std::error::Error>> {
    let mut messages = ctx.http
            .channel_messages(channel_id)
            .before(message_id)
            .limit(2)?
            .await?;
    let last_message = messages.remove(0);

    Ok(last_message)
}

pub fn get_reaction_emoji(reaction_type: &ReactionType) -> &str {
    if let ReactionType::Unicode { name } = reaction_type {
        name
    }
    else {
        ""
    }
}

pub fn nsfw_check(wrapped_channel: Arc<GuildChannel>) -> bool {
    let mut is_nsfw = false;

    if let GuildChannel::Text(ref channel) = *wrapped_channel {
        is_nsfw = channel.nsfw;
    }

    is_nsfw
}