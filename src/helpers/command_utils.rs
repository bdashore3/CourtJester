use serenity::model::{
    channel::ReactionType, 
    id::{ChannelId, GuildId, MessageId}
};

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