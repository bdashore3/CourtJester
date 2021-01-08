use regex::Regex;
use serenity::{
    framework::standard::CommandResult,
    model::{
        channel::Message,
        id::{ChannelId, GuildId, MessageId},
    },
};

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "https://discordapp.com/channels/{}/{}/{}",
        guild_id.0, channel_id.0, message_id.0
    )
}

pub fn deconstruct_time(input: String) -> CommandResult<u64> {
    let mut segments = input.rsplit(":");

    let seconds = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(secs) => secs,
        Err(_) => return Err("seconds".into()),
    };
    let minutes = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(mins) => mins,
        Err(_) => return Err("minutes".into()),
    };
    let hours = match segments.next().unwrap_or("0").parse::<u64>() {
        Ok(mins) => mins,
        Err(_) => return Err("hours".into()),
    };

    let result = seconds + (minutes * 60) + (hours * 3600);

    Ok(result)
}

pub fn check_mention_prefix(msg: &Message) -> bool {
    let words = msg.content.split_whitespace().collect::<Vec<&str>>();

    let re = Regex::new(r"<@!?\d+>").unwrap();

    re.is_match(words[0])
}

pub fn get_allowed_commands() -> Vec<String> {
    let allowed_commands: Vec<String> = vec![
        "prefix".to_owned(),
        "help".to_owned(),
        "restore".to_owned(),
        "resetprefix".to_owned(),
    ];

    allowed_commands
}