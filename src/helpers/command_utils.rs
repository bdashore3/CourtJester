use twilight::{
    http::Client,
    model::id::ChannelId
};

pub async fn send_message(http: &Client, channel_id: ChannelId, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).content(format!("{}", content))?.await?;

    Ok(())
}