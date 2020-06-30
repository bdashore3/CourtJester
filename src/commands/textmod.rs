use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer
};
use twilight::model::{
    id::{MessageId, ChannelId}, 
    channel::message::Message
};
use rand::prelude::*;

pub async fn get_input_string(ctx: &Context<'_>, content: &str, 
        channel_id: ChannelId, message_id: MessageId, last: bool) -> Result<String, Box<dyn std::error::Error>> {
    if last {
        let last_message = get_last_message(ctx, channel_id, message_id).await?;
        return Ok(last_message.content)
    } else {
        let input = string_renderer::join_string(content, 1);
        return Ok(input)
    }
}

pub async fn mock(ctx: &Context<'_>, msg: &Message, last: bool) -> CommandResult {
    let input = get_input_string(ctx, &msg.content, msg.channel_id, msg.id, last).await?;

    let mut mock_string = String::with_capacity(input.len());

    for x in input.chars() {
        if random() {
            mock_string.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            mock_string.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
    }

    send_message(ctx.http, msg.channel_id, mock_string).await?;
    
    Ok(())
}

pub async fn inv(ctx: &Context<'_>, msg: &Message, last: bool) -> CommandResult {
    let input = get_input_string(ctx, &msg.content, msg.channel_id, msg.id, last).await?;
    let mut inv_string = String::with_capacity(input.len());

    for x in input.chars() {
        if x.is_uppercase() {
            inv_string.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
        else if x.is_lowercase() {
            inv_string.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            inv_string.push(x)
        }
    }

    send_message(ctx.http, msg.channel_id, inv_string).await?;

    Ok(())
}

pub async fn upp(ctx: &Context<'_>, msg: &Message, last: bool) -> CommandResult {
    let upp_string = get_input_string(ctx, &msg.content, msg.channel_id, msg.id, last).await?.to_uppercase();

    send_message(ctx.http, msg.channel_id, upp_string).await?;

    Ok(())
}

pub async fn low(ctx: &Context<'_>, msg: &Message, last: bool) -> CommandResult {
    let low_string = get_input_string(ctx, &msg.content, msg.channel_id, msg.id, last).await?.to_lowercase();

    send_message(ctx.http, msg.channel_id, low_string).await?;

    Ok(())
}

pub async fn space(ctx: &Context<'_>, msg: &Message, last: bool, biggspace: bool) -> CommandResult {
    let input = get_input_string(ctx, &msg.content, msg.channel_id, msg.id, last).await?;
    let pass_string: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    let output = pass_string.split("").map(|x|
        if rand::random() {
            if biggspace {
                format!("{}            ", x)
            }

            else {
                format!("{}    ", x)
            }

        } else {
            if biggspace {
                format!("{}     ", x)
            }

            else {
                format!("{}  ", x)
            }
        }).collect::<String>();
    
    send_message(ctx.http, msg.channel_id, output).await?;

    Ok(())
}