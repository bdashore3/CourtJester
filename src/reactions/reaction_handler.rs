use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::channel::{Reaction, ReactionType},
};

use crate::reactions::starboard;

pub async fn dispatch_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    if let ReactionType::Unicode(name) = &reaction.emoji {
        #[allow(clippy::single_match)]
        match name.as_str() {
            "\u{2b50}" => starboard::quote_reaction(ctx, reaction, remove).await?,
            _ => {}
        }
    }

    Ok(())
}
