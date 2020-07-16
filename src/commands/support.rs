use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};
use crate::commands::{
    config::*,
    starboard::*,
    textchannel_send::*,
    ciphers::*,
    textmod::*
};

#[command]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 {
        default_help_message(ctx, msg.channel_id).await;
        return Ok(())
    }

    let subcommand = args.single::<String>()?;
    
    match subcommand.as_str() {
        "prefix" => prefix_help(ctx, msg.channel_id).await,
        "command" => command_help(ctx, msg.channel_id).await,
        "starboard" => starboard_help(ctx, msg.channel_id).await,
        "senders" => sender_help(ctx, msg.channel_id).await,
        "ciphers" => cipher_help(ctx, msg.channel_id).await,
        "text" => textmod_help(ctx, msg.channel_id).await,
        _ => {}
    }

    Ok(())
}

async fn default_help_message(ctx: &Context, channel_id: ChannelId) {
    let mut categories = String::new();
    categories.push_str("prefix \n");
    categories.push_str("command \n");
    categories.push_str("starboard \n");
    categories.push_str("senders \n");
    categories.push_str("ciphers \n");
    categories.push_str("text \n");

    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("CourtJester Help");
            e.description("Help for the CourtJester Discord bot");
            e.field("Subcategories", format!("```\n{}```", categories), false);
            e
        })
    }).await;
}