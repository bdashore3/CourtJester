use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer,
    commands::{
        config::*,
        starbot::*,
        textchannel_send::*,
        ciphers::*,
        textmod::*
    }
};
use twilight::{
    model::{
        channel::Message, 
        id::ChannelId
    }, 
    builders::embed::EmbedBuilder
};

pub async fn send_help(ctx: &Context, msg: &Message) -> CommandResult<()> {
    if string_renderer::get_command_length(&msg.content) < 2 {
        default_help_message(ctx, msg.channel_id).await;
        return Ok(())
    }

    let subcommand = string_renderer::get_message_word(&msg.content, 1);

    match subcommand {
        "prefix" => prefix_help(ctx, msg.channel_id).await,
        "command" => command_help(ctx, msg.channel_id).await,
        "starboard" => starbot_help(ctx, msg.channel_id).await,
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

    let mut eb = EmbedBuilder::new();
    eb = eb.title("CourtJester Help");
    eb = eb.description("Help for the CourtJester Discord bot");
    eb = eb.add_field("Subcategories", format!("```\n{}```", categories)).commit();

    let _ = send_embed(&ctx.http, channel_id, eb.build()).await;
}