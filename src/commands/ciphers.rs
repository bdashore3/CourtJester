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
    let b64_bytes = base64::decode(args.rest()).unwrap();

    let decoded_string = String::from_utf8(b64_bytes).unwrap();

    msg.channel_id.say(&ctx.http, format!("{}", decoded_string)).await?;
    Ok(())
}