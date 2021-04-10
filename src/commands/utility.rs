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

#[command]
#[aliases("steal")]
#[required_permissions("MANAGE_EMOJIS")]
async fn kang(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let emoji = match args.single::<EmojiIdentifier>() {
        Ok(id) => id,
        Err(_) => {
            msg.channel_id
                .say(ctx, JesterError::MissingError("custom emoji"))
                .await?;

            return Ok(());
        }
    };

    let guild = msg.guild(ctx).await.unwrap();
    if guild.emojis.contains_key(&emoji.id) {
        msg.channel_id
            .say(ctx, "This emoji already exists in this server! Aborting...")
            .await?;

        return Ok(());
    }

    let emoji_url = emoji.url();

    let url_length = emoji_url.len();
    let ext = &emoji_url[url_length - 3..url_length];

    let image_bytes = reqwest::get(&emoji.url()).await?.bytes().await?;
    let encoded_bytes = base64::encode(image_bytes);
    let formatted_bytes = format!("data:image/{};base64,{}", ext, encoded_bytes);

    let name = args.single::<String>().unwrap_or(emoji.name);

    match guild.create_emoji(ctx, &name, &formatted_bytes).await {
        Ok(new_emoji) => {
            msg.channel_id
                .say(
                    ctx,
                    format!("New emoji {} created! {}", new_emoji.name, new_emoji),
                )
                .await?;

            Ok(())
        }
        Err(e) => {
            msg.channel_id.say(
                ctx,
                "Something went wrong with emoji creation. Check your emoji limit? The error message is below."
            ).await?;

            Err(e.into())
        }
    }
}

pub async fn utility_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "avatar (user mention/ID): Gets your own, or the mentioned person's avatar \n\n",
        "kang <emoji> (new name): Steal an emoji from anywhere and load it to your server. Requires the `manage emojis` permission"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Miscellaneous Utility Help");
                e.description("Description: Various utility commands");
                e.field("Commands", content, false);
                e
            })
        })
        .await;
}
