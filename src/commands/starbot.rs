use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*,
    helpers::string_renderer,
    helpers::permissions_helper,
};
use twilight::{
    model::{
        guild::Permissions, 
        channel::{ReactionType, Message}, 
        gateway::payload::{MessageCreate, ReactionAdd}, id::ChannelId
    }, builders::embed::EmbedBuilder
};

pub async fn starbot(ctx: &Context, msg: &Message) -> CommandResult<()> {
    if !permissions_helper::check_permission(ctx, msg, Permissions::ADMINISTRATOR).await {
        return Ok(())
    }
    let subcommand = string_renderer::get_message_word(&msg.content, 1);
    match subcommand {
        "wizard" => {
            starbot_wizard(ctx, msg).await?;
        }
        "threshold" => {
            let new_threshold = string_renderer::get_message_word(&msg.content, 2).parse::<i32>();
            sqlx::query!("UPDATE guild_info SET starbot_threshold = $1 WHERE guild_id = $2", new_threshold.unwrap(), msg.guild_id.unwrap().0 as i64)
                .execute(ctx.pool.as_ref()).await?;
            
            send_message(&ctx.http, msg.channel_id, "New threshold sucessfully set!").await?;
        },
        "channel" => {
            let new_channel_string = string_renderer::get_message_word(&msg.content, 2);
            let new_channel = get_raw_id(new_channel_string, "channel");

            sqlx::query!("INSERT INTO text_channels VALUES($1, null, null, $2)
                        ON CONFLICT (guild_id)
                        DO UPDATE SET quote_id = $2",
                        msg.guild_id.unwrap().0 as i64, new_channel.unwrap() as i64)
                        .execute(ctx.pool.as_ref()).await?;
            
            send_message(&ctx.http, msg.channel_id, "New starbot channel sucessfully set!").await?;
        },
        "deactivate" => {
            deactivate_starbot(ctx, msg).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn deactivate_starbot(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let sent_message = ctx.http.create_message(msg.channel_id).content("Removing the starbot re-enables quoting! You want to do this?")?.await?;

    ctx.http.create_reaction(msg.channel_id, sent_message.id, ReactionType::Unicode { name: String::from("✅") }).await?;
    ctx.http.create_reaction(msg.channel_id, sent_message.id, ReactionType::Unicode { name: String::from("❌") }).await?;

    let author_id = msg.author.id;

    let reaction = ctx.standby.wait_for_reaction(sent_message.id, move |event: &ReactionAdd| {
        event.user_id == author_id
    }).await?;

    let reaction_emoji = get_reaction_emoji(&reaction.0.emoji);

    if reaction_emoji == "✅" {
        sqlx::query!("UPDATE guild_info SET starbot_threshold = null WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
            .execute(ctx.pool.as_ref()).await?;

        sqlx::query!("UPDATE text_channels SET quote_id = null WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
            .execute(ctx.pool.as_ref()).await?;

        send_message(&ctx.http, msg.channel_id, "Starbot deactivated.").await?;
    }
    else if reaction_emoji == "❌" {
        send_message(&ctx.http, msg.channel_id, "Aborting...").await?;
    }

    Ok(())
}

async fn starbot_wizard(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let mut intro_string = String::new();
    intro_string.push_str("Welcome to starbot configuration! \n");
    intro_string.push_str("Reacting with ✅ will disable quoting on your guild!");

    let sent_message = ctx.http.create_message(msg.channel_id).content(intro_string)?.await?;

    ctx.http.create_reaction(msg.channel_id, sent_message.id, ReactionType::Unicode { name: String::from("✅") }).await?;
    ctx.http.create_reaction(msg.channel_id, sent_message.id, ReactionType::Unicode { name: String::from("❌") }).await?;

    let author_id = msg.author.id;

    let reaction = ctx.standby.wait_for_reaction(sent_message.id, move |event: &ReactionAdd| {
        event.user_id == author_id
    }).await?;

    let reaction_emoji = get_reaction_emoji(&reaction.0.emoji);

    if reaction_emoji == "✅" {
        starbot_wizard_threshold(ctx, msg).await?;
    }
    else if reaction_emoji == "❌" {
        send_message(&ctx.http, msg.channel_id, "Aborting...").await?;
    }

    Ok(())
}

async fn starbot_wizard_threshold(ctx: &Context, msg: &Message) -> CommandResult<()> {
    send_message(&ctx.http, msg.channel_id, "Sounds good! Please enter a number greater than 0 for the starbot threshold!").await?;
    let author_id = msg.author.id;

    let mut _is_channel = false;
    loop {
        let threshold_message = ctx.standby.wait_for_message(msg.channel_id, move |event: &MessageCreate| {
            event.0.author.id == author_id
        }).await?;

        match threshold_message.content.parse::<u32>() {
            Ok(threshold) => {
                if threshold > 0 {
                    sqlx::query!("UPDATE guild_info SET starbot_threshold = $1 WHERE guild_id = $2", threshold as i32, msg.guild_id.unwrap().0 as i64)
                        .execute(ctx.pool.as_ref()).await?;
                        
                    _is_channel = true;
                    break;
                } else {
                    send_message(&ctx.http, msg.channel_id, "Please enter an integer greater than 0!").await?;
                }
            },
            Err(_e) => {
                send_message(&ctx.http, msg.channel_id, "Please enter an integer greater than 0!").await?;
            }
        }
    }

    if _is_channel {
        starbot_wizard_channel(ctx, msg).await?;
    }

    Ok(())
}

async fn starbot_wizard_channel(ctx: &Context, msg: &Message) -> CommandResult<()> {
    let author_id = msg.author.id;

    let channel_check = sqlx::query!("SELECT quote_id FROM text_channels WHERE guild_id = $1", msg.guild_id.unwrap().0 as i64)
        .fetch_one(ctx.pool.as_ref()).await?;

    if channel_check.quote_id.is_some() {
        send_message(&ctx.http, msg.channel_id, 
    "You already have a channel set up for quotes! \nIf you want to change it, run `starbot channel`").await?;
        
        send_message(&ctx.http, msg.channel_id, "Enjoy your new starbot!").await?;
    } else {
        send_message(&ctx.http, msg.channel_id, "Now please mention the channel you want messages sent to!").await?;

        loop {
            let channel_message = ctx.standby.wait_for_message(msg.channel_id, move |event: &MessageCreate| {
                event.0.author.id == author_id
            }).await?;

            let given_id = string_renderer::get_message_word(&channel_message.content, 0);

            match get_raw_id(given_id, "channel") {
                Ok(channel_id) => {
                    if channel_id == 0 {
                        send_message(&ctx.http, msg.channel_id, "Please mention a channel in this guild!").await?
                    } else {
                        sqlx::query!("INSERT INTO text_channels VALUES($1, null, null, $2)
                                    ON CONFLICT (guild_id)
                                    DO UPDATE SET quote_id = $2",
                                msg.guild_id.unwrap().0 as i64, channel_id as i64)
                            .execute(ctx.pool.as_ref()).await?;

                        send_message(&ctx.http, msg.channel_id, "Enjoy your new starbot!").await?;
                        return Ok(());
                    }
                },
                Err(_e) => send_message(&ctx.http, msg.channel_id, "Please mention a channel in this guild!").await?
            }
        }
    }

    Ok(())
}

pub async fn starbot_help(ctx: &Context, channel_id: ChannelId) {
    let mut content = String::new();
    content.push_str("wizard: Easy way to setup the starboard \n\n");
    content.push_str("threshold: Sets the threshold for a message to appear \n\n");
    content.push_str("channel: Sets the channel where starboard embeds are sent \n\n");
    content.push_str("Deactivate: Deactivates the starboard and re-enables quoting");
    
    let mut eb = EmbedBuilder::new();

    eb = eb.title("Starboard Help");
    eb = eb.description("Description: admin commands for starboarding in a discord server");
    eb = eb.add_field("Commands", content).commit();
    eb = eb.footer("Enabling the starboard will disable the quote command!").commit();

    let _ = send_embed(&ctx.http, channel_id, eb.build()).await;
}