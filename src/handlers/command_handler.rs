use twilight::model::channel::message::Message;
use crate::helpers::string_renderer;
use crate::helpers::command_utils;
use crate::structures::{
    CommandResult,
    Context,
};
use crate::commands;

pub async fn handle_command(msg: Message, ctx: &Context<'_>) -> CommandResult {
    let command = string_renderer::get_message_word(&msg.content, 0);
    match command {
        "ping" => command_utils::send_message(ctx.http, msg.channel_id, "Pong!").await?,
        "mock" => commands::textmod::mock(ctx, msg, false).await?,
        "mockl" => commands::textmod::mock(ctx, msg, true).await?,
        _ => println!("No such command!"),
    };

    Ok(())
}