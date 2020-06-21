use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer,
    helpers::permissions_helper
};

use twilight::{ 
    model::{
        channel::message::Message,
        guild::Permissions, 
        id::{ChannelId, GuildId},
    },
    builders::embed::EmbedBuilder
};
use sqlx;
use sqlx::PgPool;

pub async fn nice(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    let mut channel_tuple: (i64, bool) = (0, false);
    let guild_id = msg.guild_id.unwrap();
    
    if string_renderer::get_command_length(&msg.content) == 2 {
        let channel_str = string_renderer::get_message_word(&msg.content, 1);
        channel_tuple = match check_channel(ctx, channel_str).await {
            Ok(tuple) => tuple,
            Err(_) => {
                send_message(ctx.http, msg.channel_id, "Please execute this command without arguments!").await?;
                return Ok(())
            }
        };
    }

    if channel_tuple.1 {
        if permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
            
            let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM text_channels WHERE guild_id = $1)", guild_id.0 as i64)
                .fetch_one(ctx.pool)
                .await?;
                
            if check.exists.unwrap() {
                sqlx::query!("UPDATE text_channels SET nice_id = $1 WHERE guild_id = $2", channel_tuple.0, guild_id.0 as i64)
                    .execute(ctx.pool).await?;
            } else {
                create_channel_row(ctx.pool, guild_id.0 as i64, channel_tuple.0, None, None).await?;
            }

            send_message(ctx.http, msg.channel_id, "Channel successfully set!").await?;
        }

        return Ok(())
    }
                

    let channel_num = get_send_channel(ctx.pool, guild_id, "nice").await?;

    if channel_num == 0 {
        send_message(ctx.http, msg.channel_id, "The Nice channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    let message_url = get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    let send_id = ChannelId::from(channel_num as u64);
    let content = format!("Nice - {}", msg.author.name);

    let mut eb = EmbedBuilder::new();
    eb = eb.title(content);
    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    send_embed(ctx.http, send_id, eb.build()).await?;

    Ok(())
}

pub async fn bruh(ctx: &Context<'_>, msg: &Message) -> CommandResult {
    let mut channel_tuple: (i64, bool) = (0, false);
    let guild_id = msg.guild_id.unwrap();
    
    if string_renderer::get_command_length(&msg.content) == 2 {
        let channel_str = string_renderer::get_message_word(&msg.content, 1);
        channel_tuple = match check_channel(ctx, channel_str).await {
            Ok(tuple) => tuple,
            Err(_) => {
                send_message(ctx.http, msg.channel_id, "Please execute this command without arguments!").await?;
                return Ok(())
            }
        };
    }

    if channel_tuple.1 {
        if permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
            
            let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM text_channels WHERE guild_id = $1)", guild_id.0 as i64)
                .fetch_one(ctx.pool)
                .await?;
                
            if check.exists.unwrap() {
                sqlx::query!("UPDATE text_channels SET bruh_id = $1 WHERE guild_id = $2", channel_tuple.0, guild_id.0 as i64)
                    .execute(ctx.pool).await?;
            } else {
                create_channel_row(ctx.pool, guild_id.0 as i64, None, channel_tuple.0, None).await?;
            }

            send_message(ctx.http, msg.channel_id, "Channel successfully set!").await?;
        }

        return Ok(())
    }
                

    let channel_num = get_send_channel(ctx.pool, guild_id, "bruh").await?;

    if channel_num == 0 {
        send_message(ctx.http, msg.channel_id, "The Bruh channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    let message_url = get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    let send_id = ChannelId::from(channel_num as u64);
    let content = format!("A bruh moment has been declared by <@!{}>", msg.author.id);
    send_message(ctx.http, msg.channel_id, "***BRUH MOMENT***").await?;
    
    let mut eb = EmbedBuilder::new();
    eb = eb.title("Ladies and Gentlemen!");
    eb = eb.description(content);
    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    send_embed(ctx.http, send_id, eb.build()).await?;

    Ok(())
}

async fn create_channel_row(pool: &PgPool, guild_id: i64, 
    nice_id: impl Into<Option<i64>>, bruh_id: impl Into<Option<i64>>, 
    quote_id: impl Into<Option<i64>>) -> Result<(), Box<dyn std::error::Error>> {

    sqlx::query!("INSERT INTO text_channels VALUES($1, $2, $3, $4)", guild_id, nice_id.into().unwrap_or(0), bruh_id.into().unwrap_or(0), quote_id.into().unwrap_or(0))
        .execute(pool).await?;

    Ok(())
}

pub async fn check_channel(ctx: &Context<'_>, channel_str: &str) -> Result<(i64, bool), std::num::ParseIntError> {
    let channel_num = match get_raw_id(channel_str, "channel") {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    let channel_id = ChannelId::from(channel_num);
    let output = match ctx.cache.guild_channel(channel_id).await.unwrap() {
        Some(_) => (channel_num as i64, true),
        None => (0, false)
    };
    
    Ok(output)
}

async fn get_send_channel(pool: &PgPool, guild_id: GuildId, channel_type: &str) -> Result<i64, Box<dyn std::error::Error>>{

    let mut result: i64 = 0;

    let data = sqlx::query!("SELECT nice_id, bruh_id, quote_id FROM text_channels WHERE guild_id = $1", guild_id.0 as i64)
        .fetch_optional(pool)
        .await?;
    
    if let Some(data) = data {
        result = match channel_type {
            "nice" => data.nice_id,
            "bruh" => data.bruh_id,
            "quote" => data.quote_id,
            _ => 0 
        };
    }

    Ok(result)
}