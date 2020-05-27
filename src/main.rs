use std::{
    env,
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
        standard::macros::hook,
        standard::CommandError
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready, channel::Message},
    prelude::*,
};

use commands::{
    other::*,
    textmod::*,
    ciphers::*,
    textchannel_send::*,
    custom::*
};

use sqlx::PgPool;

use helpers::database_helper::*;

mod commands;
mod helpers;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(mock, inv, upp, low, space, biggspace)]
struct Text;

#[group]
#[commands(mockl, invl, uppl, lowl, spacel, biggspacel)]
struct TextLast;

#[group]
#[commands(b64encode, b64decode)]
struct Ciphers;

#[group]
#[commands(nice, bruh, quote)]
struct TextChannelSend;

#[group]
#[commands(command)]
struct CustomCommands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials_helper::read_creds(args[1].to_string()).unwrap();
    let token = &creds.bot_token;

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    #[hook]
    async fn unrecognized_command_hook(ctx: &Context, msg: &Message, command_name: &str) {
        let data = ctx.data.read().await;

        let pool = data.get::<ConnectionPool>().unwrap();
    
        let guild_id = msg.guild_id.unwrap();

        let command = sqlx::query!("SELECT * FROM commands WHERE guild_id = $1 AND name = $2", guild_id.0 as i64, command_name)
            .fetch_optional(pool)
            .await
            .unwrap();
        
        if let Some(x) = command {
            let _ = msg.channel_id.say(ctx, format!("{}", x.content.unwrap())).await;
        }
    }

    #[hook]
    async fn after(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
        if let Err(why) = error {
            println!("Error in {}: {:?}", cmd_name, why);
        }
    }

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix(&creds.default_prefix))
            .unrecognised_command(unrecognized_command_hook)
            .after(after)
        
        .group(&GENERAL_GROUP)
        .group(&TEXT_GROUP)
        .group(&TEXTLAST_GROUP)
        .group(&CIPHERS_GROUP)
        .group(&TEXTCHANNELSEND_GROUP)
        .group(&CUSTOMCOMMANDS_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        let pool = obtain_pool().await?;
        data.insert::<ConnectionPool>(pool.clone());

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let _owners = match client.cache_and_http.http.get_current_application_info().await {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
