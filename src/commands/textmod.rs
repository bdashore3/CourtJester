use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult, Args,
    macros::command,
};
use crate::{JesterError, helpers::*};

/// Outputs a spongebob mock string
/// Usage: `mock <message>`
#[command]
#[min_args(1)]
pub async fn mock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to mock")).await?;
    } else {
        let mock_string = textmod_helper::get_mock_string(args.rest());

        msg.channel_id.say(ctx, mock_string).await?;
    }

    Ok(())
}

#[command]
pub async fn mockl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    }).await?;
    
    let mock_string = textmod_helper::get_mock_string(&input_message[0].content);

    msg.channel_id.say(ctx, mock_string).await?;

    Ok(())
}

/// Inverts the characters in a string
/// Usage: `inv <message>`
#[command]
#[min_args(1)]
async fn inv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to invert")).await?;
    } else {
        let inv_string = textmod_helper::get_inverted_string(args.rest());

        msg.channel_id.say(ctx, inv_string).await?;
    }

    Ok(())
}

#[command]
async fn invl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    }).await?;
    
    let inv_string = textmod_helper::get_inverted_string(&input_message[0].content);

    msg.channel_id.say(ctx, inv_string).await?;

    Ok(())
}

/// Converts the provided string to uppercase letters
/// Usage: `upp <message>`
#[command]
#[min_args(1)]
async fn upp(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to uppercase")).await?;
    } else {
        msg.channel_id.say(ctx, args.rest().to_uppercase()).await?;
    }

    Ok(())
}

#[command]
async fn uppl(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    }).await?;

    msg.channel_id.say(ctx, input_message[0].content.to_uppercase()).await?;

    Ok(())
}

/// Converts the provided string to lowercase
/// Usage: `low <message>`
#[command]
#[min_args(1)]
async fn low (ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to lowercase")).await?;
    } else {
        msg.channel_id.say(ctx, args.rest().to_lowercase()).await?;
    }

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
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to space out")).await?;
    } else {
        let spaced_string = textmod_helper::get_spaced_string(args.rest(), false);

        msg.channel_id.say(ctx, spaced_string).await?;
    }

    Ok(())
}

#[command]
async fn spacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    })
    .await?;

    let spaced_string = textmod_helper::get_spaced_string(&input_message[0].content, false);

    msg.channel_id.say(ctx, spaced_string).await?;

    Ok(())
}

/// Similar to space, but puts a larger amount of space between each character
/// Usage: `biggspace <message>`
#[command]
#[aliases("bigspace")]
#[min_args(1)]
async fn biggspace(ctx: &Context, msg: &Message, args: Args) -> CommandResult { 
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to B I G G S P A C E")).await?;
    } else {
        let bigspace_string = textmod_helper::get_spaced_string(args.rest(), true);

        msg.channel_id.say(ctx, bigspace_string).await?;
    }

    Ok(()) 
}

#[command]
async fn biggspacel(ctx: &Context, msg: &Message) -> CommandResult {
    let input_message = msg.channel_id.messages(ctx, |retriever| {
        retriever.before(msg.id).limit(1)
    }).await?;

    let bigspace_string = textmod_helper::get_spaced_string(&input_message[0].content, true);

    msg.channel_id.say(ctx, bigspace_string).await?;

    Ok(())
}

#[command]
async fn h4ck(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to h4ck")).await?;
    } else {
        let hacked_string = textmod_helper::get_hacked_string(args.rest());

        msg.channel_id.say(ctx, hacked_string).await?;
    }

    Ok(())
}

#[command]
async fn uwu(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("string to uwu")).await?;
    } else {
        let uwu_string = textmod_helper::get_uwu_string(args.rest());

        msg.channel_id.say(ctx, uwu_string).await?;
    }

    Ok(())
}

pub async fn textmod_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "mock <message>: Spongebob mocks a string \n\n",
        "inv <message>: Inverts capitalization of each letter in the message \n\n",
        "upp <message>: Every letter becomes uppercase \n\n",
        "low <message>: Every letter becomes lowercase \n\n",
        "space <message>: Spaces out each letter in the message (whitespace omitted) \n\n",
        "biggspace <message>: Same as space, but W I D E R",
        "h4ck <message>: Become a hackerman by making h4ck3d w0rd5",
        "uwu <message>: Translate to the uwu wanguwage uwu");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Text Modification Help");
            e.description("Description: Commands that modify text");
            e.field("Commands", content, false);
            e.footer(|f| {
                f.text(concat!("Putting an l in front of any command",
                    "(except h4ck and uwu) will use the last message"));
                f
            });
            e
        })
    }).await;
}
