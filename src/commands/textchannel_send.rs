use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};
use serenity::utils::*;

use sqlx;
use crate::ConnectionPool;
use crate::helpers::*;

/// Sends `nice` to a specified channel. Provide a channel as the first argument to set it
/// Usage: `nice <message>` or `nice <channel>`
#[command]
async fn nice(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let test_id = args.single::<String>().unwrap_or_default();

    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(&ctx, &msg, Permissions::MANAGE_MESSAGES).await {

            let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM text_channels WHERE guild_id = $1)", guild_id.0 as i64)
                .fetch_one(pool)
                .await?;
        
            if check.exists.unwrap() {
                sqlx::query!("UPDATE text_channels SET nice_id = $1 WHERE guild_id = $2", channel_id as i64, guild_id.0 as i64)
                    .execute(pool).await?;
            } else {
                textmod_helper::create_channel_row(pool, guild_id.0 as i64, channel_id as i64, None, None).await?;
            }
        
            msg.channel_id.say(&ctx, "Channel sucessfully set!").await?;
        }

        return Ok(())
    }

    if !args.is_empty() {
        msg.channel_id.say(&ctx, "Please execute this command without any arguments").await?;
        return Ok(())
    }

    let channel_num = textmod_helper::get_channel(pool, guild_id, "nice").await?;

    if channel_num == 0 {
        msg.channel_id.say(&ctx, "The Nice channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    ChannelId(channel_num as u64).say(&ctx, format!("Nice - {}", msg.author)).await?;

    Ok(())
}

/// Sends `bruh` to a specified channel. Provide a channel as the first argument to set it
/// Usage: `bruh <message>` or `bruh <channel>`
#[command]
async fn bruh(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let test_id = args.single::<String>().unwrap_or_default();

    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(&ctx, &msg, Permissions::MANAGE_MESSAGES).await {

            let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM text_channels WHERE guild_id = $1)", guild_id.0 as i64)
                .fetch_one(pool)
                .await?;
        
            if check.exists.unwrap() {
                sqlx::query!("UPDATE text_channels SET bruh_id = $1 WHERE guild_id = $2", channel_id as i64, guild_id.0 as i64)
                    .execute(pool).await?;
            } else {
                textmod_helper::create_channel_row(pool, guild_id.0 as i64, None, channel_id as i64, None).await?;
            }
        
            msg.channel_id.say(&ctx, "Channel sucessfully set!").await?;
        }

        return Ok(())
    }

    if !args.is_empty() {
        msg.channel_id.say(&ctx, "Please execute this command without any arguments").await?;
        return Ok(())
    }

    let channel_num = textmod_helper::get_channel(pool, guild_id, "bruh").await?;

    if channel_num == 0 {
        msg.channel_id.say(&ctx, "The Bruh channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx, "***BRUH MOMENT***").await?;
    ChannelId(channel_num as u64).say(&ctx, format!("A bruh moment has been declared by {}", msg.author)).await?;

    Ok(())
}

/// Quotes yourself or a specified user
/// Usage: `quote <user mention> <content>` or `quote <content>`
#[command]
async fn quote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let mut self_quote = true;

    let test_id = args.single::<String>().unwrap_or_default();

    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(&ctx, &msg, Permissions::MANAGE_MESSAGES).await {

            let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM text_channels WHERE guild_id = $1)", guild_id.0 as i64)
            .fetch_one(pool)
                .await?;
        
            if check.exists.unwrap() {
                sqlx::query!("UPDATE text_channels SET quote_id = $1 WHERE guild_id = $2", channel_id as i64, guild_id.0 as i64)
                    .execute(pool).await?;
            } else {
                textmod_helper::create_channel_row(pool, guild_id.0 as i64, None, channel_id as i64, None).await?;
            }
        
            msg.channel_id.say(&ctx, "Channel sucessfully set!").await?;
        }

        return Ok(())
    }

    if parse_username(&test_id).is_some() {
        self_quote = false;
    }

    if args.is_empty() || test_id == "" {
        msg.channel_id.say(&ctx, "Please provide the quote (with author if you are quoting someone else)").await?;
        return Ok(())
    }

    let channel_num = textmod_helper::get_channel(pool, guild_id, "quote").await?;

    if channel_num == 0 {
        msg.channel_id.say(&ctx, "The Quote channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    if self_quote {
        ChannelId(channel_num as u64).say(&ctx, format!("\"{}\" \n - {}", args.rest(), msg.author)).await?;
    }
    else {
        ChannelId(channel_num as u64).say(&ctx, format!("\"{}\" \n - {}", args.rest(), msg.mentions[0])).await?;
    }

    Ok(())
}