use serenity::{
    model::channel::{Reaction, ReactionType},
    client::Context, framework::standard::CommandResult
};
use crate::reactions::starboard;

pub async fn dispatch_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    if let ReactionType::Unicode(name) = &reaction.emoji {
        match name.as_str() {
            "\u{2b50}" => starboard::quote_reaction(ctx, reaction, remove).await?, 
            _ => {}
        }
    }

    Ok(())
}
