use regex::Regex;
use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::{
        channel::Message,
        id::{ChannelId, GuildId, MessageId},
    },
};

use crate::structures::cmd_data::{PrefixMap, PubCreds};

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "https://discordapp.com/channels/{}/{}/{}",
        guild_id.0, channel_id.0, message_id.0
    )
}

// `str::split_whitespace` returns substrings to the original string
// this means we can calculate the index to the beginning of the substring
// in the original string with simple arithmetic on their pointers
// PRECONDITION: `l` and `r` must be from the same memory
fn ptr_diff(l: &str, r: &str) -> usize {
    l.as_ptr() as usize - r.as_ptr() as usize
}

pub fn truncate(s: &str, words: usize) -> String {
    if s.is_empty() || words == 0 {
        return String::new();
    }

    let s = s.trim();

    let n = match s.split_whitespace().take(words).last() {
        Some(word) => {
            let index = ptr_diff(word, s) + word.len();

            // If `index` is equal to the length of the string,
            // then the string contains less words than `words`
            if index == s.len() {
                return s.to_string();
            }

            index
        }
        None => return s.to_string(),
    };

    format!("{}...", &s[..n])
}

#[allow(clippy::needless_lifetimes)]
pub async fn get_command_name<'a>(ctx: &Context, msg: &'a Message) -> &'a str {
    let (prefixes, default_prefix) = {
        let data = ctx.data.read().await;

        let prefixes = data.get::<PrefixMap>().unwrap().clone();
        let default_prefix = data
            .get::<PubCreds>()
            .unwrap()
            .get("default prefix")
            .cloned()
            .unwrap();

        (prefixes, default_prefix)
    };

    let guild_id = msg.guild_id.unwrap();

    let prefix_length = match prefixes.get(&guild_id) {
        Some(prefix_guard) => prefix_guard.value().len(),
        None => default_prefix.len(),
    };

    let words = msg.content.split_whitespace().collect::<Vec<&str>>();
    let command = words[0];

    &command[prefix_length..]
}

pub fn deconstruct_time(input: String) -> CommandResult<u64> {
    let mut segments = input.rsplit(':');

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
