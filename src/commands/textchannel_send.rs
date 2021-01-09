use rand::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::{parse_channel, parse_username},
};
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

use crate::{
    helpers::{command_utils, permissions_helper},
    ConnectionPool,
};

struct TextChannels {
    nice_id: Option<i64>,
    bruh_id: Option<i64>,
    quote_id: Option<i64>,
}

/// Sends `nice` to a specified channel. Provide a channel as the first argument to set it
/// Usage: `nice <message>` or `nice <channel>`
#[command]
async fn nice(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let test_id = args.single::<String>().unwrap_or_default();

    let pool = ctx
        .data
        .read()
        .await
        .get::<ConnectionPool>()
        .cloned()
        .unwrap();

    let check = sqlx::query!(
        "SELECT EXISTS(SELECT nice_id FROM text_channels WHERE guild_id = $1)",
        guild_id.0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(ctx, &msg, None, false).await? {
            if check.exists.unwrap() {
                sqlx::query!(
                    "UPDATE text_channels SET nice_id = $1 WHERE guild_id = $2",
                    channel_id as i64,
                    guild_id.0 as i64
                )
                .execute(&pool)
                .await?;
            } else {
                insert_or_update(&pool, guild_id, "nice", channel_id as i64).await?;
            }

            msg.channel_id.say(ctx, "Channel sucessfully set!").await?;
        }

        return Ok(());
    }

    if !args.is_empty() {
        msg.channel_id
            .say(ctx, "Please execute this command without any arguments")
            .await?;
        return Ok(());
    }

    if !check.exists.unwrap() {
        msg.channel_id
            .say(ctx, "The Nice channel isn't set! Please specify a channel!")
            .await?;
        return Ok(());
    }

    let channel_num = get_channels(&pool, guild_id).await?;

    if channel_num.nice_id.is_none() {
        msg.channel_id
            .say(ctx, "The Nice channel isn't set! Please specify a channel!")
            .await?;
        return Ok(());
    }

    let message_url = command_utils::get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);
    ChannelId(channel_num.nice_id.unwrap() as u64)
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x290e05);
                e.title(format!("Nice - {}", msg.author.name));
                e.field("Source", format!("[Jump!]({})", message_url), false)
            })
        })
        .await?;

    Ok(())
}

/// Sends `bruh` to a specified channel. Provide a channel as the first argument to set it
/// Usage: `bruh <message>` or `bruh <channel>`
#[command]
async fn bruh(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let test_id = args.single::<String>().unwrap_or_default();

    let pool = ctx
        .data
        .read()
        .await
        .get::<ConnectionPool>()
        .cloned()
        .unwrap();

    let check = sqlx::query!(
        "SELECT EXISTS(SELECT bruh_id FROM text_channels WHERE guild_id = $1)",
        guild_id.0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(ctx, msg, None, false).await? {
            if check.exists.unwrap() {
                sqlx::query!(
                    "UPDATE text_channels SET bruh_id = $1 WHERE guild_id = $2",
                    channel_id as i64,
                    guild_id.0 as i64
                )
                .execute(&pool)
                .await?;
            } else {
                insert_or_update(&pool, guild_id, "bruh", channel_id as i64).await?;
            }

            msg.channel_id.say(ctx, "Channel sucessfully set!").await?;
        }

        return Ok(());
    }

    if !args.is_empty() {
        msg.channel_id
            .say(ctx, "Please execute this command without any arguments")
            .await?;
        return Ok(());
    }

    if !check.exists.unwrap() {
        msg.channel_id
            .say(ctx, "The Bruh channel isn't set! Please specify a channel!")
            .await?;
        return Ok(());
    }

    let channel_nums = get_channels(&pool, guild_id).await?;

    if channel_nums.bruh_id.is_none() {
        msg.channel_id
            .say(ctx, "The Bruh channel isn't set! Please specify a channel!")
            .await?;
        return Ok(());
    }

    let message_url = command_utils::get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);

    msg.channel_id.say(ctx, "***BRUH MOMENT***").await?;
    ChannelId(channel_nums.bruh_id.unwrap() as u64)
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0xfc5e03);
                e.title("Ladies and Gentlemen!");
                e.description(format!(
                    "A bruh moment has been declared by {}",
                    msg.author.mention()
                ));
                e.field("Source", format!("[Jump!]({})", message_url), false)
            })
        })
        .await?;

    Ok(())
}

/// Quotes yourself or a specified user
/// Usage: `quote <user mention> <content>` or `quote <content>`
#[command]
async fn quote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let mut self_quote = true;

    let test_id = args.parse::<String>().unwrap_or_default();

    let pool = ctx
        .data
        .read()
        .await
        .get::<ConnectionPool>()
        .cloned()
        .unwrap();

    let starboard_data = sqlx::query!(
        "SELECT starboard_threshold FROM guild_info WHERE guild_id = $1",
        guild_id.0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if starboard_data.starboard_threshold.is_some() {
        msg.channel_id
            .say(
                ctx,
                "You can't use the quote command because starboard is enabled in this server!",
            )
            .await?;
        return Ok(());
    }

    let check = sqlx::query!(
        "SELECT EXISTS(SELECT quote_id FROM text_channels WHERE guild_id = $1)",
        guild_id.0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if let Some(channel_id) = parse_channel(&test_id) {
        if permissions_helper::check_permission(ctx, msg, None, false).await? {
            if check.exists.unwrap() {
                sqlx::query!(
                    "UPDATE text_channels SET quote_id = $1 WHERE guild_id = $2",
                    channel_id as i64,
                    guild_id.0 as i64
                )
                .execute(&pool)
                .await?;
            } else {
                insert_or_update(&pool, guild_id, "quote", channel_id as i64).await?;
            }

            msg.channel_id.say(ctx, "Channel sucessfully set!").await?;
        }

        return Ok(());
    }

    if !check.exists.unwrap() {
        msg.channel_id
            .say(
                ctx,
                "The Quote channel isn't set! Please specify a channel!",
            )
            .await?;
        return Ok(());
    }

    if parse_username(&test_id).is_some() {
        self_quote = false;
    }

    if args.is_empty() || test_id == "" {
        msg.channel_id
            .say(
                ctx,
                "Please provide the quote (with author if you are quoting someone else)",
            )
            .await?;
        return Ok(());
    }

    let channels = get_channels(&pool, guild_id).await?;

    if channels.quote_id.is_none() {
        msg.channel_id
            .say(
                ctx,
                "The Quote channel isn't set! Please specify a channel!",
            )
            .await?;
        return Ok(());
    }

    let message_url = command_utils::get_message_url(msg.guild_id.unwrap(), msg.channel_id, msg.id);

    let avatar_option = if self_quote {
        msg.author.avatar_url()
    } else {
        msg.mentions[0].avatar_url()
    };

    let avatar_id = match avatar_option {
        Some(avatar_id) => avatar_id,
        None => {
            if self_quote {
                msg.author.default_avatar_url()
            } else {
                msg.mentions[0].default_avatar_url()
            }
        }
    };

    ChannelId(channels.quote_id.unwrap() as u64)
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0xfabe21);
                if self_quote {
                    e.author(|a| {
                        a.name(&msg.author.name);
                        a.icon_url(&avatar_id);
                        a
                    });
                    e.description(args.rest());
                } else {
                    e.author(|a| {
                        a.name(&msg.mentions[0].name);
                        a.icon_url(&avatar_id);
                        a
                    });
                    args.advance();
                    e.description(args.rest());
                }
                e.field("Source", format!("[Jump!]({})", message_url), false)
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn vibecheck(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Initiating vibe check...").await?;

    sleep(Duration::from_secs(3)).await;

    if random() {
        let success_vec = vec![
            "Continue vibing good sir/madam",
            "Have a wonderful day",
            "Your wish will come true",
            "STRAIGHT vibing! I like that",
            "Drop your favorite vibes in the chat",
        ];

        let mut rng = StdRng::from_entropy();

        let val = rng.gen_range(0..=success_vec.len() - 1);

        msg.channel_id
            .say(
                ctx,
                format!(
                    "{} has passed the vibe check. {}.",
                    msg.author.mention(),
                    success_vec[val]
                ),
            )
            .await?;
    } else {
        msg.channel_id
            .say(
                ctx,
                format!(
                    "{} has failed the vibe check. Show me your vibing license!",
                    msg.author.mention()
                ),
            )
            .await?;
    }
    Ok(())
}

async fn get_channels(
    pool: &PgPool,
    guild_id: GuildId,
) -> Result<TextChannels, Box<dyn std::error::Error + Send + Sync>> {
    let data = sqlx::query_as!(
        TextChannels,
        "SELECT nice_id, bruh_id, quote_id FROM text_channels WHERE guild_id = $1",
        guild_id.0 as i64
    )
    .fetch_one(pool)
    .await?;

    Ok(data)
}

async fn insert_or_update(
    pool: &PgPool,
    guild_id: GuildId,
    channel_type: &str,
    channel_id: i64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match channel_type {
        "nice" => {
            sqlx::query!(
                "INSERT INTO text_channels VALUES($1, $2, null, null)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET nice_id = $2",
                guild_id.0 as i64,
                channel_id
            )
            .execute(pool)
            .await?;
        }
        "bruh" => {
            sqlx::query!(
                "INSERT INTO text_channels VALUES($1, null, $2, null)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET bruh_id = $2",
                guild_id.0 as i64,
                channel_id
            )
            .execute(pool)
            .await?;
        }
        "quote" => {
            sqlx::query!(
                "INSERT INTO text_channels VALUES($1, null, null, $2)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET quote_id = $2",
                guild_id.0 as i64,
                channel_id
            )
            .execute(pool)
            .await?;
        }
        _ => {}
    }

    Ok(())
}

pub async fn sender_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "nice: Sends nice to a defined channel \n\n",
        "bruh: Sends a bruh moment to a defined channel \n\n",
        "quote <author> <text>: Quotes a user. Deactivated when starboard is enabled \n\n",
        "vibecheck: Checks your vibe. Try it out!"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Textchannel Sender Help");
                e.description("Description: Commands that send messages to specified channels");
                e.field("Commands", content, false);
                e.footer(|f| {
                    f.text("Adding a channel mention will set the sender channel (Moderator only)");
                    f
                });
                e
            })
        })
        .await;
}
