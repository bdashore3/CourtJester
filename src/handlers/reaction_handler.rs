use crate::structures::{CommandResult, Context};
use crate::reactions::{
    starbot::*
};
use twilight::model::channel::{Reaction, ReactionType};

pub async fn dispatch_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult<()> {
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

