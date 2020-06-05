use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult, Args,
    macros::command,
};

use crate::helpers::*;

/// Outputs a spongebob mock string
/// Usage: `mock <message>`
#[command]
#[min_args(1)]
pub async fn mock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mock_string = textmod_helper::get_mock_string(args.rest());
    msg.channel_id.say(ctx, mock_string).await?;

    Ok(())
}

#[command]
pub async fn mockl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let mock_string = textmod_helper::get_mock_string(&input_message[0].content);

    msg.channel_id.say(ctx, mock_string).await?;

    Ok(())
}

/// Inverts the characters in a string
/// Usage: `inv <message>`
#[command]
#[min_args(1)]
async fn inv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let inv_string = textmod_helper::get_inverted_string(args.rest());
    msg.channel_id.say(ctx, inv_string).await?;

    Ok(())
}

#[command]
async fn invl(ctx: &Context, msg: &Message) -> CommandResult {

    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let inv_string = textmod_helper::get_inverted_string(&input_message[0].content);

    msg.channel_id.say(ctx, inv_string).await?;

    Ok(())
}

/// Converts the provided string to uppercase letters
/// Usage: `upp <message>`
#[command]
#[min_args(1)]
async fn upp(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(ctx, args.rest().to_uppercase()).await?;

    Ok(())
}

#[command]
async fn uppl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(ctx, input_message[0].content.to_uppercase()).await?;

    Ok(())
}

/// Converts the provided string to lowercase
/// Usage: `low <message>`
#[command]
#[min_args(1)]
async fn low (ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(ctx, args.rest().to_lowercase()).await?;

    Ok(())
}

#[command]
async fn lowl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(ctx, input_message[0].content.to_lowercase()).await?;

    Ok(())
}

/// Puts a random amount of spaces between each character of the message
/// Usage: `space <message>`
#[command]
#[min_args(1)]
async fn space(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(ctx, textmod_helper::get_spaced_string(args.rest(), false)).await?;

    Ok(())
}

#[command]
async fn spacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(ctx, textmod_helper::get_spaced_string(&input_message[0].content, false)).await?;

    Ok(())
}

/// Similar to space, but puts a larger amount of space between each character
/// Usage: `biggspace <message>`
#[command]
#[min_args(1)]
async fn biggspace(ctx: &Context, msg: &Message, args: Args) -> CommandResult { 
    msg.channel_id.say(ctx, textmod_helper::get_spaced_string(args.rest(), true)).await?;

    Ok(()) 
}

#[command]
async fn biggspacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(ctx, textmod_helper::get_spaced_string(&input_message[0].content, true)).await?;

    Ok(())
}