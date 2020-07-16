use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};

use sqlx::{self, PgPool, };

use crate::{
    ConnectionPool,
    DefaultPrefix,
    helpers::permissions_helper
};

/// Sets the prefix for the server using the first message argument
/// Execute this command with no arguments to get the current prefix
#[command]
#[only_in("guilds")]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    let default_prefix = data.get::<DefaultPrefix>().unwrap().to_string();
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild_name = msg.guild(ctx).await.unwrap().name;

    if args.is_empty() {
        let cur_prefix = get_prefix(pool, msg.guild_id.unwrap(), default_prefix).await?;

        msg.channel_id.say(ctx, format!("My prefix for `{}` is `{}`", guild_name, cur_prefix)).await?;
        return Ok(())
    }
    
    if !permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
        return Ok(())
    }

    let new_prefix = args.single::<String>().unwrap();

    if new_prefix == default_prefix {
        sqlx::query!("UPDATE guild_info SET prefix = null WHERE guild_id = $1", guild_id)
            .execute(pool).await?;
    }
    else {
        sqlx::query!("UPDATE guild_info SET prefix = $1 WHERE guild_id = $2", new_prefix, guild_id)
            .execute(pool).await?;
    }

    msg.channel_id.say(ctx, format!("My new prefix is `{}` for `{}`!", new_prefix, guild_name)).await?;

    Ok(())
}

pub async fn get_prefix(pool: &PgPool, guild_id: GuildId, default_prefix: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut cur_prefix = default_prefix;
    let guild_data = sqlx::query!("SELECT prefix FROM guild_info WHERE guild_id = $1", guild_id.0 as i64)
        .fetch_optional(pool).await?;
    
    if let Some(guild_data) = guild_data {
        if let Some(prefix) = guild_data.prefix {
            cur_prefix = prefix;
        }
    }

    Ok(cur_prefix)
}

pub async fn prefix_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("prefix: Gets the server's current prefix \n\n");
    content.push_str("prefix <character>: Sets the server's prefix (Can be one or multiple characters)");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Custom Prefix Help");
            e.description("Description: Commands for custom bot prefixes");
            e.field("Commands", content, false);
            e
        })
    }).await;
}

/// Custom commands for your server that output a message
/// Usage to set: `command set <name> <content to be said>`
/// Usage to remove: `command remove <name>`
#[command]
#[sub_commands(set, remove, list)]
async fn command(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Please use one of the subcommands! (set, remove, list)").await?;

    Ok(())
}

// Subcommand to set/update a custom command
#[command]
#[required_permissions(Administrator)]
#[aliases("add")]
#[min_args(2)]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = args.single::<String>().unwrap();
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;

    sqlx::query!("INSERT INTO commands(guild_id, name, content)
            VALUES($1, $2, $3)
            ON CONFLICT (guild_id, name)
            DO UPDATE
            SET content = EXCLUDED.content",
            guild_id, command_name, args.rest())
        .execute(pool).await?;

    msg.channel_id.say(ctx, format!("Command `{}` sucessfully set!", command_name)).await?;

    Ok(())
}

// Subcommand used to remove a custom command
#[command]
#[required_permissions(Administrator)]
#[min_args(1)]
async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = args.single::<String>().unwrap();
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;

    sqlx::query!("DELETE FROM commands WHERE guild_id = $1 AND name = $2", guild_id, command_name)
        .execute(pool)
        .await?;

    msg.channel_id.say(ctx, format!("Command {} sucessfully deleted!", command_name)).await?;

    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let mut command_map: Vec<String> = Vec::new();

    let command_data = sqlx::query!("SELECT name, content FROM commands WHERE guild_id = $1", guild_id)
        .fetch_all(pool).await?;

    for i in command_data {
        command_map.push(i.name);
    }

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Custom commands");
            e.description(format!("```{} \n```", command_map.join(" \n")))
        });
    
        m
    }).await?;

    Ok(())
}

pub async fn command_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("set <name> <content>: Sets a new custom command, {user} is replaced with a mention \n\n");
    content.push_str("remove <name>: Removes an existing custom command \n\n");
    content.push_str("list: Lists all custom commands in the server");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Custom Command Help");
            e.description("Description: Custom command configuration (For administrators only!)");
            e.field("Commands", content, false);
            e
        })
    }).await;
}