use std::borrow::Cow;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::JesterError;

#[command]
async fn avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = if let Ok(user_id) = args.single::<UserId>() {
        match user_id.to_user(ctx).await {
            Ok(user) => Cow::Owned(user),
            Err(_) => {
                msg.channel_id
                    .say(ctx, JesterError::MissingError("User ID/mention"))
                    .await?;

                return Ok(());
            }
        }
    } else if args.is_empty() {
        Cow::Borrowed(&msg.author) 
    } else {
        msg.channel_id
            .say(ctx, JesterError::MissingError("User ID/mention"))
            .await?;

        return Ok(());
    };

    msg.channel_id.say(ctx, user.face()).await?;

    Ok(())
}
