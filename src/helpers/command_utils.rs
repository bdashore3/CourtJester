use twilight::{
    http::Client,
    model::id::ChannelId
};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::structures::Context;

pub async fn send_message(http: Client, channel_id: ChannelId, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    http.create_message(channel_id).content(format!("{}", content))?.await?;

    Ok(())
}

pub async fn create_context(http: Client, pool: PgPool, data: HashMap<String, String>) -> Context {
    let ctx = Context {
        http: http,
        pool: pool,
        data: data
    };

    ctx
}