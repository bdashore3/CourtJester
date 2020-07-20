use serenity::{client::{bridge::voice::ClientVoiceManager, Context}, model::{channel::{Message, ReactionType}, id::{ChannelId, GuildId, MessageId, UserId}, guild::Guild}, prelude::Mutex};
use crate::structures::VoiceManager;
use std::sync::Arc;

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

pub async fn check_voice_state(guild: Guild, user_id: UserId) -> bool {
    let mut vc_active = true;

    if guild.voice_states.contains_key(&user_id) {
        vc_active = true;
    } else {
        vc_active = false;
    }

    vc_active
}