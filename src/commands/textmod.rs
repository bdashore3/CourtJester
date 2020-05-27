use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult, Args,
    macros::command,
};

use crate::helpers::*;

#[command]
pub async fn mock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mock_string = textmod_helper::get_mock_string(args.rest());
    msg.channel_id.say(&ctx.http, mock_string).await?;

    Ok(())
}

#[command]
pub async fn mockl(ctx: &Context, msg: &Message) -> CommandResult {

    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let mock_string = textmod_helper::get_mock_string(&input_message[0].content);

    msg.channel_id.say(&ctx.http, mock_string).await?;

    Ok(())
}

#[command]
async fn inv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let inv_string = textmod_helper::get_inverted_string(args.rest());
    msg.channel_id.say(&ctx.http, inv_string).await?;

    Ok(())
}

#[command]
async fn invl(ctx: &Context, msg: &Message) -> CommandResult {

    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let inv_string = textmod_helper::get_inverted_string(&input_message[0].content);

    msg.channel_id.say(&ctx.http, inv_string).await?;

    Ok(())
}

#[command]
async fn upp(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, args.rest().to_uppercase()).await?;

    Ok(())
}

#[command]
async fn uppl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(&ctx.http, input_message[0].content.to_uppercase()).await?;

    Ok(())
}

#[command]
async fn low (ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, args.rest().to_lowercase()).await?;

    Ok(())
}

#[command]
async fn lowl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(&ctx.http, input_message[0].content.to_lowercase()).await?;

    Ok(())
}

#[command]
async fn space(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, textmod_helper::get_spaced_string(args.rest(), false)).await?;

    Ok(())
}

#[command]
async fn spacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(&ctx.http, textmod_helper::get_spaced_string(&input_message[0].content, false)).await?;

    Ok(())
}

#[command]
async fn biggspace(ctx: &Context, msg: &Message, args: Args) -> CommandResult { 
    msg.channel_id.say(&ctx.http, textmod_helper::get_spaced_string(args.rest(), true)).await?;

    Ok(()) 
}

#[command]
async fn biggspacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    msg.channel_id.say(&ctx.http, textmod_helper::get_spaced_string(&input_message[0].content, true)).await?;

    Ok(())
}