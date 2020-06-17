use crate::{
    structures::Context,
    helpers::command_utils::*,
};

use twilight::model::{
    channel::message::Message,
    guild::Permissions
};

pub async fn check_permission(ctx: &Context<'_>, msg: &Message, permission: Permissions) -> bool {
    let member = ctx.cache.member(msg.guild_id.unwrap(), msg.author.id).await.unwrap().unwrap();

    let mut permissions = Permissions::empty();

    for role_id in &member.roles {
        let role = ctx.cache.role(*role_id).await.unwrap().unwrap();

        permissions |= role.permissions;
    }

    if permissions.contains(permission) {
        return true
    }

    match permission {
        Permissions::MANAGE_MESSAGES => {
            let _ = send_message(ctx.http, msg.channel_id, "You can't execute this command because you're not a moderator!").await;
        },
        _ => {
            let _ = send_message(ctx.http, msg.channel_id, "You can't execute this command!").await;
        }
    };

    false
}