use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::{channel::Reaction, prelude::ReactionType},
};

use crate::reactions::starboard;

pub async fn dispatch_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    if let ReactionType::Unicode(emoji) = &reaction.emoji {
        if emoji == "⭐" {
            starboard::quote_reaction(ctx, reaction, remove).await?;
        }
    }

    Ok(())
}
