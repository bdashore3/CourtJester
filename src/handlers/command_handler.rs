use twilight::model::channel::message::Message;
use crate::helpers::string_renderer;
use crate::helpers::command_utils::*;
use crate::structures::{
    CommandResult,
    Context
};
use crate::commands::{
    textmod::*,
    textchannel_send::*,
    ciphers::*,
    config::*
};

pub async fn handle_command(msg: &Message, ctx: &Context<'_>, prefix_len: usize) -> CommandResult {
    let command = string_renderer::get_command(&msg.content, prefix_len);
    match command {
        "ping" => send_message(ctx.http, msg.channel_id, "Pong!").await?,
        "mock" => mock(ctx, msg, false).await?,
        "mockl" => mock(ctx, msg, true).await?,
        "upp" => upp(ctx, msg, false).await?,
        "uppl" => upp(ctx, msg, true).await?,
        "low" => low(ctx, msg, false).await?,
        "lowl" => low(ctx, msg, true).await?,
        "inv" => inv(ctx, msg, false).await?,
        "invl" => inv(ctx, msg, true).await?,
        "space" => space(ctx, msg, false, false).await?,
        "spacel" => space(ctx, msg, true, false).await?,
        "biggspace" => space(ctx, msg, false, true).await?,
        "biggspacel" => space(ctx, msg, true, true).await?,
        "nice" => nice(ctx, msg).await?,
        "bruh" => bruh(ctx, msg).await?,
        "b64encode" => encode_b64(ctx, msg).await?,
        "b64decode" => decode_b64(ctx, msg).await?,
        "prefix" => handle_prefix(ctx, msg).await?,
        "command" => dispatch_custom_command(ctx, msg).await?,
        _ => {
            let data = sqlx::query!(
                    "SELECT content FROM commands WHERE guild_id = $1 AND name = $2", 
                    msg.guild_id.unwrap().0 as i64, command)
                .fetch_optional(ctx.pool)
                .await?;

            if let Some(data) = data {
                let content = data.content.unwrap()
                    .replace("{user}", &format!("<@!{}>", msg.author.id.0));
                send_message(ctx.http, msg.channel_id, content).await?;
            }
        },
    };

    Ok(())
}