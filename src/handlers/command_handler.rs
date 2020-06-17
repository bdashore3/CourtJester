use twilight::model::channel::message::Message;
use crate::helpers::string_renderer;
use crate::helpers::command_utils;
use crate::structures::{
    CommandResult,
    Context,
};
use crate::commands::{
    textmod::*,
    textchannel_send::*
};

pub async fn handle_command(msg: Message, ctx: &Context<'_>) -> CommandResult {
    let command = string_renderer::get_message_word(&msg.content, 0);
    match command {
        "ping" => command_utils::send_message(ctx.http, msg.channel_id, "Pong!").await?,
        "mock" => mock(ctx, msg, false).await?,
        "mockl" => mock(ctx, msg, true).await?,
        "upp" => upp(ctx, msg, false).await?,
        "low" => low(ctx, msg, false).await?,
        "inv" => inv(ctx, msg, false).await?,
        "nice" => nice(ctx, msg).await?,
        "bruh" => bruh(ctx, msg).await?,
        _ => println!("No such command!"),
    };

    Ok(())
}