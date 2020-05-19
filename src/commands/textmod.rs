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

pub fn get_mock_string(input: &str) -> String {

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