use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils,
    helpers::string_renderer
};
use twilight::model::channel::{GuildChannel, message::Message};
use rand::prelude::*;

pub async fn mock(ctx: &Context<'_>, msg: Message, last: bool) -> CommandResult {
    let mut input = "".to_string();

    if last {
        let wrapped_channel = ctx.cache.guild_channel(msg.channel_id).await?.unwrap();

        if let GuildChannel::Text(ref channel) = *wrapped_channel {
            println!("called!");
            let last_message = channel.last_message_id.unwrap();
            input = last_message.0.to_string();
        }
    } else {
        input = string_renderer::join_string(&msg.content);
    }

    let mut mock_string = String::with_capacity(input.len());

    for x in input.chars() {
        if random() {
            mock_string.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            mock_string.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
    }

    command_utils::send_message(ctx.http, msg.channel_id, &mock_string).await?;

    Ok(())
}