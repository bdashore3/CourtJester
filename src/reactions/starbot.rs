use crate::{
    structures::CommandResult,
    structures::Context,
    helpers::command_utils::*
};
use twilight::{
    model::{id::{MessageId, ChannelId}, user::User},
    model::channel::Reaction,
    builders::embed::EmbedBuilder
};

struct StarbotConfig {
    starbot_threshold: Option<i32>,
    quote_id: Option<i64>
}

pub async fn quote_reaction(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult<()> {
    let reaction_message = ctx.http.message(reaction.channel_id, reaction.message_id).await?.unwrap();
    let reactions = reaction_message.reactions;
    let stars = match reactions.into_iter()
        .find(|x| get_reaction_emoji(&x.emoji) == "\u{2b50}") {
            Some(reaction) => reaction.count,
            None => 0
        };

    let config_data = sqlx::query_as!(StarbotConfig, "SELECT guild_info.starbot_threshold, text_channels.quote_id
                                    FROM guild_info
                                    INNER JOIN text_channels ON guild_info.guild_id=text_channels.guild_id")
        .fetch_one(ctx.pool.as_ref()).await?;
    
    if config_data.starbot_threshold.is_none() || config_data.quote_id.is_none() {
        send_message(&ctx.http, reaction.channel_id, 
            "Starbot isn't enabled for this guild! Please set a threshold and channel to send messages in!").await?;
        
        return Ok(())
    }

    if stars == config_data.starbot_threshold.unwrap() as u64 && !remove {
        let send_channel = ChannelId::from(config_data.quote_id.unwrap() as u64);
        let first_message = format!("\u{2b50} {} ID: {}", stars, reaction.message_id);

        let user = reaction_message.author;

        let eb = get_starbot_embed(reaction, user, reaction_message.content);

        let sent_message = ctx.http.create_message(send_channel).content(first_message)?.embed(eb.build())?.await?;
        sqlx::query!("INSERT INTO starbot VALUES($1, $2, $3) ON CONFLICT DO NOTHING", 
                reaction.guild_id.unwrap().0 as i64, reaction_message.id.0 as i64, sent_message.id.0 as i64)
            .execute(ctx.pool.as_ref()).await?;
    }
    else if (stars as i32) < config_data.starbot_threshold.unwrap() && remove {
        let message_data = sqlx::query!("SELECT sent_message_id FROM starbot WHERE guild_id = $1 AND reaction_message_id = $2", 
            reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
        .fetch_optional(ctx.pool.as_ref()).await?;

        if let Some(data) = message_data {
            let quote_channel = ChannelId::from(config_data.quote_id.unwrap() as u64);
            let sent_messaage = MessageId::from(data.sent_message_id as u64);
            ctx.http.delete_message(quote_channel, sent_messaage).await?;

            sqlx::query!("DELETE FROM starbot WHERE guild_id = $1 and reaction_message_id = $2",
                reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
            .execute(ctx.pool.as_ref()).await?;
        }
    }
    else if stars > config_data.starbot_threshold.unwrap() as u64 || remove {
        let message_data = sqlx::query!("SELECT sent_message_id FROM starbot WHERE guild_id = $1 AND reaction_message_id = $2", 
                reaction.guild_id.unwrap().0 as i64, reaction.message_id.0 as i64)
        .fetch_optional(ctx.pool.as_ref()).await?;

        if let Some(data) = message_data {
            let first_message = format!("\u{2b50} {} ID: {}", stars, reaction.message_id);
            let eb = get_starbot_embed(reaction, reaction_message.author, reaction_message.content);

            ctx.http.update_message(
                    ChannelId::from(config_data.quote_id.unwrap() as u64), MessageId::from(data.sent_message_id as u64))
                .content(first_message)?.embed(eb.build())?.await?;
        }
    }

    Ok(())
}

fn get_starbot_embed(reaction: &Reaction, user: User, content: String) -> EmbedBuilder {
    let user_avatar = match user.avatar.as_ref() {
        Some(avatar_id) => {
            get_avatar_url(user.id, avatar_id)
        }
        None => {
            get_default_avatar_url(&user.discriminator)
        }
    };
    
    let mut eb = EmbedBuilder::new();
    eb = eb.author().name(&user.name).icon_url(user_avatar).commit();
    eb = eb.color(0xfabe21);
    eb = eb.description(&content);

    let message_url = get_message_url(reaction.guild_id.unwrap(), reaction.channel_id, reaction.message_id);
    eb = eb.add_field("Source", format!("[Jump!]({})", message_url)).commit();

    eb
}