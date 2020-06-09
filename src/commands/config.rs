use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};

use sqlx;
use uuid::Uuid;

use crate::{
    ConnectionPool,
    GuildMap,
    DefaultPrefix,
    helpers::guild_cache,
    helpers::permissions_helper
};

/// Sets the prefix for the server using the first message argument
/// Execute this command with no arguments to get the current prefix
#[command]
#[only_in("guilds")]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix;
    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();
    let guild_map = data.get::<GuildMap>().unwrap();
    let default_prefix = data.get::<DefaultPrefix>().unwrap().to_string();
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild_name = msg.guild(ctx).await.unwrap().name;
    let mut guild_data = guild_map.get_mut(&guild_id).unwrap();

    if args.is_empty() {
        prefix = match guild_data.prefix.as_str() {
            "" => default_prefix.to_string(),
            _ => guild_data.prefix.to_string()
        };

        msg.channel_id.say(ctx, format!("My prefix for `{}` is `{}`", guild_name, prefix)).await?;
        return Ok(())
    }
    
    if !permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
        return Ok(())
    }

    prefix = args.single::<String>().unwrap();

    if prefix == default_prefix {
        guild_data.prefix = "".to_string();
    }
    else {
        guild_data.prefix = prefix.to_string();
    }

    if prefix == default_prefix {
        sqlx::query!("UPDATE guild_info SET prefix = null WHERE guild_id = $1", guild_id)
            .execute(pool).await?;
    }
    sqlx::query!("UPDATE guild_info SET prefix = $1 WHERE guild_id = $2", prefix.to_string(), guild_id)
        .execute(pool).await?;

    msg.channel_id.say(ctx, format!("My new prefix is `{}` for `{}`!", prefix, guild_name)).await?;

    Ok(())
}

// Dev command: Refreshes guild data from the Database
#[command]
#[owners_only(true)]
async fn restore(ctx: &Context, msg: &Message) -> CommandResult {

    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    guild_cache::fetch_guild_data(pool).await?;

    msg.channel_id.say(ctx, "All prefixes sucessfully refreshed from the database!").await?;

    Ok(())
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
    let guild_map = data.get::<GuildMap>().unwrap();
    let guild_data = guild_map.get_mut(&guild_id).unwrap();

    guild_data.commands.insert(command_name.to_string(), args.rest().to_string());

    let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM commands WHERE guild_id = $1 AND name = $2)", guild_id, &command_name)
        .fetch_one(pool)
        .await?;

    if check.exists.unwrap() {
        sqlx::query!("UPDATE commands SET content = $1 WHERE guild_id = $2", args.rest(), guild_id)
            .execute(pool).await?;
        msg.channel_id.say(ctx, format!("Command {} updated!", command_name)).await?;
    } else {
        sqlx::query!("INSERT INTO commands VALUES($1, $2, $3, $4)", Uuid::new_v4(), guild_id, &command_name, args.rest())
            .execute(pool).await?;
        msg.channel_id.say(ctx, format!("New command {} created!", command_name)).await?;
    }
    
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
    let guild_map = data.get::<GuildMap>().unwrap();
    let guild_data = guild_map.get_mut(&guild_id).unwrap();

    guild_data.commands.remove(&command_name);

    sqlx::query!("DELETE FROM commands WHERE guild_id = $1 AND name = $2", guild_id, command_name)
        .execute(pool)
        .await?;

    msg.channel_id.say(ctx, format!("Command {} sucessfully deleted!", command_name)).await?;

    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild_map = data.get::<GuildMap>().unwrap();
    let guild_data = guild_map.get(&guild_id).unwrap();

    let mut message_content: Vec<String> = Vec::new();
    for i in guild_data.commands.clone() {
        message_content.push(i.0)
    };

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Custom commands");
            e.description(format!("```{} \n```", message_content.join(" \n")))
        });
    
        m
    }).await?;

    Ok(())
}