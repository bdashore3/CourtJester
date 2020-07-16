use serenity::{
    client::Context, 
    framework::standard::{CommandResult, macros::command, Args, Delimiter}, 
    model::{id::ChannelId, channel::{ReactionType, Message}},
    utils::parse_channel
};
use sqlx::PgPool;
use crate::structures::ConnectionPool;
use crate::helpers::command_utils::*;

#[command]
#[required_permissions(Administrator)]
#[sub_commands("deactivate", "wizard", "threshold", "channel")]
async fn starboard(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn threshold(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let new_threshold = match args.single::<u32>() {
        Ok(threshold) => threshold,
        Err(_e) => {
            msg.channel_id.say(ctx, "Please enter a number greater than 0!").await?;
            return Ok(())
        }
    };

    sqlx::query!("UPDATE guild_info SET starbot_threshold = $1 WHERE guild_id = $2", new_threshold as i32, msg.guild_id.unwrap().0 as i64)
        .execute(pool).await?;

    msg.channel_id.say(ctx, "New threshold sucessfully set!").await?;

    Ok(())
}

#[command]
async fn channel (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let test_id = args.single::<String>().unwrap();
    let new_channel = match parse_channel(&test_id) {
        Some(channel_id) => channel_id,
        None => {
            msg.channel_id.say(ctx, "Please mention a channel!").await?;
            return Ok(())
        }
    };

    sqlx::query!("INSERT INTO text_channels VALUES($1, null, null, $2)
                ON CONFLICT (guild_id)
                DO UPDATE SET quote_id = $2",
            msg.guild_id.unwrap().0 as i64, new_channel as i64)
        .execute(pool).await?;

    msg.channel_id.say(ctx, "New starboard channel sucessfully set!").await?;

    Ok(())
}

#[command]
async fn deactivate(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let sent_message = msg.channel_id.say(ctx, "Removing the starbot re-enables quoting! You want to do this?").await?;
    sent_message.react(ctx, ReactionType::Unicode(String::from("✅"))).await?;
    sent_message.react(ctx, ReactionType::Unicode(String::from("❌"))).await?;

    let reaction_action = sent_message.await_reaction(ctx).await.unwrap();
    let reaction = reaction_action.as_inner_ref();
    let reaction_emoji = get_reaction_emoji(&reaction.emoji);

    if reaction_emoji == "✅" {
        sqlx::query!("UPDATE guild_info SET starbot_threshold = null WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
            .execute(pool).await?;

        sqlx::query!("UPDATE text_channels SET quote_id = null WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
            .execute(pool).await?;
        
        msg.channel_id.say(ctx, "The starboard has been deactivated").await?;
    }
    else if reaction_emoji == "❌" {
        msg.channel_id.say(ctx, "Aborting...").await?;
    }

    Ok(())
}

#[command]
async fn wizard(ctx: &Context, msg: &Message) -> CommandResult {
    let mut intro_string = String::new();
    intro_string.push_str("Welcome to starboard configuration \n");
    intro_string.push_str("Reacting with ✅ will disable quoting on your guild!");

    let sent_message = msg.channel_id.say(ctx, intro_string).await?;
    sent_message.react(ctx, ReactionType::Unicode(String::from("✅"))).await?;
    sent_message.react(ctx, ReactionType::Unicode(String::from("❌"))).await?;

    let reaction_action = sent_message.await_reaction(ctx).await.unwrap();
    let reaction = reaction_action.as_inner_ref();

    let reaction_emoji = get_reaction_emoji(&reaction.emoji);

    if reaction_emoji == "✅" {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();

        starboard_wizard_threshold(ctx, msg, pool).await?
    }
    else if reaction_emoji == "❌" {
        msg.channel_id.say(ctx, "Aborting...").await?;
    }

    Ok(())
}

async fn starboard_wizard_threshold(ctx: &Context, msg: &Message, pool: &PgPool) -> CommandResult {
    msg.channel_id.say(ctx, "Sounds good! Please enter a number greater than 0 for the starbot threshold!").await?;
    
    let mut _is_channel = false;
    loop {
        let threshold_message = msg.author.await_reply(ctx).await.unwrap();

        match threshold_message.content.parse::<u32>() {
            Ok(threshold) => {
                if threshold > 0 {
                    sqlx::query!("UPDATE guild_info SET starbot_threshold = $1 WHERE guild_id = $2",
                        threshold as i32, msg.guild_id.unwrap().0 as i64)
                    .execute(pool).await?;

                    _is_channel = true;
                    break;
                }
            }
            Err(_e) => {
                msg.channel_id.say(ctx, "Please enter an integer greater than 0!").await?;
            }
        }
    }

    if _is_channel {
        starboard_wizard_channel(ctx, msg, pool).await?;
    }

    Ok(())
}

async fn starboard_wizard_channel(ctx: &Context, msg: &Message, pool: &PgPool) -> CommandResult {
    let channel_check = sqlx::query!("SELECT quote_id FROM text_channels WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
        .fetch_one(pool).await?;
    
    if channel_check.quote_id.is_some() {
        let mut send_string = String::new();
        send_string.push_str("You already have a channel set up for quotes! \nIf you want to change it, run `starbot channel` \n");
        send_string.push_str("Enjoy your new starboard!");
        msg.channel_id.say(ctx, send_string).await?;
    } else {
        msg.channel_id.say(ctx, "Now please mention the channel you want messages sent to!").await?;

        loop {
            let channel_message = msg.author.await_reply(ctx).await.unwrap();
            let args = Args::new(&channel_message.content, &[Delimiter::Single(' ')]);
            let given_id = args.parse::<String>().unwrap();

            match parse_channel(given_id) {
                Some(channel_id) => {
                    sqlx::query!("INSERT INTO text_channels VALUES($1, null, null, $2)
                                ON CONFLICT (guild_id)
                                DO UPDATE SET quote_id = $2",
                            msg.guild_id.unwrap().0 as i64, channel_id as i64)
                        .execute(pool).await?;

                    msg.channel_id.say(ctx, "Enjoy your new starboard!").await?;
                    break;
                },
                None => {
                    msg.channel_id.say(ctx, "Please mention a channel in this guild!").await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn starboard_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("wizard: Easy way to setup the starboard \n\n");
    content.push_str("threshold: Sets the threshold for a message to appear \n\n");
    content.push_str("channel: Sets the channel where starboard embeds are sent \n\n");
    content.push_str("Deactivate: Deactivates the starboard and re-enables quoting");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Starboard Help");
            e.description("Description: admin commands for starboarding in a discord server");
            e.field("Commands", content, false);
            e.footer(|f| {
                f.text("Enabling the starboard will disable the quote command!");
                f
            });
            e
        })
    }).await;
}