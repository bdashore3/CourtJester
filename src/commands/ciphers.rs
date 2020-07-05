use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer
};

use twilight::{
    model::{id::ChannelId, channel::message::Message},
    builders::embed::EmbedBuilder
};

pub async fn encode_b64(ctx: &Context, msg: &Message) -> CommandResult<()> {
    if string_renderer::get_command_length(&msg.content) < 2 {
        send_message(&ctx.http, msg.channel_id, "Please provide a message to encode!").await?;
        return Ok(())
    }

    let b64_string = base64::encode(string_renderer::join_string(&msg.content, 1));

    let mut eb = EmbedBuilder::new();
    eb = eb.title("Base64 Engine");
    eb = eb.add_field("Encoded String",  b64_string).commit();

    send_embed(&ctx.http, msg.channel_id, eb.build()).await?;
    Ok(())
}

pub async fn decode_b64(ctx: &Context, msg: &Message) -> CommandResult<()> {
    if string_renderer::get_command_length(&msg.content) < 2 {
        send_message(&ctx.http, msg.channel_id, "Please provide a message to decode!").await?;
        return Ok(())
    }

     let b64_bytes = match base64::decode(string_renderer::join_string(&msg.content, 1)) {
        Ok(bytes) => bytes,
        Err(_) => {
            send_message(&ctx.http, msg.channel_id, "Please provide a base64 string!").await?;
            return Ok(())
        }
    };

    let decoded_string = String::from_utf8(b64_bytes).unwrap();

    let mut eb = EmbedBuilder::new();
    eb = eb.title("Base64 Engine");
    eb = eb.add_field("Decoded String",  decoded_string).commit();

    send_embed(&ctx.http, msg.channel_id, eb.build()).await?;
    Ok(())
}

pub async fn cipher_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("b64encode <message>: Encodes a message in base64 \n\n");
    content.push_str("b64decode <b64 string>: Decodes a base64 message");
    
    let mut eb = EmbedBuilder::new();

    eb = eb.title("Cipher Help");
    eb = eb.description("Help for Encoding/Decoding messages");
    eb = eb.add_field("Commands", content).commit();

    let _ = send_embed(&ctx.http, channel_id, eb.build()).await;
}