use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer,
    helpers::permissions_helper,
    commands::textchannel_send::*
};
use twilight::{
    model::{guild::Permissions, channel::{ReactionType, Message}, gateway::payload::{ReactionAdd, MessageCreate}}, standby::Standby
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


pub async fn dispatch_custom_command(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    if string_renderer::get_command_length(&msg.content) < 2 {
        return Ok(())
    }

    let subcommand = string_renderer::get_message_word(&msg.content, 1);

    match subcommand {
        "set" => {
            if !permissions_helper::check_permission(ctx, msg, Permissions::ADMINISTRATOR).await {
                return Ok(())
            }
            set_custom_command(ctx, msg).await?;
        },
        "remove" => {
            if !permissions_helper::check_permission(ctx, msg, Permissions::ADMINISTRATOR).await {
                return Ok(())
            }
            remove_custom_command(ctx, msg).await?;
        }
        _ => {}
    }

    Ok(())
}

pub async fn set_custom_command(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    let command_name = string_renderer::get_message_word(&msg.content, 2);
    let command_message = string_renderer::join_string(&msg.content, 2);

    sqlx::query!("INSERT INTO commands(guild_id, name, content) 
            VALUES($1, $2, $3) 
            ON CONFLICT (guild_id, name) 
            DO UPDATE
            SET content = EXCLUDED.content", 
            msg.guild_id.unwrap().0 as i64, command_name, command_message)
        .execute(ctx.pool).await?;

    send_message(ctx.http, msg.channel_id, format!("Command `{}` sucessfully set!", command_name)).await?;

    Ok(())
}

pub async fn remove_custom_command(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    let command_name = string_renderer::get_message_word(&msg.content, 2);

    sqlx::query!("DELETE FROM commands WHERE guild_id = $1 AND name = $2", msg.guild_id.unwrap().0 as i64, command_name)
        .execute(ctx.pool)
        .await?;

    send_message(ctx.http, msg.channel_id, format!("Command `{}` sucessfully deleted!", command_name)).await?;

    Ok(())
}

pub async fn starbot(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    let subcommand = string_renderer::get_message_word(&msg.content, 1);
    match subcommand {
        "threshold" => {
            let new_threshold = string_renderer::get_message_word(&msg.content, 2).parse::<i32>();
            sqlx::query!("UPDATE guild_info SET starbot_threshold = $1 WHERE guild_id = $2", new_threshold.unwrap(), msg.guild_id.unwrap().0 as i64)
                .execute(ctx.pool).await?;
            
            send_message(ctx.http, msg.channel_id, "New threshold sucessfully set!").await?;
        },
        "channel" => {
            let new_channel_string = string_renderer::get_message_word(&msg.content, 2);
            let new_channel = get_raw_id(new_channel_string, "channel");

            sqlx::query!("INSERT INTO text_channels VALUES($1, $2, null, null)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET quote_id = $2",
                        msg.guild_id.unwrap().0 as i64, new_channel.unwrap() as i64)
                        .execute(ctx.pool).await?;
            
            send_message(ctx.http, msg.channel_id, "New starbot channel sucessfully set!").await?;
        }
        _ => {}
    }

    Ok(())
}