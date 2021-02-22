use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::{
    commands::{
        ciphers::*, config::*, images::*, japan::*, music::*, starboard::*, textchannel_send::*,
        textmod::*,
    },
    helpers::{botinfo::*, command_utils, voice_utils::*},
};

#[command]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        if command_utils::check_mention_prefix(msg) {
            emergency_help_message(ctx, msg.channel_id).await;
        } else {
            default_help_message(ctx, msg.channel_id).await;
        }

        return Ok(());
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
        "japan" => japan_help(ctx, msg.channel_id).await,
        _ => {}
    }

    Ok(())
}

async fn emergency_help_message(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "prefix (characters): Sets the server's bot prefix \n\n",
        "resetprefix: Reset's the server's prefix back to the default one"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("CourtJester Emergency Help");
                e.description("You should only use this if you mess up your prefix!");
                e.field("Commands", content, false);
                e
            })
        })
        .await;
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
        "images \n",
        "japan \n"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("CourtJester Help");
                e.description(concat!(
                    "Help for the CourtJester Discord bot \n",
                    "Command parameters: <> is required and () is optional \n",
                    "Please use `help <subcategory>` to see that category's help"
                ));
                e.field("Subcategories", format!("```\n{}```", categories), false);
                e.footer(|f| {
                    f.text("Use the support command for any further help!");
                    f
                });
                e
            })
        })
        .await;
}

#[command]
async fn support(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("CourtJester Support");
                e.description("Need more help?");
                e.field("Support Server", "https://discord.gg/pswt7by", false);
                e.field(
                    "Github repository",
                    "https://github.com/bdashore3/courtjester",
                    false,
                );
                e.field("kingbri's twitter", "https://twitter.com/kingbri1st", false);
                e.footer(|f| {
                    f.text("Created with ❤️ by kingbri#6666");
                    f
                })
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let mut eb = CreateEmbed::default();

    let guild_count = ctx.cache.guilds().await.len();
    let channel_count = ctx.cache.guild_channel_count().await;
    let user_count = ctx.cache.user_count().await;

    let guild_name = if guild_count < 2 { "guild" } else { "guilds" };

    get_system_info(ctx).await;

    let last_commit = get_last_commit(ctx).await?;
    let sys_info = get_system_info(ctx).await;

    let mut story_string = String::new();
    story_string.push_str(&format!(
        "Currently running on commit [{}]({}) \n",
        &last_commit.sha[..7],
        last_commit.html_url
    ));
    story_string.push_str(&format!("Inside `{}` {} \n", guild_count, guild_name));
    story_string.push_str(&format!("With `{}` total channels \n", channel_count));
    story_string.push_str(&format!("Along with `{}` faithful users \n", user_count));
    story_string.push_str(&format!(
        "Consuming `{:.3} MB` of memory \n",
        sys_info.memory
    ));
    story_string.push_str(&format!("With a latency of `{}`", sys_info.shard_latency));

    eb.title("CourtJester is");
    eb.color(0xfda50f);
    eb.description(story_string);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.0 = eb.0;
                e
            })
        })
        .await?;

    Ok(())
}
