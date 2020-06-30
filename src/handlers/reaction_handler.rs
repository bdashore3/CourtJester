use crate::structures::Context;
use crate::reactions::{
    starbot::*
};
use twilight::model::channel::{Reaction, ReactionType};

pub async fn dispatch_reaction(ctx: &Context<'_>, reaction: &Reaction, remove: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let ReactionType::Unicode { name } = &reaction.emoji {
        match name.as_str() {
            "\u{2b50}" => {
                quote_reaction(ctx, reaction, remove).await?
            },
            _ => {}
        }
    }
    Ok(())
}