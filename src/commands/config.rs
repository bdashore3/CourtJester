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
    PrefixMap,
    DefaultPrefix,
    helpers::prefix_cache
};

/// Sets the prefix for the server using the first message argument
/// Execute this command with no arguments to get the current prefix
#[command]
#[sub_commands(restore)]
#[only_in("guilds")]
#[required_permissions(Manage_Messages)]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix;
    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();
    let prefixes = data.get::<PrefixMap>().unwrap();
    let default_prefix = data.get::<DefaultPrefix>().unwrap().to_string();
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let guild_name = msg.guild(ctx).await.unwrap().name;

    if args.is_empty() {
        prefix = match prefixes.get(&guild_id) {
            Some(prefix) => prefix.to_string(),
            None => default_prefix
        };

        msg.channel_id.say(ctx, format!("My prefix for `{}` is `{}`", guild_name, prefix)).await?;
        return Ok(())
    }
    
    prefix = args.single::<String>().unwrap();

    prefixes.insert(guild_id, prefix.to_string());
    let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM guild_info WHERE guild_id = $1)", guild_id)
        .fetch_one(pool)
        .await?;

    if check.exists.unwrap() {
        if prefix == default_prefix {
            sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild_id)
                .execute(pool).await?;
        }
        sqlx::query!("UPDATE guild_info SET prefix = $1 WHERE guild_id = $2", prefix.to_string(), guild_id)
            .execute(pool).await?;
    } else {
        sqlx::query!("INSERT INTO guild_info VALUES($1, $2)", guild_id, prefix.to_string())
            .execute(pool)
            .await?;
    }

    msg.channel_id.say(ctx, format!("My new prefix is `{}` for `{}`!", prefix, guild_name)).await?;

    Ok(())
}

#[command]
async fn restore(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    prefix_cache::fetch_prefixes(pool).await?;

    msg.channel_id.say(ctx, "All prefixes sucessfully refreshed from the database!").await?;

    Ok(())
}

/// Custom commands for your server that output a message
/// Usage to set: `command set <name> <content to be said>`
/// Usage to remove: `command remove <name>`
#[command]
#[sub_commands(set, remove)]
#[required_permissions(Administrator)]
async fn command(_: &Context, _: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = args.single::<String>().unwrap();

    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.unwrap();

    sqlx::query!("INSERT INTO commands VALUES($1, $2, $3, $4)", Uuid::new_v4(), guild_id.0 as i64, &command_name, args.rest())
        .execute(pool).await?;

    msg.channel_id.say(ctx, format!("New command {} created!", command_name)).await?;
    
    Ok(())
}

#[command]
async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = args.single::<String>().unwrap();
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();
    let guild_id = msg.guild_id.unwrap();

    sqlx::query!("DELETE FROM commands WHERE guild_id = $1 AND name = $2", guild_id.0 as i64, command_name)
        .execute(pool)
        .await?;

    msg.channel_id.say(ctx, format!("Command {} sucessfully deleted!", command_name)).await?;

    Ok(())
}