use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer
};

use twilight::{
    model::channel::message::Message,
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