use serenity::{model::{
    channel::ReactionType, 
    id::{ChannelId, GuildId, MessageId}
}, framework::standard::CommandResult};

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!("https://discordapp.com/channels/{}/{}/{}", guild_id.0, channel_id.0, message_id.0)
}

pub fn get_reaction_emoji(emoji_type: &ReactionType) -> &str {
    if let ReactionType::Unicode(name) = emoji_type {
        name
    }
    else {
        ""
    }
}

pub fn deconstruct_time(input: String) -> CommandResult<u64> {
    let mut segments = input.rsplit(":");

    let seconds = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(secs) => secs,
        Err(_) => return Err("seconds".into())
    };
    let minutes = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(mins) => mins,
        Err(_) => return Err("minutes".into())
    };
    let hours = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(mins) => mins,
        Err(_) => return Err("hours".into())
    };

    let result = seconds + (minutes*60) + (hours*3600);

    Ok(result)
}
