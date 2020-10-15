use serenity::{
    prelude::*,
    model::prelude::*
};

use crate::structures::errors::{JesterError, PermissionType};

pub async fn check_permission(ctx: &Context, msg: &Message, user_id: Option<UserId>, check_admin: bool) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();
    let permissions = channel.permissions_for_user(ctx, user_id.unwrap_or(msg.author.id)).await?;

    if permissions.administrator() {
        return Ok(true)
    } 
    else if check_admin && user_id.is_none() {
        msg.channel_id.say(ctx, JesterError::PermissionError(PermissionType::SelfPerm("administrator"))).await?;

        return Ok(false)
    } 
    else {
        if user_id.is_none() && !permissions.manage_messages() {
            msg.channel_id.say(ctx, JesterError::PermissionError(PermissionType::SelfPerm("moderator"))).await?;
        }

        return Ok(permissions.manage_messages())
    }
}
