use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer,
    helpers::permissions_helper
};
use twilight::{
    model::{guild::Permissions, channel::Message}
};
use sqlx;
use sqlx::PgPool;

pub async fn handle_prefix(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    if !permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
        return Ok(())
    }

    let guild = ctx.cache.guild(msg.guild_id.unwrap()).await?.unwrap();
    let default_prefix = ctx.data.get("default_prefix").unwrap();

    if string_renderer::get_command_length(&msg.content) < 2 {
        let cur_prefix = get_prefix(&ctx.pool, msg.guild_id.unwrap().0 as i64, default_prefix.to_string()).await?;
        send_message(ctx.http, msg.channel_id, 
            format!("My prefix for `{}` is `{}`", guild.name, cur_prefix)).await?;

        return Ok(())
    }

    let new_prefix = string_renderer::get_message_word(&msg.content, 1);

    if new_prefix == default_prefix {
        sqlx::query!("UPDATE guild_info SET prefix = null WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
            .execute(ctx.pool).await?;
    }
    else {
        sqlx::query!("UPDATE guild_info SET prefix = $1 WHERE guild_id = $2", new_prefix, msg.guild_id.unwrap().0 as i64)
            .execute(ctx.pool).await?;
    }

    send_message(ctx.http, msg.channel_id, format!("My new prefix for `{}` is `{}`", guild.name, new_prefix)).await?;

    Ok(())
}

pub async fn get_prefix(pool: &PgPool, guild_id: i64, default_prefix: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut cur_prefix = default_prefix;
    let guild_data = sqlx::query!("SELECT prefix FROM guild_info WHERE guild_id = $1", guild_id)
        .fetch_optional(pool).await?;
    
    if let Some(guild_data) = guild_data {
        if let Some(prefix) = guild_data.prefix {
            cur_prefix = prefix;
        }
    }

    Ok(cur_prefix)
}
