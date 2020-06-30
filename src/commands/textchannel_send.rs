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
        id::{ChannelId, GuildId, UserId},
    },
    builders::embed::EmbedBuilder
};
use sqlx;
use sqlx::PgPool;

struct TextChannels {
    nice_id: Option<i64>,
    bruh_id: Option<i64>,
    quote_id: Option<i64>
}

pub async fn nice(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let mut channel_tuple: (i64, bool) = (0, false);
    let guild_id = msg.guild_id.unwrap();
    
    if string_renderer::get_command_length(&msg.content) == 2 {
        let channel_str = string_renderer::get_message_word(&msg.content, 1);
        channel_tuple = match check_channel(ctx, channel_str).await {
            Ok(tuple) => tuple,
            Err(_) => {
                send_message(&ctx.http, msg.channel_id, "Please execute this command without arguments!").await?;
                return Ok(())
            }
        };
    }

    if channel_tuple.1 {
        if permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
            
            insert_or_update(ctx.pool.as_ref(), guild_id, "nice", channel_tuple.0).await?;
            send_message(&ctx.http, msg.channel_id, "Channel successfully set!").await?;
        }

        return Ok(())
    }
                

    let channel_data = get_channels(ctx.pool.as_ref(), guild_id).await?;

    if channel_data.nice_id.is_none() {
        send_message(&ctx.http, msg.channel_id, "The Nice channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    let message_url = get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    let send_id = ChannelId::from(channel_data.nice_id.unwrap() as u64);

    let mut eb = EmbedBuilder::new();
    eb = eb.title(format!("Nice - {}", msg.author.name));
    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    Ok(send_embed(&ctx.http, send_id, eb.build()).await?)
}

pub async fn bruh(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let mut channel_tuple: (i64, bool) = (0, false);
    let guild_id = msg.guild_id.unwrap();
    
    if string_renderer::get_command_length(&msg.content) == 2 {
        let channel_str = string_renderer::get_message_word(&msg.content, 1);
        channel_tuple = match check_channel(ctx, channel_str).await {
            Ok(tuple) => tuple,
            Err(_) => {
                send_message(&ctx.http, msg.channel_id, "Please execute this command without arguments!").await?;
                return Ok(())
            }
        };
    }

    if channel_tuple.1 {
        if permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
            
            insert_or_update(ctx.pool.as_ref(), guild_id, "bruh", channel_tuple.0).await?;

            send_message(&ctx.http, msg.channel_id, "Channel successfully set!").await?;
        }

        return Ok(())
    }
                

    let channel_data = get_channels(ctx.pool.as_ref(), guild_id).await?;

    if channel_data.bruh_id.is_none() {
        send_message(&ctx.http, msg.channel_id, "The Bruh channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }

    let message_url = get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    let send_id = ChannelId::from(channel_data.bruh_id.unwrap() as u64);
    send_message(&ctx.http, msg.channel_id, "***BRUH MOMENT***").await?;
    
    let mut eb = EmbedBuilder::new();
    eb = eb.title("Ladies and Gentlemen!");
    eb = eb.color(0x22dee4);
    eb = eb.description(format!("A bruh moment has been declared by <@!{}>", msg.author.id));
    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    send_embed(&ctx.http, send_id, eb.build()).await?;

    Ok(())
}

pub async fn quote(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let mut channel_tuple: (i64, bool) = (0, false);
    let guild_id = msg.guild_id.unwrap();

    let starbot_data = sqlx::query!("SELECT starbot_threshold FROM guild_info WHERE guild_id = $1", guild_id.0 as i64)
        .fetch_one(ctx.pool.as_ref()).await?;
    
    if starbot_data.starbot_threshold.is_some() {
        send_message(&ctx.http, msg.channel_id, "You can't use the quote command because starbot is enabled in this server!").await?;
        return Ok(())
    }
    
    if string_renderer::get_command_length(&msg.content) == 2 {
        let channel_str = string_renderer::get_message_word(&msg.content, 1);
        channel_tuple = match check_channel(ctx, channel_str).await {
            Ok(tuple) => tuple,
            Err(_) => {
                send_message(&ctx.http, msg.channel_id, "Please execute this command without arguments!").await?;
                return Ok(())
            }
        };
    }

    if channel_tuple.1 {
        if permissions_helper::check_permission(ctx, msg, Permissions::MANAGE_MESSAGES).await {
            
            insert_or_update(ctx.pool.as_ref(), msg.guild_id.unwrap(), "quote", channel_tuple.0).await?;

            send_message(&ctx.http, msg.channel_id, "Channel successfully set!").await?;
        }

        return Ok(())
    }
                

    let channel_data = get_channels(ctx.pool.as_ref(), guild_id).await?;

    if channel_data.quote_id.is_none() {
        send_message(&ctx.http, msg.channel_id, "The Quotes channel isn't set! Please specify a channel!").await?;
        return Ok(())
    }


    let message_url = get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    let send_id = ChannelId::from(channel_data.quote_id.unwrap() as u64);

    let mut eb = EmbedBuilder::new();
    eb = eb.color(0xfabe21);


    if msg.mentions.values().len() < 1 {
        let author_avatar = match msg.author.avatar.as_ref() {
            Some(avatar_id) => {
                get_avatar_url(msg.author.id, avatar_id)
            }
            None => {
                get_default_avatar_url(&msg.author.discriminator)
            }
        };

        eb = eb.author().name(&msg.author.name).icon_url(author_avatar).commit();
        eb = eb.description(string_renderer::join_string(&format!("{}", &msg.content), 0));
    }
    else {
        let given_id = string_renderer::get_message_word(&msg.content, 1);
        let quote_user = msg.mentions.get(&UserId::from(get_raw_id(given_id, "user").unwrap())).unwrap();
        let quote_user_avatar = match quote_user.avatar.as_ref() {
            Some(avatar_id) => {
                get_avatar_url(quote_user.id, avatar_id)
            }
            None => {
                get_default_avatar_url(&quote_user.discriminator)
            }
        };

        eb = eb.author().name(&quote_user.name).icon_url(quote_user_avatar).commit();
        eb = eb.description(string_renderer::join_string(&format!("{}", &msg.content), 1));
    }

    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    send_embed(&ctx.http, send_id, eb.build()).await?;

    Ok(())
}

async fn get_channels(pool: &PgPool, guild_id: GuildId) -> Result<TextChannels, Box<dyn std::error::Error>>{

    let data = sqlx::query_as!(TextChannels, "SELECT nice_id, bruh_id, quote_id FROM text_channels WHERE guild_id = $1", guild_id.0 as i64)
        .fetch_one(pool).await?;

    Ok(data)
}

async fn insert_or_update(pool: &PgPool, guild_id: GuildId, channel_type: &str, channel_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    match channel_type {
        "nice" => {
            sqlx::query!("INSERT INTO text_channels VALUES($1, $2, null, null)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET nice_id = $2",
                        guild_id.0 as i64, channel_id)
                        .execute(pool).await?;
        },
        "bruh" => {
            sqlx::query!("INSERT INTO text_channels VALUES($1, null, $2, null)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET bruh_id = $2",
                        guild_id.0 as i64, channel_id)
                        .execute(pool).await?;
        },
        "quote" => {
            sqlx::query!("INSERT INTO text_channels VALUES($1, null, null, $2)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET quote_id = $2",
                        guild_id.0 as i64, channel_id)
                        .execute(pool).await?;
        },
        _ => {}
    }

    Ok(())
}

pub async fn check_channel(ctx: &Context, channel_str: &str) -> Result<(i64, bool), std::num::ParseIntError> {
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