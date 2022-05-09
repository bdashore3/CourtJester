use std::collections::HashSet;

use crate::{
    helpers::command_utils,
    structures::{commands::*, errors::*},
    ConnectionPool, EmergencyCommands, PrefixMap, PubCreds,
};
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError, DispatchError, StandardFramework},
    model::{channel::Message, id::UserId, Permissions},
    prelude::Mentionable,
};

pub fn get_framework(bot_id: UserId, owners: HashSet<UserId>) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.dynamic_prefix(dynamic_prefix)
                .prefix("")
                .on_mention(Some(bot_id))
                .owners(owners)
        })
        .on_dispatch_error(dispatch_error)
        .unrecognised_command(unrecognized_command_hook)
        .before(before)
        .after(after)
        .group(&GENERAL_GROUP)
        .group(&TEXT_GROUP)
        .group(&TEXTLAST_GROUP)
        .group(&CIPHERS_GROUP)
        .group(&TEXTCHANNELSEND_GROUP)
        .group(&CONFIG_GROUP)
        .group(&SUPPORT_GROUP)
        .group(&STARBOARD_GROUP)
        .group(&VOICE_GROUP)
        .group(&MUSIC_GROUP)
        .group(&IMAGES_GROUP)
        .group(&JAPAN_GROUP)
        .group(&UTILITY_GROUP)
}

#[hook]
async fn unrecognized_command_hook(ctx: &Context, msg: &Message, command_name: &str) {
    let pool = ctx
        .data
        .read()
        .await
        .get::<ConnectionPool>()
        .cloned()
        .unwrap();

    let guild_id = msg.guild_id.unwrap().0 as i64;

    let cmd_data = sqlx::query!(
        "SELECT content FROM commands WHERE guild_id = $1 AND name = $2",
        guild_id,
        command_name
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(cmd_data) = cmd_data {
        let content = cmd_data
            .content
            .unwrap()
            .replace("{user}", &msg.author.mention().to_string());
        let _ = msg.channel_id.say(ctx, content).await;
    }
}

#[hook]
async fn before(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    if command_utils::check_mention_prefix(msg) {
        let emergency_commands = ctx
            .data
            .read()
            .await
            .get::<EmergencyCommands>()
            .cloned()
            .unwrap();

        if emergency_commands.contains(&cmd_name.to_owned()) {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    format!(
                        "{}, you are running an emergency command!",
                        msg.author.mention()
                    ),
                )
                .await;
            return true;
        } else {
            return false;
        }
    }

    true
}

// After a command is executed, goto here
#[hook]
async fn after(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        let part_1 = "Looks like the bot encountered an error! \n";
        let part_2 = "Please use the `support` command and send the output to the support server!";
        let error_string = format!("{}{}", part_1, part_2);

        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.color(0xff69b4);
                    e.title("Aw Snap!");
                    e.description(error_string);
                    e.field("Command Name", cmd_name, false);
                    e.field("Error", format!("```{} \n```", why), false);
                    e
                })
            })
            .await;
    }
}

// On a dispatch error, go to this function
#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    match error {
        DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    JesterError::PermissionError(PermissionType::UserPerm("administrator")),
                )
                .await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    JesterError::PermissionError(PermissionType::UserPerm("manage messages")),
                )
                .await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_EMOJIS_AND_STICKERS) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    JesterError::PermissionError(PermissionType::UserPerm("manage emojis")),
                )
                .await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    format!("Args required: {}. Args given: {}", min, given),
                )
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(ctx, "This is a bot dev only command!")
                .await;
        }
        _ => println!("Unhandled dispatch error: {:?}", error),
    }
}

/*
 * The heart of custom prefixes
 * If the guild has a prefix in the Dashmap, use that prefix
 * Otherwise, use the default prefix from credentials_helper
 */

#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let (prefixes, default_prefix) = {
        let data = ctx.data.read().await;
        let prefixes = data.get::<PrefixMap>().cloned().unwrap();
        let default_prefix = data
            .get::<PubCreds>()
            .unwrap()
            .get("default prefix")
            .cloned()
            .unwrap();

        (prefixes, default_prefix)
    };

    let guild_id = msg.guild_id.unwrap();

    let wrapped_prefix = prefixes.get(&guild_id);

    match wrapped_prefix {
        Some(prefix_guard) => Some(prefix_guard.value().to_owned()),
        None => Some(default_prefix),
    }
}
