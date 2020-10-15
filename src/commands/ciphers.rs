use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};
use crate::JesterError;

/// Encodes a message in base64
/// Usage `b64encode <message>`
#[command]
async fn b64encode(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let b64_string = base64::encode(args.rest());

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Base64 Engine");
            e.description(format!("Encoded Message: `{}`", b64_string));
            e
        })
    }).await?;
    Ok(())
}

/// Decodes a message in base64
/// Usage `b64encode <message>`
#[command]
async fn b64decode(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let b64_bytes = match base64::decode(args.rest()) {
        Ok(bytes) => bytes,
        Err(_error) => {
            msg.channel_id.say(ctx, JesterError::MissingError("base64 string")).await?;
            return Ok(())
        }
    };

    let decoded_string = String::from_utf8(b64_bytes).unwrap();

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Base64 Engine");
            e.description(format!("Decoded Message: `{}`", decoded_string));
            e
        })
    }).await?;

    Ok(())
}

pub async fn cipher_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "b64encode <message>: Encodes a message in base64 \n\n",
        "b64decode <b64 string>: Decodes a base64 message");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Cipher Help");
            e.description("Description: Encoding/Decoding messages");
            e.field("Commands", content, false);
            e
        })
    }).await;
}
