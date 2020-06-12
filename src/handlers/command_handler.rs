use twilight::model::gateway::payload::MessageCreate;
use twilight::http::Client;
use crate::helpers::string_renderer;
use crate::helpers::command_utils;
use crate::structures::{
    CommandResult,
    Context
};
use crate::commands;

pub async fn handle_command(msg: Box<MessageCreate>, ctx: Context) -> CommandResult {
    let command = string_renderer::get_command(&msg.content);
    match command {
        "ping" => command_utils::send_message(ctx.http, msg.channel_id, "Pong!").await?,
        _ => println!("No such command!"),
    };

    Ok(())
}