use std::sync::Arc;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};
use crate::{
    ConnectionPool,
    PubCreds,
    CommandNameMap,
    PrefixMap,
    helpers::permissions_helper, 
    helpers::database_helper, 
};

/// Sets the prefix for the server using the first message argument
/// Execute this command with no arguments to get the current prefix
#[command]
#[sub_commands(restore)]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let (pool, prefixes, default_prefix) = {
        let data = ctx.data.read().await;

        let pool = data.get::<ConnectionPool>().cloned().unwrap();
        let prefixes = data.get::<PrefixMap>().unwrap().clone();
        let default_prefix = data.get::<PubCreds>().unwrap().get("default prefix").cloned().unwrap();

        (pool, prefixes, default_prefix)
    };
    let guild_id = msg.guild_id.unwrap();
    let guild_name = msg.guild(ctx).await.unwrap().name;

    if args.is_empty() {
        let cur_prefix = match prefixes.get(&guild_id) {
            Some(prefix_guard) => prefix_guard.value().to_owned(),
            None => default_prefix
        };

        msg.channel_id.say(ctx, format!("My prefix for `{}` is `{}`", guild_name, cur_prefix)).await?;
        return Ok(())
    }
    
    if !permissions_helper::check_permission(ctx, msg, None, false).await? {
        return Ok(())
    }

    let new_prefix = args.single::<String>().unwrap();

    if new_prefix == default_prefix {
        sqlx::query!("UPDATE guild_info SET prefix = null WHERE guild_id = $1", guild_id.0 as i64)
            .execute(&pool).await?;
        
        prefixes.remove(&guild_id);
    } else {
        sqlx::query!("UPDATE guild_info SET prefix = $1 WHERE guild_id = $2", new_prefix, guild_id.0 as i64)
            .execute(&pool).await?;

        prefixes.insert(guild_id, new_prefix.to_owned());
    }

    msg.channel_id.say(ctx, format!("My new prefix is `{}` for `{}`!", new_prefix, guild_name)).await?;

    Ok(())
}

#[command]
#[required_permissions("ADMINISTRATOR")]
async fn resetprefix(ctx: &Context, msg: &Message) -> CommandResult {
    let (pool, prefixes, default_prefix) = {
        let data = ctx.data.read().await;

        let pool = data.get::<ConnectionPool>().cloned().unwrap();
        let prefixes = data.get::<PrefixMap>().unwrap().clone();
        let default_prefix = data.get::<PubCreds>().unwrap().get("default prefix").cloned().unwrap();

        (pool, prefixes, default_prefix)
    };


    let guild_id = msg.guild_id.unwrap();

    if prefixes.contains_key(&guild_id) {
        prefixes.remove(&guild_id);

        sqlx::query!("UPDATE guild_info SET prefix = null WHERE guild_id = $1", guild_id.0 as i64)
            .execute(&pool).await?;
    }

    msg.channel_id.say(ctx, format!("Reset the prefix back to {}", default_prefix)).await?;

    Ok(())
}

#[command]
#[owners_only(true)]
async fn restore(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = ctx.data.read().await
        .get::<ConnectionPool>().cloned().unwrap();

    {
        let mut data = ctx.data.write().await;
        let new_prefixes = database_helper::fetch_prefixes(&pool).await?;

        data.insert::<PrefixMap>(Arc::new(new_prefixes));
    }

    msg.channel_id.say(ctx, "Prefixes successfully restored!").await?;

    Ok(())
}

pub async fn prefix_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "prefix: Gets the server's current prefix \n\n",
        "prefix (characters): Sets the server's prefix (Can be one or multiple characters)");
    
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
    let (pool, command_names) = {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().cloned().unwrap();
        let command_names = data.get::<CommandNameMap>().cloned().unwrap();

        (pool, command_names)
    };


    if command_names.contains(&command_name) {
        msg.channel_id.say(ctx, "This command is already hardcoded! Please choose a different name!").await?;
        return Ok(())
    }

    let guild_id = msg.guild_id.unwrap().0 as i64;

    sqlx::query!("INSERT INTO commands(guild_id, name, content)
            VALUES($1, $2, $3)
            ON CONFLICT (guild_id, name)
            DO UPDATE
            SET content = EXCLUDED.content",
            guild_id, command_name, args.rest())
        .execute(&pool).await?;

    msg.channel_id.say(ctx, format!("Command `{}` sucessfully set!", command_name)).await?;

    Ok(())
}

// Subcommand used to remove a custom command
#[command]
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
    let pool = ctx.data.read().await
        .get::<ConnectionPool>().cloned().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let mut command_map: Vec<String> = Vec::new();

    let command_data = sqlx::query!("SELECT name, content FROM commands WHERE guild_id = $1", guild_id)
        .fetch_all(&pool).await?;

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
    let content = concat!(
        "set <name> <content>: Sets a new custom command, {user} is replaced with a mention \n\n",
        "remove <name>: Removes an existing custom command \n\n",
        "list: Lists all custom commands in the server");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Custom Command Help");
            e.description("Description: Custom command configuration (For administrators only!)");
            e.field("Commands", content, false);
            e
        })
    }).await;
}