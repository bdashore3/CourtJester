use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::{builder::CreateEmbed, framework::standard::{
    CommandResult,
    macros::command,
    Args
}};
use crate::commands::{
    config::*,
    starboard::*,
    textchannel_send::*,
    ciphers::*,
    textmod::*,
    music::*,
    images::*
};
use crate::helpers::botinfo::*;
use crate::helpers::voice_utils::*;

#[command]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 {
        default_help_message(ctx, msg.channel_id).await;
        return Ok(())
    }

    let subcommand = args.single::<String>()?;
    
    match subcommand.as_str() {
        "prefix" => prefix_help(ctx, msg.channel_id).await,
        "command" => command_help(ctx, msg.channel_id).await,
        "starboard" => starboard_help(ctx, msg.channel_id).await,
        "senders" => sender_help(ctx, msg.channel_id).await,
        "ciphers" => cipher_help(ctx, msg.channel_id).await,
        "text" => textmod_help(ctx, msg.channel_id).await,
        "voice" => voice_help(ctx, msg.channel_id).await,
        "music" => music_help(ctx, msg.channel_id).await,
        "images" => image_help(ctx, msg.channel_id).await,
        _ => {}
    }

    Ok(())
}

async fn default_help_message(ctx: &Context, channel_id: ChannelId) {
    let categories = concat!(
        "prefix \n",
        "command \n",
        "starboard \n",
        "senders \n",
        "ciphers \n",
        "text \n",
        "voice \n",
        "music \n",
        "images \n");

    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("CourtJester Help");
            e.description("Help for the CourtJester Discord bot");
            e.field("Subcategories", format!("```\n{}```", categories), false);
            e.footer(|f| {
                f.text("Use the support command for any further help!");
                f
            });
            e
        })
    }).await;
}

#[command]
async fn support(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("CourtJester Support");
            e.description("Need more help?");
            e.field("Support Server", "https://discord.gg/pswt7by", false);
            e.field("kingbri's twitter", "https://twitter.com/kingbri1st", false);
            e.footer(|f| {
                f.text("Created with ❤️ by kingbri#6666");
                f
            })
        })
    }).await?;

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let mut eb = CreateEmbed::default();

    let guild_count = ctx.cache.guilds().await.len();
    let channel_count = ctx.cache.guild_channel_count().await;
    let user_count = ctx.cache.user_count().await;

    let guild_name = if guild_count < 2 {
                "guild"
            } else {
                "guilds"
            };

    
    get_system_info(ctx).await;
    
    let last_commit = get_last_commit(ctx).await?;
    let sys_info = get_system_info(ctx).await;

    let mut story_string = String::new();
    story_string.push_str(&format!("Currently running on commit [{}]({}) \n", &last_commit.sha[..7], last_commit.html_url));
    story_string.push_str(&format!("Inside `{}` {} \n", guild_count, guild_name));
    story_string.push_str(&format!("With `{}` total channels \n", channel_count));
    story_string.push_str(&format!("Along with `{}` faithful users \n", user_count));
    story_string.push_str(&format!("Consuming `{:.3} MB` of memory \n", sys_info.memory));
    story_string.push_str(&format!("With a latency of `{}`", sys_info.shard_latency));

    eb.title("CourtJester is");
    eb.color(0xfda50f);
    eb.description(story_string);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.0 = eb.0;
            e
        })
    }).await?;

    Ok(())
}