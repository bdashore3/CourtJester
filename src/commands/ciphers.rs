use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};

/// Encodes a message in base64
/// Usage `b64encode <message>`
#[command]
async fn b64encode(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let b64_string = base64::encode(args.rest());

    msg.channel_id.say(ctx, format!("Your encoded base64 message: `{}`", b64_string)).await?;
    Ok(())
}

/// Decodes a message in base64
/// Usage `b64encode <message>`
#[command]
async fn b64decode(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let b64_bytes = match base64::decode(args.rest()) {
        Ok(bytes) => bytes,
        Err(_error) => {
            msg.channel_id.say(ctx, "Please provide a base64 string!").await?;
            return Ok(())
        }
    };

    let decoded_string = String::from_utf8(b64_bytes).unwrap();

    msg.channel_id.say(ctx, format!("{}", decoded_string)).await?;
    Ok(())
}