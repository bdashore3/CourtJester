use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult, Args,
    macros::command,
};

use rand::prelude::*;

#[command]
pub async fn mock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mock_string = get_mock_string(args.rest());
    msg.channel_id.say(&ctx.http, mock_string).await?;

    Ok(())
}

#[command]
pub async fn mockl(ctx: &Context, msg: &Message) -> CommandResult {

    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let mock_string = get_mock_string(&input_message[0].content);

    msg.channel_id.say(&ctx.http, mock_string).await?;

    Ok(())
}

fn get_mock_string(input: &str) -> String {

    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if random() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
    }

    output
}

#[command]
async fn inv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let inv_string = get_inverted_string(args.rest());
    msg.channel_id.say(&ctx.http, inv_string).await?;

    Ok(())
}

#[command]
async fn invl(ctx: &Context, msg: &Message) -> CommandResult {

    let input_message = msg.channel_id.messages(&ctx.http, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;
    
    let inv_string = get_inverted_string(&input_message[0].content);

    msg.channel_id.say(&ctx.http, inv_string).await?;

    Ok(())
}


fn get_inverted_string(input: &str) -> String {
    
    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if x.is_uppercase() {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
        else if x.is_lowercase() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
    }

    output
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