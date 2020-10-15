use serenity::{
    model::{
        id::ChannelId, 
        channel::{ReactionType, Reaction, Attachment}, 
        prelude::User
    },
    client::Context, 
    framework::standard::CommandResult, 
    builder::CreateEmbed, prelude::Mentionable
};

use crate::{
    helpers::command_utils,
    structures::cmd_data::ConnectionPool
};

struct StarbotConfig {
    starbot_threshold: Option<i32>,
    quote_id: Option<i64>
}

pub async fn quote_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    let pool = ctx.data.read().await
        .get::<ConnectionPool>().cloned().unwrap();
    let reaction_message = reaction.message(ctx).await?;

    let reaction_channel = reaction.channel(ctx).await?;

    let reactions = reaction_message.reactions;
    let stars = match reactions.into_iter()
        .find(|_x| reaction.emoji == ReactionType::Unicode("\u{2b50}".to_string())) {
            Some(reaction) => reaction.count,
            None => 0
        };

    let config_data = sqlx::query_as!(StarbotConfig, "SELECT guild_info.starbot_threshold, text_channels.quote_id
                                    FROM guild_info
                                    INNER JOIN text_channels ON guild_info.guild_id=text_channels.guild_id
                                    WHERE guild_info.guild_id = $1", reaction.guild_id.unwrap().0 as i64)
        .fetch_one(&pool).await?;
    
    if config_data.starbot_threshold.is_none() || config_data.quote_id.is_none() {        
        return Ok(())
    }
    
    let star_channel_id = ChannelId(config_data.quote_id.unwrap() as u64);
    let star_channel = match ctx.cache.channel(star_channel_id).await {
        Some(star_channel) => star_channel,
        None => {
            star_channel_id.say(ctx,"The star channel can't be found! Please set a new one!").await?;
            return Ok(())
        }
    };

    if star_channel.is_nsfw() {
        reaction_channel.id().say(ctx, "You can't star an NSFW message in a non-nsfw starboard!").await?;
        return Ok(())
    }

    if stars == config_data.starbot_threshold.unwrap() as u64 && !remove {
        let first_message = format!("\u{2b50} {} {} ID: {}", stars, reaction_channel.mention(), reaction.message_id);
        let starboard_embed = get_starboard_embed(reaction, &reaction_message.author, reaction_message.content, reaction_message.attachments);
        let sent_message = star_channel_id.send_message(ctx, |m| {
            m.content(first_message);
            m.embed(|e| {
               e.0 = starboard_embed.0;
               e
            })
        }).await?;

        sqlx::query!("INSERT INTO starbot VALUES($1, $2, $3) ON CONFLICT DO NOTHING",
                reaction.guild_id.unwrap().0 as i64, reaction_message.id.0 as i64, sent_message.id.0 as i64)
            .execute(&pool).await?;
    }
    else if (stars as i32) < config_data.starbot_threshold.unwrap() && remove {
        let message_data = sqlx::query!("SELECT sent_message_id FROM starbot WHERE guild_id = $1 AND reaction_message_id = $2", 
                reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
            .fetch_optional(&pool).await?;
        
        if let Some(data) = message_data {
            ctx.http.delete_message(star_channel_id.0 as u64, data.sent_message_id as u64).await?;

            sqlx::query!("DELETE FROM starbot WHERE guild_id = $1 and reaction_message_id = $2",
                    reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
                .execute(&pool).await?;
        }
    }
    else if stars > config_data.starbot_threshold.unwrap() as u64 || remove {
        let message_data = sqlx::query!("SELECT sent_message_id FROM starbot WHERE guild_id = $1 AND reaction_message_id = $2", 
                reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
            .fetch_optional(&pool).await?;

        if let Some(data) = message_data {
            let first_message = format!("\u{2b50} {} ID: {}", stars, reaction.message_id);
            let eb = get_starboard_embed(reaction, &reaction_message.author, reaction_message.content, reaction_message.attachments);

            let mut sent_message = ctx.http.get_message(star_channel_id.0 as u64, data.sent_message_id as u64).await?;
            sent_message.edit(ctx, |m| {
                m.content(first_message);
                m.embed(|e| {
                    e.0 = eb.0;
                    e
                })
            }).await?
        }
    }

    Ok(())
}

fn get_starboard_embed(reaction: &Reaction, user: &User, content: String, attachments: Vec<Attachment>) -> CreateEmbed {
    let mut eb = CreateEmbed::default();

    eb.color(0xfabe21);
    eb.author(|a| {
        a.name(&user.name);
        a.icon_url(match user.avatar_url() {
            Some(avatar_url) => avatar_url,
            None =>  user.default_avatar_url()
        });
        a
    });
    eb.description(content);

    if attachments.len() > 0 {
        if [".png", ".jpeg", ".jpg", ".webp", ".gif"].iter().any(|ext| attachments[0].url.ends_with(ext)) {
            eb.image(&attachments[0].url);
        }
    }

    let message_url = command_utils::get_message_url(reaction.guild_id.unwrap(), reaction.channel_id, reaction.message_id);
    eb.field("Source", format!("[Jump!]({})", message_url), false);

    eb
}
