use twilight::{
    http::Client,
    model::{channel::embed::Embed, id::{GuildId, ChannelId, MessageId}}
};

pub async fn send_message(http: &Client, channel_id: ChannelId, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).content(format!("{}", content))?.await?;

    Ok(())
}

pub async fn send_embed(http: &Client, channel_id: ChannelId, content: Embed) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).embed(content)?.await?;
    
    Ok(())
}

pub fn get_raw_id(given_id: &str, mention_type: &str) -> Result<u64, std::num::ParseIntError> {
    let output = match mention_type {
        "channel" => {
            match given_id[2 .. given_id.len() - 1].parse::<u64>() {
                Ok(i) => i,
                Err(e) => return Err(e) 
            }
        },
        "user" => {
            if &given_id[..3] == "<@!" {
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