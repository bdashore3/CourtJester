use serenity::prelude::*;
use serenity::model::prelude::*;

pub async fn check_permission(ctx: &Context, msg: &Message, permission: Permissions) -> bool {

    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();

    let permissions = channel.permissions_for_user(ctx, msg.author.id).await.unwrap();

    if permissions.contains(permission) {
        return true
    }

    let _ = msg.channel_id.say(ctx, "You can't execute this command!").await;
    return false;
}

pub async fn check_mentioned_permission(ctx: &Context, msg: &Message, user: UserId, permission: Permissions) -> bool {

    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();

    let permissions = channel.permissions_for_user(ctx, user).await.unwrap();

    if permissions.contains(permission) {
        return true;
    }

    let _ = msg.channel_id.say(ctx, "You can't execute this command!").await;
    return false;
}