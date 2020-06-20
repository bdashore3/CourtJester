use crate::{
    structures::Context,
    helpers::command_utils::*,
};

use twilight::model::{
    channel::message::Message,
    guild::Permissions, id::{UserId, GuildId}
};

pub async fn check_permission(ctx: &Context<'_>, msg: &Message, permission: Permissions) -> bool {

    let permissions = compile_permissions(ctx, msg.guild_id.unwrap(), msg.author.id).await.unwrap();
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

pub async fn compile_permissions(ctx: &Context<'_>, guild_id: GuildId, user_id: UserId) -> Result<Permissions, Box<dyn std::error::Error>> {
    let member = ctx.cache.member(guild_id, user_id).await.unwrap().unwrap();

    let guild = ctx.cache.guild(guild_id).await?.unwrap();

    let mut permissions = Permissions::empty();

    if user_id == guild.owner_id {
        permissions = Permissions::all();
    } else {
        for role_id in &member.roles {
            let role = ctx.cache.role(*role_id).await?.unwrap();
    
            permissions |= role.permissions;
        }
    }

    Ok(permissions)
}