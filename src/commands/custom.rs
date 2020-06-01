use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};

use sqlx;
use crate::ConnectionPool;
use uuid::Uuid;

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