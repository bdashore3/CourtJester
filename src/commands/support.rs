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
    textmod::*,
    music::*,
    images::*
};
use crate::helpers::voice_utils::*;

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
        "voice" => voice_help(ctx, msg.channel_id).await,
        "music" => music_help(ctx, msg.channel_id).await,
        "images" => image_help(ctx, msg.channel_id).await,
        "support" => support_message(ctx, msg.channel_id).await,
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
    categories.push_str("voice \n");
    categories.push_str("music \n");
    categories.push_str("images \n");

    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("CourtJester Help");
            e.description("Help for the CourtJester Discord bot");
            e.field("Subcategories", format!("```\n{}```", categories), false);
            e
        })
    }).await;
}

async fn support_message(ctx: &Context, channel_id: ChannelId) {
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("CourtJester Support");
            e.description("Need more help?");
            e.field("Support Server", "https://discord.gg/pswt7by", false);
            e.field("kingbri's twitter", "https://twitter.com/kingbri1st", false);
            e.footer(|f| {
                f.text("Created with ❤️ by kingbri#6666");
                f
            })
        })
    }).await;
}