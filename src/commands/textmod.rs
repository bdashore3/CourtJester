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
#[aliases("bigspace")]
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

pub async fn textmod_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("mock <message>: Spongebob mocks a string \n\n");
    content.push_str("inv <message>: Inverts capitalization of each letter in the message \n\n");
    content.push_str("upp <message>: Every letter becomes uppercase \n\n");
    content.push_str("low <message>: Every letter becomes lowercase \n\n");
    content.push_str("space <message>: Spaces out each letter in the message (whitespace omitted) \n\n");
    content.push_str("biggspace <message>: Same as space, but W I D E R");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Text Modification Help");
            e.description("Description: Commands that modify text");
            e.field("Commands", content, false);
            e.footer(|f| {
                f.text("Putting an l in front of any command will use the last message");
                f
            });
            e
        })
    }).await;
}