use serenity::prelude::*;
use serenity::model::prelude::*;

pub async fn check_permission(ctx: &Context, msg: &Message, permission: Permissions) -> bool {

    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();

    let permissions = channel.permissions_for_user(ctx, msg.author.id).await.unwrap();

    if permissions.contains(permission) {
        return true
    }

    match permission {
        Permissions::ADMINISTRATOR => {
            let _ = msg.channel_id.say(ctx, 
                "You can't execute this command because you aren't an administrator!").await;
        }
        Permissions::MANAGE_MESSAGES => {
            let _ = msg.channel_id.say(ctx, 
                "You can't execute this command because you aren't a moderator (Manage Messages permission)!").await;
        }
        _ => {
            let _ = msg.channel_id.say(ctx, "You can't execute this command!").await;
        }
    }
    
    return false;
}

pub async fn _check_mentioned_permission(ctx: &Context, msg: &Message, user: UserId, permission: Permissions) -> bool {

    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();

    let permissions = channel.permissions_for_user(ctx, user).await.unwrap();

    if permissions.contains(permission) {
        return true;
    }

    let _ = msg.channel_id.say(ctx, "You can't execute this command!").await;
    return false;
}