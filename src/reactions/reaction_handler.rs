use serenity::{client::Context, framework::standard::CommandResult, model::channel::Reaction};

use crate::reactions::starboard;

pub async fn dispatch_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    if reaction.emoji.as_data() == "⭐" {
        starboard::quote_reaction(ctx, reaction, remove).await?;
    }

    Ok(())
}
