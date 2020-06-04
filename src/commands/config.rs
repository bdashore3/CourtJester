use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args
};

use sqlx;
use crate::{
    ConnectionPool,
    PrefixMap
};

#[command]
#[min_args(1)]
#[only_in("guilds")]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix;
    let data = ctx.data.read().await;

    let pool = data.get::<ConnectionPool>().unwrap();
    let prefixes = data.get::<PrefixMap>().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;

    if args.is_empty() {
        prefix = match prefixes.get(&guild_id) {
            Some(prefix) => prefix.to_string(),
            None => "~".to_string()
        };

        msg.channel_id.say(ctx, format!("Your server's prefix is: `{}`", prefix)).await?;
        return Ok(())
    }
    
    prefix = args.single::<String>().unwrap();
    prefixes.insert(guild_id, prefix.to_string());
    sqlx::query!("INSERT INTO guild_info VALUES($1, $2)", guild_id, prefix.to_string())
        .execute(pool)
        .await?;

    Ok(())
}